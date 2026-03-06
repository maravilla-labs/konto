use chrono::{Datelike, NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::FiscalYearStatus;
use konto_db::entities::{fiscal_period, fiscal_year};
use konto_db::repository::fiscal_year_repo::{FiscalPeriodRepo, FiscalYearRepo};
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::journal_service::{JournalLineInput, JournalService};
use super::report_service::ReportService;

pub struct FiscalYearService;

impl FiscalYearService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<fiscal_year::Model>, u64), AppError> {
        FiscalYearRepo::find_paginated(db, page, per_page, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<fiscal_year::Model, AppError> {
        FiscalYearRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fiscal year not found".to_string()))
    }

    pub async fn get_periods(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<fiscal_period::Model>, AppError> {
        FiscalPeriodRepo::find_by_fiscal_year_id(db, fiscal_year_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<fiscal_year::Model, AppError> {
        if end_date <= start_date {
            return Err(AppError::Validation(
                "End date must be after start date".to_string(),
            ));
        }

        let now = Utc::now().naive_utc();
        let fy_id = Uuid::new_v4().to_string();

        let model = fiscal_year::ActiveModel {
            id: Set(fy_id.clone()),
            name: Set(name.to_string()),
            start_date: Set(start_date),
            end_date: Set(end_date),
            status: Set(FiscalYearStatus::Open.to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let fy = FiscalYearRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Auto-generate 12 monthly periods
        Self::generate_monthly_periods(db, &fy_id, start_date, end_date).await?;

        Ok(fy)
    }

    async fn generate_monthly_periods(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();
        let mut current = start_date;
        let mut period_num = 1;

        while current < end_date && period_num <= 12 {
            let month_end = if current.month() == 12 {
                NaiveDate::from_ymd_opt(current.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1)
            };
            let period_end = match month_end {
                Some(d) => d.pred_opt().unwrap_or(d),
                None => end_date,
            };
            let period_end = if period_end > end_date {
                end_date
            } else {
                period_end
            };

            let name = current.format("%B %Y").to_string();

            let period = fiscal_period::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                fiscal_year_id: Set(fiscal_year_id.to_string()),
                name: Set(name),
                start_date: Set(Some(current)),
                end_date: Set(Some(period_end)),
                period_number: Set(Some(period_num)),
                status: Set(FiscalYearStatus::Open.to_string()),
                created_at: Set(now),
                updated_at: Set(now),
            };

            FiscalPeriodRepo::create(db, period)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            current = period_end
                .succ_opt()
                .unwrap_or(end_date);
            period_num += 1;
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<fiscal_year::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        if existing.status == FiscalYearStatus::Closed.as_str() {
            return Err(AppError::Validation("Cannot edit a closed fiscal year".to_string()));
        }
        let mut model: fiscal_year::ActiveModel = existing.into();

        if let Some(name) = name {
            model.name = Set(name);
        }
        if let Some(sd) = start_date {
            model.start_date = Set(sd);
        }
        if let Some(ed) = end_date {
            model.end_date = Set(ed);
        }
        model.updated_at = Set(Utc::now().naive_utc());

        FiscalYearRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn close(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<fiscal_year::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        if existing.status == FiscalYearStatus::Closed.as_str() {
            return Err(AppError::Validation(
                "Fiscal year is already closed".to_string(),
            ));
        }

        let closing_date = existing.end_date;

        // Compute actual P&L for the fiscal year
        let pl = ReportService::profit_loss(db, existing.start_date, existing.end_date).await?;
        let net_income = pl.net_income;

        // Only create closing entry if there is a non-zero result
        if !net_income.is_zero() {
            let abs_amount = net_income.abs();
            let lines = if net_income.is_sign_positive() {
                // Profit: Debit 2979 Jahresergebnis / Credit 2970 Gewinnvortrag
                vec![
                    JournalLineInput {
                        account_id: "acct-2979".to_string(),
                        debit_amount: abs_amount,
                        credit_amount: rust_decimal::Decimal::ZERO,
                        description: Some("Closing entry - annual result".to_string()),
                        vat_rate_id: None,
                    },
                    JournalLineInput {
                        account_id: "acct-2970".to_string(),
                        debit_amount: rust_decimal::Decimal::ZERO,
                        credit_amount: abs_amount,
                        description: Some("Closing entry - retained earnings".to_string()),
                        vat_rate_id: None,
                    },
                ]
            } else {
                // Loss: Debit 2970 / Credit 2979
                vec![
                    JournalLineInput {
                        account_id: "acct-2970".to_string(),
                        debit_amount: abs_amount,
                        credit_amount: rust_decimal::Decimal::ZERO,
                        description: Some("Closing entry - retained earnings".to_string()),
                        vat_rate_id: None,
                    },
                    JournalLineInput {
                        account_id: "acct-2979".to_string(),
                        debit_amount: rust_decimal::Decimal::ZERO,
                        credit_amount: abs_amount,
                        description: Some("Closing entry - annual result".to_string()),
                        vat_rate_id: None,
                    },
                ]
            };

            let _closing = JournalService::create(
                db,
                closing_date,
                &format!("Closing entry for {}", existing.name),
                Some(format!("CLOSE-{}", existing.name)),
                None,
                None,
                Some(user_id.to_string()),
                lines,
            )
            .await?;
        }

        // Mark fiscal year as closed
        let mut model: fiscal_year::ActiveModel = existing.into();
        model.status = Set(FiscalYearStatus::Closed.to_string());
        tracing::info!(fiscal_year_id = %id, action = "closed", "Fiscal year closed");
        model.updated_at = Set(Utc::now().naive_utc());

        FiscalYearRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
