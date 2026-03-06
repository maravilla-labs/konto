use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::FiscalYearStatus;
use konto_common::enums::JournalStatus;
use konto_db::entities::{journal_entry, journal_line};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::fiscal_year_repo::FiscalYearRepo;
use konto_db::repository::journal_repo::JournalRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct JournalService;

#[derive(Debug, serde::Deserialize)]
pub struct JournalLineInput {
    pub account_id: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub vat_rate_id: Option<String>,
}

impl JournalService {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<(Vec<journal_entry::Model>, u64), AppError> {
        JournalRepo::find_entries_paginated(
            db, page, per_page, search, date_from, date_to, sort_by, sort_order,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<(journal_entry::Model, Vec<journal_line::Model>), AppError> {
        let entry = JournalRepo::find_entry_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Journal entry not found".to_string()))?;

        let lines = JournalRepo::find_lines_by_entry(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((entry, lines))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        date: chrono::NaiveDate,
        description: &str,
        reference: Option<String>,
        currency_id: Option<String>,
        exchange_rate: Option<Decimal>,
        created_by: Option<String>,
        lines: Vec<JournalLineInput>,
    ) -> Result<(journal_entry::Model, Vec<journal_line::Model>), AppError> {
        // Validate debit = credit
        let total_debit: Decimal = lines.iter().map(|l| l.debit_amount).sum();
        let total_credit: Decimal = lines.iter().map(|l| l.credit_amount).sum();

        if total_debit != total_credit {
            return Err(AppError::Validation(format!(
                "Debits ({total_debit}) must equal credits ({total_credit})"
            )));
        }

        if lines.is_empty() {
            return Err(AppError::Validation("At least one journal line is required".to_string()));
        }

        for (i, line) in lines.iter().enumerate() {
            if line.debit_amount == Decimal::ZERO && line.credit_amount == Decimal::ZERO {
                return Err(AppError::Validation(format!(
                    "Line {}: debit or credit amount must be non-zero", i + 1
                )));
            }
            if line.debit_amount < Decimal::ZERO || line.credit_amount < Decimal::ZERO {
                return Err(AppError::Validation(format!(
                    "Line {}: amounts cannot be negative", i + 1
                )));
            }
            // Validate account exists and is active
            let account = AccountRepo::find_by_id(db, &line.account_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::Validation(format!(
                    "Line {}: account {} not found", i + 1, line.account_id
                )))?;
            if !account.is_active {
                return Err(AppError::Validation(format!(
                    "Line {}: account {} ({}) is inactive", i + 1, account.number, account.name
                )));
            }
        }

        // Validate date falls within an open fiscal year
        validate_fiscal_year(db, date).await?;

        let now = Utc::now().naive_utc();
        let entry_id = Uuid::new_v4().to_string();

        let entry_model = journal_entry::ActiveModel {
            id: Set(entry_id.clone()),
            date: Set(date),
            reference: Set(reference),
            description: Set(description.to_string()),
            status: Set(JournalStatus::Draft.to_string()),
            currency_id: Set(currency_id),
            exchange_rate: Set(exchange_rate),
            created_by: Set(created_by),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let entry = JournalRepo::create_entry(db, entry_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut created_lines = Vec::new();
        for line in lines {
            let line_model = journal_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                journal_entry_id: Set(entry_id.clone()),
                account_id: Set(line.account_id),
                debit_amount: Set(line.debit_amount),
                credit_amount: Set(line.credit_amount),
                description: Set(line.description),
                vat_rate_id: Set(line.vat_rate_id),
                currency_amount: Set(None),
                base_currency_amount: Set(None),
            };

            let created = JournalRepo::create_line(db, line_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            created_lines.push(created);
        }

        Ok((entry, created_lines))
    }

    pub async fn post_entry(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<journal_entry::Model, AppError> {
        let entry = JournalRepo::find_entry_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Journal entry not found".to_string()))?;

        if entry.status != JournalStatus::Draft.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot post entry with status '{}'",
                entry.status
            )));
        }

        // Validate posting date is in an open fiscal year
        validate_fiscal_year(db, entry.date).await?;

        let mut model: journal_entry::ActiveModel = entry.into();
        model.status = Set(JournalStatus::Posted.to_string());
        tracing::info!(journal_id = %id, action = "posted", "Journal entry posted");
        model.updated_at = Set(Utc::now().naive_utc());

        JournalRepo::update_entry(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn bulk_post(
        db: &DatabaseConnection,
        entry_ids: Option<Vec<String>>,
        all_drafts: bool,
    ) -> Result<u32, AppError> {
        let entries = if all_drafts {
            JournalRepo::find_all_by_status(db, JournalStatus::Draft.as_str())
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
        } else {
            let ids = entry_ids.unwrap_or_default();
            let mut entries = Vec::new();
            for id in &ids {
                if let Some(entry) = JournalRepo::find_entry_by_id(db, id)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                    .filter(|e| e.status == JournalStatus::Draft.as_str())
                {
                    entries.push(entry);
                }
            }
            entries
        };

        let mut posted = 0u32;
        for entry in entries {
            validate_fiscal_year(db, entry.date).await?;
            let mut model: journal_entry::ActiveModel = entry.into();
            model.status = Set(JournalStatus::Posted.to_string());
            model.updated_at = Set(Utc::now().naive_utc());
            JournalRepo::update_entry(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            posted += 1;
        }

        Ok(posted)
    }

    pub async fn reverse_entry(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<(journal_entry::Model, Vec<journal_line::Model>), AppError> {
        let entry = JournalRepo::find_entry_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Journal entry not found".to_string()))?;

        if entry.status != JournalStatus::Posted.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot reverse entry with status '{}'",
                entry.status
            )));
        }

        let original_lines = JournalRepo::find_lines_by_entry(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Create reversal lines (swap debits/credits)
        let reversal_lines: Vec<JournalLineInput> = original_lines
            .iter()
            .map(|l| JournalLineInput {
                account_id: l.account_id.clone(),
                debit_amount: l.credit_amount,
                credit_amount: l.debit_amount,
                description: l.description.clone(),
                vat_rate_id: l.vat_rate_id.clone(),
            })
            .collect();

        // Create reversal entry
        let rev_desc = format!("Reversal of: {}", entry.description);
        let rev_ref = format!("REV-{}", entry.reference.as_deref().unwrap_or(&entry.id));
        let rev_currency = entry.currency_id.clone();
        let rev_rate = entry.exchange_rate;

        let reversal = Self::create(
            db,
            entry.date,
            &rev_desc,
            Some(rev_ref),
            rev_currency,
            rev_rate,
            Some(user_id.to_string()),
            reversal_lines,
        )
        .await?;

        // Mark original as reversed
        let mut original_model: journal_entry::ActiveModel = entry.into();
        original_model.status = Set(JournalStatus::Reversed.to_string());
        tracing::info!(journal_id = %id, action = "reversed", "Journal entry reversed");
        original_model.updated_at = Set(Utc::now().naive_utc());

        JournalRepo::update_entry(db, original_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(reversal)
    }
}

/// Validate that the given date falls within an open fiscal year.
async fn validate_fiscal_year(
    db: &DatabaseConnection,
    date: chrono::NaiveDate,
) -> Result<(), AppError> {
    let (years, _) = FiscalYearRepo::find_paginated(db, 1, 100, None)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // If no fiscal years defined, allow any date (initial setup)
    if years.is_empty() {
        return Ok(());
    }

    for fy in &years {
        if date >= fy.start_date && date <= fy.end_date {
            if fy.status == FiscalYearStatus::Closed.as_str() {
                return Err(AppError::Validation(format!(
                    "Posting period is closed: {} ({} to {})",
                    fy.name, fy.start_date, fy.end_date
                )));
            }
            return Ok(());
        }
    }

    Err(AppError::Validation(format!(
        "No fiscal year found for date {}",
        date
    )))
}
