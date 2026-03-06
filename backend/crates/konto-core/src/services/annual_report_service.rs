use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::{AnnualReportStatus, JournalStatus};
use konto_db::entities::{annual_report, fiscal_year};
use konto_db::repository::annual_report_repo::AnnualReportRepo;
use konto_db::repository::fiscal_year_repo::FiscalYearRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use uuid::Uuid;

use super::annual_report_note_service::AnnualReportNoteService;
use super::report_service::ReportService;
use super::report_types::*;
use super::shareholder_service::ShareholderService;

pub struct AnnualReportService;

impl AnnualReportService {
    /// Get or create the annual report record for a fiscal year.
    pub async fn get_or_create(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<annual_report::Model, AppError> {
        if let Some(report) = AnnualReportRepo::find_by_fiscal_year(db, fiscal_year_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        {
            return Ok(report);
        }

        let now = Utc::now().naive_utc();
        let model = annual_report::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            fiscal_year_id: Set(fiscal_year_id.to_string()),
            status: Set(AnnualReportStatus::Draft.to_string()),
            generated_at: Set(None),
            generated_by: Set(None),
            pdf_path: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        // Seed default notes for this fiscal year
        AnnualReportNoteService::seed_defaults(db, fiscal_year_id).await?;

        AnnualReportRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Build complete data for the annual report PDF.
    pub async fn build_data(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<AnnualReportData, AppError> {
        let fy = FiscalYearRepo::find_by_id(db, fiscal_year_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fiscal year not found".into()))?;

        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;

        // Current year reports
        let bs_current =
            ReportService::swiss_balance_sheet(db, fy.end_date).await?;
        let is_current =
            ReportService::swiss_income_statement(db, fy.start_date, fy.end_date).await?;

        // Prior year (if exists)
        let (bs_prior, is_prior) = build_prior_year(db, &fy).await?;

        // Shareholders
        let shareholders_models = ShareholderService::list(db).await?;
        let shareholders: Vec<ShareholderData> = shareholders_models
            .into_iter()
            .map(|s| ShareholderData {
                name: s.name,
                city: s.city,
                role: s.role,
                signing_rights: s.signing_rights,
            })
            .collect();

        // Notes
        let notes_models =
            AnnualReportNoteService::get_all_for_year(db, fiscal_year_id).await?;
        let mut notes: HashMap<String, serde_json::Value> = HashMap::new();
        let mut ordered_notes: Vec<NoteEntry> = Vec::new();
        for note in &notes_models {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&note.content_json) {
                notes.insert(note.section_key.clone(), val.clone());
                ordered_notes.push(NoteEntry {
                    section_key: note.section_key.clone(),
                    label: note.label.clone(),
                    section_type: note.section_type.clone(),
                    content: val,
                    sort_order: note.sort_order,
                });
            }
        }
        ordered_notes.sort_by_key(|n| n.sort_order);

        // FX rates as of fiscal year end
        let fx_rates = load_fx_rates(db, fy.end_date).await?;

        // Retained earnings from prior year
        let prior_retained = get_account_balance(db, 2970, fy.start_date).await;
        let current_net = is_current.subtotals.net_result;

        Ok(AnnualReportData {
            company_name: settings.legal_name,
            company_city: settings.city,
            jurisdiction: settings.jurisdiction,
            legal_entity_type: settings.legal_entity_type.unwrap_or_default(),
            fiscal_year_name: fy.name,
            fiscal_year_end: fy.end_date.to_string(),
            fiscal_year_start: fy.start_date.to_string(),
            balance_sheet_current: bs_current,
            balance_sheet_prior: bs_prior,
            income_statement_current: is_current,
            income_statement_prior: is_prior,
            shareholders,
            notes,
            ordered_notes,
            fx_rates,
            prior_retained_earnings: prior_retained,
            current_net_result: current_net,
            audit_optout: settings.audit_optout,
        })
    }

    /// Mark the annual report as finalized.
    pub async fn finalize(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        user_id: &str,
    ) -> Result<annual_report::Model, AppError> {
        let report = Self::get_or_create(db, fiscal_year_id).await?;
        let now = Utc::now().naive_utc();
        let mut model: annual_report::ActiveModel = report.into();
        model.status = Set(AnnualReportStatus::Finalized.to_string());
        model.generated_at = Set(Some(now));
        model.generated_by = Set(Some(user_id.to_string()));
        model.updated_at = Set(now);

        AnnualReportRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Store the PDF path after generation.
    pub async fn set_pdf_path(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        pdf_path: &str,
    ) -> Result<(), AppError> {
        let report = Self::get_or_create(db, fiscal_year_id).await?;
        let now = Utc::now().naive_utc();
        let mut model: annual_report::ActiveModel = report.into();
        model.pdf_path = Set(Some(pdf_path.to_string()));
        model.updated_at = Set(now);
        AnnualReportRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

async fn build_prior_year(
    db: &DatabaseConnection,
    fy: &fiscal_year::Model,
) -> Result<(Option<SwissBalanceSheet>, Option<SwissIncomeStatement>), AppError> {
    // Find fiscal year ending just before current one starts
    let all_fys = FiscalYearRepo::find_all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let prior = all_fys
        .iter()
        .filter(|f| f.end_date < fy.start_date)
        .max_by_key(|f| f.end_date);

    match prior {
        Some(pfy) => {
            let bs = ReportService::swiss_balance_sheet(db, pfy.end_date).await?;
            let is = ReportService::swiss_income_statement(db, pfy.start_date, pfy.end_date).await?;
            Ok((Some(bs), Some(is)))
        }
        None => Ok((None, None)),
    }
}

async fn load_fx_rates(
    db: &DatabaseConnection,
    as_of: NaiveDate,
) -> Result<Vec<FxRateData>, AppError> {
    use konto_db::entities::{currency, exchange_rate};
    use sea_orm::*;

    // Load all currencies for code lookup
    let currencies: Vec<currency::Model> = currency::Entity::find()
        .all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    let currency_map: HashMap<String, String> =
        currencies.into_iter().map(|c| (c.id.clone(), c.code)).collect();

    let rates: Vec<exchange_rate::Model> = exchange_rate::Entity::find()
        .filter(exchange_rate::Column::ValidDate.lte(as_of))
        .order_by_desc(exchange_rate::Column::ValidDate)
        .all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Deduplicate: keep only the latest rate per from_currency_id
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for rate in rates {
        if seen.insert(rate.from_currency_id.clone()) {
            let from_code = currency_map
                .get(&rate.from_currency_id)
                .cloned()
                .unwrap_or_else(|| rate.from_currency_id.clone());
            let to_code = currency_map
                .get(&rate.to_currency_id)
                .cloned()
                .unwrap_or("CHF".to_string());
            result.push(FxRateData {
                currency_from: from_code,
                currency_to: to_code,
                rate: rate.rate,
                valid_date: rate.valid_date.to_string(),
            });
        }
    }
    Ok(result)
}

async fn get_account_balance(
    db: &DatabaseConnection,
    account_number: i32,
    before_date: NaiveDate,
) -> Decimal {
    use konto_db::entities::{account, journal_entry, journal_line};
    use sea_orm::*;

    let rows: Vec<(journal_line::Model, Option<account::Model>)> =
        journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .filter(journal_entry::Column::Date.lt(before_date))
            .find_also_related(account::Entity)
            .all(db)
            .await
            .unwrap_or_default();

    rows.iter()
        .filter(|(_, acct)| {
            acct.as_ref()
                .map(|a| a.number == account_number)
                .unwrap_or(false)
        })
        .map(|(line, _)| line.debit_amount - line.credit_amount)
        .sum()
}
