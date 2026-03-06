use chrono::{Months, NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::recurring_invoice;
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::recurring_invoice_repo::RecurringInvoiceRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::invoice_service::{InvoiceService, LineInput};

/// JSON structure stored in template_data
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TemplateData {
    pub language: Option<String>,
    pub currency_id: Option<String>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub lines: Vec<TemplateLineItem>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TemplateLineItem {
    pub description: String,
    pub quantity: rust_decimal::Decimal,
    pub unit_price: rust_decimal::Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: String,
}

pub struct RecurringInvoiceService;

impl RecurringInvoiceService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        is_active: Option<bool>,
        search: Option<&str>,
    ) -> Result<(Vec<recurring_invoice::Model>, u64), AppError> {
        RecurringInvoiceRepo::find_paginated(db, page, per_page, is_active, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<recurring_invoice::Model, AppError> {
        RecurringInvoiceRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Recurring invoice not found".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        contact_id: &str,
        project_id: Option<String>,
        template_data: &TemplateData,
        frequency: &str,
        interval_days: Option<i32>,
        next_run_date: NaiveDate,
        end_date: Option<NaiveDate>,
        auto_send: bool,
        user_id: Option<String>,
    ) -> Result<recurring_invoice::Model, AppError> {
        validate_frequency(frequency, interval_days)?;

        ContactRepo::find_by_id(db, contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;

        if template_data.lines.is_empty() {
            return Err(AppError::Validation("At least one line is required".into()));
        }

        let now = Utc::now().naive_utc();
        let json = serde_json::to_string(template_data)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let model = recurring_invoice::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            contact_id: Set(contact_id.to_string()),
            project_id: Set(project_id),
            template_data: Set(json),
            frequency: Set(frequency.to_string()),
            interval_days: Set(interval_days),
            next_run_date: Set(next_run_date),
            end_date: Set(end_date),
            auto_send: Set(auto_send),
            is_active: Set(true),
            last_generated_at: Set(None),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        RecurringInvoiceRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        contact_id: &str,
        project_id: Option<String>,
        template_data: &TemplateData,
        frequency: &str,
        interval_days: Option<i32>,
        next_run_date: NaiveDate,
        end_date: Option<NaiveDate>,
        auto_send: bool,
        is_active: bool,
    ) -> Result<recurring_invoice::Model, AppError> {
        validate_frequency(frequency, interval_days)?;
        let existing = Self::get_by_id(db, id).await?;

        if template_data.lines.is_empty() {
            return Err(AppError::Validation("At least one line is required".into()));
        }

        let json = serde_json::to_string(template_data)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut model: recurring_invoice::ActiveModel = existing.into();
        model.contact_id = Set(contact_id.to_string());
        model.project_id = Set(project_id);
        model.template_data = Set(json);
        model.frequency = Set(frequency.to_string());
        model.interval_days = Set(interval_days);
        model.next_run_date = Set(next_run_date);
        model.end_date = Set(end_date);
        model.auto_send = Set(auto_send);
        model.is_active = Set(is_active);
        model.updated_at = Set(Utc::now().naive_utc());

        RecurringInvoiceRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_by_id(db, id).await?;
        if existing.is_active {
            return Err(AppError::Validation(
                "Deactivate the recurring invoice before deleting".into(),
            ));
        }
        RecurringInvoiceRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn generate_due_invoices(db: &DatabaseConnection) -> Result<u64, AppError> {
        let due = RecurringInvoiceRepo::find_due(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut count = 0u64;
        for rec in due {
            if let Err(e) = generate_single(db, &rec).await {
                tracing::error!(
                    "Failed to generate invoice for recurring {}: {}",
                    rec.id, e
                );
                continue;
            }
            count += 1;
        }

        Ok(count)
    }
}

async fn generate_single(
    db: &DatabaseConnection,
    rec: &recurring_invoice::Model,
) -> Result<(), AppError> {
    let tpl: TemplateData = serde_json::from_str(&rec.template_data)
        .map_err(|e| AppError::Internal(format!("Bad template_data: {e}")))?;

    let today = Utc::now().naive_utc().date();
    let due_date = today + chrono::Duration::days(30);

    let lines: Vec<LineInput> = tpl
        .lines
        .iter()
        .map(|l| LineInput {
            description: l.description.clone(),
            quantity: l.quantity,
            unit_price: l.unit_price,
            vat_rate_id: l.vat_rate_id.clone(),
            account_id: Some(l.account_id.clone()),
            discount_percent: None,
        })
        .collect();

    let inv = InvoiceService::create(
        db,
        &rec.contact_id,
        rec.project_id.clone(),
        today,
        due_date,
        tpl.language.clone(),
        tpl.currency_id.clone(),
        tpl.notes.clone(),
        tpl.payment_terms.clone(),
        lines,
        rec.created_by.clone(),
        None,
        None,
        None,
        None,
    )
    .await?;

    if let Some(user_id) = rec.created_by.as_ref().filter(|_| rec.auto_send) {
        let _ = InvoiceService::send_invoice(db, &inv.id, user_id).await;
    }

    let next = calc_next_run_date(rec.next_run_date, &rec.frequency, rec.interval_days);
    let now = Utc::now().naive_utc();
    let mut model: recurring_invoice::ActiveModel = rec.clone().into();
    model.next_run_date = Set(next);
    model.last_generated_at = Set(Some(now));
    model.updated_at = Set(now);

    RecurringInvoiceRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

fn calc_next_run_date(
    current: NaiveDate,
    frequency: &str,
    interval_days: Option<i32>,
) -> NaiveDate {
    match frequency {
        "monthly" => current.checked_add_months(Months::new(1)).unwrap_or(current),
        "quarterly" => current.checked_add_months(Months::new(3)).unwrap_or(current),
        "semi_annual" => current.checked_add_months(Months::new(6)).unwrap_or(current),
        "annual" => current.checked_add_months(Months::new(12)).unwrap_or(current),
        "custom" => {
            let days = interval_days.unwrap_or(30);
            current + chrono::Duration::days(days as i64)
        }
        _ => current.checked_add_months(Months::new(1)).unwrap_or(current),
    }
}

fn validate_frequency(frequency: &str, interval_days: Option<i32>) -> Result<(), AppError> {
    let valid = ["monthly", "quarterly", "semi_annual", "annual", "custom"];
    if !valid.contains(&frequency) {
        return Err(AppError::Validation(format!(
            "Invalid frequency: {frequency}. Must be one of: {}", valid.join(", ")
        )));
    }
    if frequency == "custom" && interval_days.is_none() {
        return Err(AppError::Validation(
            "interval_days is required for custom frequency".into(),
        ));
    }
    Ok(())
}
