use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::CreditNoteStatus;
use konto_db::entities::{credit_note, credit_note_line, vat_rate};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::credit_note_repo::CreditNoteRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use super::credit_note_workflow;

pub struct CreditNoteDetail {
    pub credit_note: credit_note::Model,
    pub lines: Vec<credit_note_line::Model>,
    pub contact_name: Option<String>,
    pub invoice_number: Option<String>,
}

pub struct CreditNoteService;

impl CreditNoteService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<credit_note::Model>, u64), AppError> {
        CreditNoteRepo::find_paginated(db, page, per_page, status, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<CreditNoteDetail, AppError> {
        let cn = Self::get_model(db, id).await?;

        let lines = CreditNoteRepo::find_lines_by_credit_note(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let contact = ContactRepo::find_by_id(db, &cn.contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let contact_name = contact.map(|c| c.name1);

        let invoice_number = if let Some(ref inv_id) = cn.invoice_id {
            InvoiceRepo::find_by_id(db, inv_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .and_then(|i| i.invoice_number)
        } else {
            None
        };

        Ok(CreditNoteDetail { credit_note: cn, lines, contact_name, invoice_number })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        contact_id: &str,
        invoice_id: Option<String>,
        issue_date: chrono::NaiveDate,
        currency_id: Option<String>,
        notes: Option<String>,
        line_inputs: Vec<LineInput>,
        user_id: Option<String>,
    ) -> Result<credit_note::Model, AppError> {
        if line_inputs.is_empty() {
            return Err(AppError::Validation("At least one line is required".into()));
        }

        ContactRepo::find_by_id(db, contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;

        if let Some(ref inv_id) = invoice_id {
            InvoiceRepo::find_by_id(db, inv_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        }

        let now = Utc::now().naive_utc();
        let cn_id = Uuid::new_v4().to_string();
        let computed = compute_lines(db, &line_inputs).await?;

        let model = credit_note::ActiveModel {
            id: Set(cn_id.clone()),
            credit_note_number: Set(None),
            invoice_id: Set(invoice_id),
            contact_id: Set(contact_id.to_string()),
            status: Set(CreditNoteStatus::Draft.to_string()),
            issue_date: Set(issue_date),
            currency_id: Set(currency_id),
            subtotal: Set(computed.subtotal),
            vat_amount: Set(computed.total_vat),
            total: Set(computed.total),
            notes: Set(notes),
            journal_entry_id: Set(None),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let cn = CreditNoteRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        for (pos, (input, cl)) in line_inputs.iter().zip(&computed.lines).enumerate() {
            let line_model = credit_note_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                credit_note_id: Set(cn_id.clone()),
                sort_order: Set((pos + 1) as i32),
                description: Set(input.description.clone()),
                quantity: Set(input.quantity),
                unit_price: Set(input.unit_price),
                vat_rate_id: Set(input.vat_rate_id.clone()),
                vat_amount: Set(cl.vat_amount),
                line_total: Set(cl.line_total),
                account_id: Set(input.account_id.clone()),
                created_at: Set(now),
            };
            CreditNoteRepo::create_line(db, line_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok(cn)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        contact_id: &str,
        invoice_id: Option<String>,
        issue_date: chrono::NaiveDate,
        currency_id: Option<String>,
        notes: Option<String>,
        line_inputs: Vec<LineInput>,
    ) -> Result<credit_note::Model, AppError> {
        let existing = Self::get_model(db, id).await?;
        if existing.status != CreditNoteStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft credit notes can be updated".into()));
        }

        if line_inputs.is_empty() {
            return Err(AppError::Validation("At least one line is required".into()));
        }

        let now = Utc::now().naive_utc();
        let computed = compute_lines(db, &line_inputs).await?;

        CreditNoteRepo::delete_lines_by_credit_note(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        for (pos, (input, cl)) in line_inputs.iter().zip(&computed.lines).enumerate() {
            let line_model = credit_note_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                credit_note_id: Set(id.to_string()),
                sort_order: Set((pos + 1) as i32),
                description: Set(input.description.clone()),
                quantity: Set(input.quantity),
                unit_price: Set(input.unit_price),
                vat_rate_id: Set(input.vat_rate_id.clone()),
                vat_amount: Set(cl.vat_amount),
                line_total: Set(cl.line_total),
                account_id: Set(input.account_id.clone()),
                created_at: Set(now),
            };
            CreditNoteRepo::create_line(db, line_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let mut model: credit_note::ActiveModel = existing.into();
        model.contact_id = Set(contact_id.to_string());
        model.invoice_id = Set(invoice_id);
        model.issue_date = Set(issue_date);
        model.currency_id = Set(currency_id);
        model.subtotal = Set(computed.subtotal);
        model.vat_amount = Set(computed.total_vat);
        model.total = Set(computed.total);
        model.notes = Set(notes);
        model.updated_at = Set(now);

        CreditNoteRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_model(db, id).await?;
        if existing.status != CreditNoteStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft credit notes can be deleted".into()));
        }
        CreditNoteRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn issue(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<credit_note::Model, AppError> {
        credit_note_workflow::issue(db, id, user_id).await
    }

    pub async fn apply(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<credit_note::Model, AppError> {
        credit_note_workflow::apply(db, id).await
    }

    pub async fn cancel(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<credit_note::Model, AppError> {
        credit_note_workflow::cancel(db, id, user_id).await
    }

    pub(crate) async fn get_model(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<credit_note::Model, AppError> {
        CreditNoteRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Credit note not found".into()))
    }
}

// --- helpers ---

pub struct LineInput {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: String,
}

struct ComputedLine {
    line_total: Decimal,
    vat_amount: Decimal,
}

struct ComputedTotals {
    subtotal: Decimal,
    total_vat: Decimal,
    total: Decimal,
    lines: Vec<ComputedLine>,
}

async fn compute_lines(
    db: &DatabaseConnection,
    inputs: &[LineInput],
) -> Result<ComputedTotals, AppError> {
    let mut lines = Vec::new();
    let mut subtotal = Decimal::ZERO;
    let mut total_vat = Decimal::ZERO;

    for input in inputs {
        let line_total = input.quantity * input.unit_price;

        let vat_amount = if let Some(ref vat_id) = input.vat_rate_id {
            let vat = vat_rate::Entity::find_by_id(vat_id)
                .one(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound(format!("VAT rate {vat_id} not found")))?;
            line_total * vat.rate / Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        AccountRepo::find_by_id(db, &input.account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::NotFound(format!("Account {} not found", input.account_id))
            })?;

        subtotal += line_total;
        total_vat += vat_amount;
        lines.push(ComputedLine { line_total, vat_amount });
    }

    Ok(ComputedTotals {
        subtotal,
        total_vat,
        total: subtotal + total_vat,
        lines,
    })
}
