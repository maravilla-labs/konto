use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::{FixedAssetStatus, FiscalYearStatus};
use konto_db::entities::{depreciation_entry, fixed_asset};
use konto_db::repository::depreciation_entry_repo::DepreciationEntryRepo;
use konto_db::repository::fixed_asset_repo::FixedAssetRepo;
use konto_db::repository::fiscal_year_repo::FiscalYearRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::journal_service::{JournalLineInput, JournalService};

pub struct DepreciationService;

impl DepreciationService {
    /// Calculate annual depreciation amount for a given asset.
    pub fn calculate(
        asset: &fixed_asset::Model,
        current_book_value: Decimal,
    ) -> Decimal {
        match asset.depreciation_method.as_str() {
            "declining_balance" => {
                let rate = asset.declining_rate.unwrap_or(Decimal::ZERO);
                let depreciation = current_book_value * rate;
                let min_book_value = asset.residual_value;
                if current_book_value - depreciation < min_book_value {
                    (current_book_value - min_book_value).max(Decimal::ZERO)
                } else {
                    depreciation
                }
            }
            _ => {
                // straight_line
                let depreciable = asset.acquisition_cost - asset.residual_value;
                if asset.useful_life_years > 0 {
                    depreciable / Decimal::from(asset.useful_life_years)
                } else {
                    Decimal::ZERO
                }
            }
        }
    }

    /// Get the depreciation schedule (all entries) for a fixed asset.
    pub async fn get_schedule(
        db: &DatabaseConnection,
        asset_id: &str,
    ) -> Result<Vec<depreciation_entry::Model>, AppError> {
        DepreciationEntryRepo::find_by_asset_id(db, asset_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Run depreciation for all active assets in a given fiscal year.
    /// Creates journal entries and depreciation entry records.
    pub async fn run_depreciation(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        user_id: &str,
    ) -> Result<Vec<depreciation_entry::Model>, AppError> {
        // Get fiscal year
        let fiscal_year = FiscalYearRepo::find_by_id(db, fiscal_year_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fiscal year not found".into()))?;

        if fiscal_year.status == FiscalYearStatus::Closed.as_str() {
            return Err(AppError::Validation("Cannot run depreciation on a closed fiscal year".into()));
        }

        // Get all active assets
        let assets = FixedAssetRepo::find_active(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut results = Vec::new();

        for asset in &assets {
            // Check if depreciation already exists for this asset + fiscal year
            let existing = DepreciationEntryRepo::find_by_asset_id(db, &asset.id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            let already_run = existing.iter().any(|e| e.fiscal_year_id == fiscal_year_id);
            if already_run {
                continue;
            }

            // Calculate current book value from previous entries
            let accumulated: Decimal = existing.iter().map(|e| e.amount).sum();
            let current_book_value = asset.acquisition_cost - accumulated;

            if current_book_value <= asset.residual_value {
                continue;
            }

            let depreciation_amount = Self::calculate(asset, current_book_value);
            if depreciation_amount <= Decimal::ZERO {
                continue;
            }

            let new_accumulated = accumulated + depreciation_amount;
            let new_book_value = asset.acquisition_cost - new_accumulated;

            // Create journal entry: Debit depreciation_account / Credit asset_account
            let lines = vec![
                JournalLineInput {
                    account_id: asset.depreciation_account_id.clone(),
                    debit_amount: depreciation_amount,
                    credit_amount: Decimal::ZERO,
                    description: Some(format!("Depreciation: {}", asset.name)),
                    vat_rate_id: None,
                },
                JournalLineInput {
                    account_id: asset.account_id.clone(),
                    debit_amount: Decimal::ZERO,
                    credit_amount: depreciation_amount,
                    description: Some(format!("Depreciation: {}", asset.name)),
                    vat_rate_id: None,
                },
            ];

            let description = format!("Depreciation {} — FY {}", asset.name, fiscal_year.name);
            let (journal_entry, _) = JournalService::create(
                db,
                fiscal_year.end_date,
                &description,
                Some(format!("DEP-{}", asset.id[..8].to_uppercase())),
                None,
                None,
                Some(user_id.to_string()),
                lines,
            )
            .await?;

            // Auto-post the journal entry
            JournalService::post_entry(db, &journal_entry.id).await?;

            // Create depreciation entry record
            let entry_model = depreciation_entry::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                fixed_asset_id: Set(asset.id.clone()),
                fiscal_year_id: Set(fiscal_year_id.to_string()),
                journal_entry_id: Set(journal_entry.id.clone()),
                amount: Set(depreciation_amount),
                accumulated: Set(new_accumulated),
                book_value: Set(new_book_value),
                period_date: Set(fiscal_year.end_date),
                created_at: Set(Utc::now().naive_utc()),
            };

            let entry = DepreciationEntryRepo::create(db, entry_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            results.push(entry);

            // Update asset status if fully depreciated
            if new_book_value <= asset.residual_value {
                let mut asset_model: fixed_asset::ActiveModel = asset.clone().into();
                asset_model.status = Set(FixedAssetStatus::FullyDepreciated.to_string());
                asset_model.updated_at = Set(Utc::now().naive_utc());
                FixedAssetRepo::update(db, asset_model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        Ok(results)
    }
}
