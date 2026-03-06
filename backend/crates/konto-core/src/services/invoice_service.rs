use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::InvoiceStatus;
use konto_db::entities::{invoice, invoice_line, vat_rate, default_account};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use konto_db::repository::project_repo::ProjectRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::language::{normalize_language, normalize_or_default};
use super::invoice_workflow;

pub struct InvoiceDetail {
    pub invoice: invoice::Model,
    pub lines: Vec<invoice_line::Model>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_language: Option<String>,
    pub project_name: Option<String>,
    pub contact_person_name: Option<String>,
}

pub struct InvoiceService;

impl InvoiceService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status: Option<&str>,
        contact_id: Option<&str>,
        project_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<invoice::Model>, u64), AppError> {
        InvoiceRepo::find_paginated(db, page, per_page, status, contact_id, project_id, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<InvoiceDetail, AppError> {
        let inv = InvoiceRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

        let lines = InvoiceRepo::find_lines_by_invoice(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let contact = ContactRepo::find_by_id(db, &inv.contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let contact_name = contact.as_ref().map(|c| c.name1.clone());
        let contact_language = contact
            .as_ref()
            .and_then(|c| normalize_language(c.language.as_deref()));
        let contact_email = contact.and_then(|c| c.email);

        let project_name = if let Some(ref pid) = inv.project_id {
            ProjectRepo::find_by_id(db, pid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|p| p.name)
        } else {
            None
        };

        // Resolve contact person name (look up from contacts table, since migration 000081 moved persons to contacts)
        let contact_person_name = if let Some(ref cp_id) = inv.contact_person_id {
            ContactRepo::find_by_id(db, cp_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|cp| cp.name1)
        } else {
            None
        };

        Ok(InvoiceDetail { invoice: inv, lines, contact_name, contact_email, contact_language, project_name, contact_person_name })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        contact_id: &str,
        project_id: Option<String>,
        issue_date: chrono::NaiveDate,
        due_date: chrono::NaiveDate,
        language: Option<String>,
        currency_id: Option<String>,
        notes: Option<String>,
        payment_terms: Option<String>,
        line_inputs: Vec<LineInput>,
        user_id: Option<String>,
        header_text: Option<String>,
        footer_text: Option<String>,
        contact_person_id: Option<String>,
        bank_account_id: Option<String>,
    ) -> Result<invoice::Model, AppError> {
        if line_inputs.is_empty() {
            return Err(AppError::Validation("At least one line is required".to_string()));
        }

        if due_date < issue_date {
            return Err(AppError::Validation("Due date must be on or after issue date".to_string()));
        }

        for (i, input) in line_inputs.iter().enumerate() {
            if input.quantity <= Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: quantity must be greater than 0", i + 1)));
            }
            if input.unit_price < Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: unit price cannot be negative", i + 1)));
            }
        }

        // Verify contact exists
        ContactRepo::find_by_id(db, contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".to_string()))?;

        let now = Utc::now().naive_utc();
        let invoice_id = Uuid::new_v4().to_string();
        let resolved_language = resolve_invoice_language(
            db,
            language.as_deref(),
            Some(contact_id),
            project_id.as_deref(),
        )
        .await?;

        // Compute totals from lines
        let computed = compute_lines(db, &line_inputs).await?;

        let model = invoice::ActiveModel {
            id: Set(invoice_id.clone()),
            invoice_number: Set(None),
            contact_id: Set(contact_id.to_string()),
            project_id: Set(project_id),
            status: Set(InvoiceStatus::Draft.to_string()),
            issue_date: Set(issue_date),
            due_date: Set(due_date),
            language: Set(Some(resolved_language)),
            currency_id: Set(currency_id),
            subtotal: Set(computed.subtotal),
            vat_amount: Set(computed.total_vat),
            total: Set(computed.total),
            notes: Set(notes),
            payment_terms: Set(payment_terms),
            journal_entry_id: Set(None),
            payment_journal_entry_id: Set(None),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
            template_id: Set(None),
            content_json: Set(None),
            header_text: Set(header_text),
            footer_text: Set(footer_text),
            contact_person_id: Set(contact_person_id),
            bank_account_id: Set(bank_account_id),
        };

        let inv = InvoiceRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Create lines
        for (pos, (input, cl)) in line_inputs.iter().zip(&computed.lines).enumerate() {
            let line_model = invoice_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                invoice_id: Set(invoice_id.clone()),
                position: Set((pos + 1) as i32),
                description: Set(input.description.clone()),
                quantity: Set(input.quantity),
                unit_price: Set(input.unit_price),
                vat_rate_id: Set(input.vat_rate_id.clone()),
                vat_amount: Set(cl.vat_amount),
                line_total: Set(cl.line_total),
                account_id: Set(cl.account_id.clone()),
                discount_percent: Set(cl.discount_percent),
                created_at: Set(now),
                updated_at: Set(now),
            };
            InvoiceRepo::create_line(db, line_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok(inv)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        contact_id: &str,
        project_id: Option<String>,
        issue_date: chrono::NaiveDate,
        due_date: chrono::NaiveDate,
        language: Option<String>,
        currency_id: Option<String>,
        notes: Option<String>,
        payment_terms: Option<String>,
        line_inputs: Vec<LineInput>,
        header_text: Option<String>,
        footer_text: Option<String>,
        contact_person_id: Option<String>,
        bank_account_id: Option<String>,
    ) -> Result<invoice::Model, AppError> {
        let existing = Self::get_invoice_model(db, id).await?;
        if existing.status != InvoiceStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft invoices can be updated".to_string()));
        }

        if line_inputs.is_empty() {
            return Err(AppError::Validation("At least one line is required".to_string()));
        }

        if due_date < issue_date {
            return Err(AppError::Validation("Due date must be on or after issue date".to_string()));
        }

        for (i, input) in line_inputs.iter().enumerate() {
            if input.quantity <= Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: quantity must be greater than 0", i + 1)));
            }
            if input.unit_price < Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: unit price cannot be negative", i + 1)));
            }
        }

        let now = Utc::now().naive_utc();
        let resolved_language = resolve_invoice_language(
            db,
            language.as_deref(),
            Some(contact_id),
            project_id.as_deref(),
        )
        .await?;
        let computed = compute_lines(db, &line_inputs).await?;

        // Delete old lines, create new ones
        InvoiceRepo::delete_lines_by_invoice(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        for (pos, (input, cl)) in line_inputs.iter().zip(&computed.lines).enumerate() {
            let line_model = invoice_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                invoice_id: Set(id.to_string()),
                position: Set((pos + 1) as i32),
                description: Set(input.description.clone()),
                quantity: Set(input.quantity),
                unit_price: Set(input.unit_price),
                vat_rate_id: Set(input.vat_rate_id.clone()),
                vat_amount: Set(cl.vat_amount),
                line_total: Set(cl.line_total),
                account_id: Set(cl.account_id.clone()),
                discount_percent: Set(cl.discount_percent),
                created_at: Set(now),
                updated_at: Set(now),
            };
            InvoiceRepo::create_line(db, line_model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let mut model: invoice::ActiveModel = existing.into();
        model.contact_id = Set(contact_id.to_string());
        model.project_id = Set(project_id);
        model.issue_date = Set(issue_date);
        model.due_date = Set(due_date);
        model.language = Set(Some(resolved_language));
        model.currency_id = Set(currency_id);
        model.subtotal = Set(computed.subtotal);
        model.vat_amount = Set(computed.total_vat);
        model.total = Set(computed.total);
        model.notes = Set(notes);
        model.payment_terms = Set(payment_terms);
        model.header_text = Set(header_text);
        model.footer_text = Set(footer_text);
        model.contact_person_id = Set(contact_person_id);
        model.bank_account_id = Set(bank_account_id);
        model.updated_at = Set(now);

        InvoiceRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_invoice_model(db, id).await?;
        if existing.status != InvoiceStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft invoices can be deleted".to_string()));
        }
        InvoiceRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn send_invoice(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<invoice::Model, AppError> {
        invoice_workflow::send_invoice(db, id, user_id).await
    }

    pub async fn mark_paid(
        db: &DatabaseConnection,
        id: &str,
        payment_date: chrono::NaiveDate,
        payment_account_id: &str,
        user_id: &str,
    ) -> Result<invoice::Model, AppError> {
        invoice_workflow::mark_paid(db, id, payment_date, payment_account_id, user_id).await
    }

    pub async fn cancel_invoice(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<invoice::Model, AppError> {
        invoice_workflow::cancel_invoice(db, id, user_id).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn record_payment(
        db: &DatabaseConnection,
        invoice_id: &str,
        amount: Decimal,
        payment_date: chrono::NaiveDate,
        payment_account_id: &str,
        payment_method: Option<String>,
        reference: Option<String>,
        user_id: &str,
    ) -> Result<konto_db::entities::invoice_payment::Model, AppError> {
        invoice_workflow::record_payment(
            db, invoice_id, amount, payment_date,
            payment_account_id, payment_method, reference, user_id,
        ).await
    }

    pub async fn list_payments(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Vec<konto_db::entities::invoice_payment::Model>, AppError> {
        invoice_workflow::list_payments(db, invoice_id).await
    }

    pub async fn amount_paid(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Decimal, AppError> {
        invoice_workflow::amount_paid(db, invoice_id).await
    }

    pub async fn duplicate(
        db: &DatabaseConnection,
        id: &str,
        user_id: Option<String>,
    ) -> Result<invoice::Model, AppError> {
        let detail = Self::get_by_id(db, id).await?;
        let original = detail.invoice;
        let original_lines = detail.lines;

        let today = chrono::Utc::now().date_naive();
        let due_date = today + chrono::Duration::days(30);

        let line_inputs: Vec<LineInput> = original_lines
            .into_iter()
            .map(|l| LineInput {
                description: l.description,
                quantity: l.quantity,
                unit_price: l.unit_price,
                vat_rate_id: l.vat_rate_id,
                account_id: Some(l.account_id),
                discount_percent: l.discount_percent,
            })
            .collect();

        Self::create(
            db,
            &original.contact_id,
            original.project_id,
            today,
            due_date,
            original.language,
            original.currency_id,
            original.notes,
            original.payment_terms,
            line_inputs,
            user_id,
            original.header_text,
            original.footer_text,
            original.contact_person_id,
            original.bank_account_id,
        )
        .await
    }

    pub(crate) async fn get_invoice_model(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<invoice::Model, AppError> {
        InvoiceRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))
    }
}

async fn resolve_invoice_language(
    db: &DatabaseConnection,
    explicit_language: Option<&str>,
    contact_id: Option<&str>,
    project_id: Option<&str>,
) -> Result<String, AppError> {
    if let Some(lang) = normalize_language(explicit_language) {
        return Ok(lang);
    }

    if let Some(pid) = project_id {
        let project_lang = ProjectRepo::find_by_id(db, pid)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .and_then(|p| normalize_language(p.language.as_deref()));
        if let Some(lang) = project_lang {
            return Ok(lang);
        }
    }

    if let Some(cid) = contact_id {
        let contact_lang = ContactRepo::find_by_id(db, cid)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .and_then(|c| normalize_language(c.language.as_deref()));
        if let Some(lang) = contact_lang {
            return Ok(lang);
        }
    }

    let settings_lang = SettingsRepo::find(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .map(|s| normalize_or_default(Some(&s.ui_language), "en"));

    Ok(settings_lang.unwrap_or_else(|| "en".to_string()))
}

// --- helpers ---

pub struct LineInput {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: Option<String>,
    pub discount_percent: Option<Decimal>,
}

struct ComputedLine {
    line_total: Decimal,
    vat_amount: Decimal,
    discount_percent: Option<Decimal>,
    account_id: String,
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

    // Lazy-load revenue default account id
    let mut revenue_default: Option<String> = None;

    for input in inputs {
        // Apply discount: line_total = qty * price * (1 - discount/100)
        let gross = input.quantity * input.unit_price;
        let line_total = if let Some(discount) = input.discount_percent {
            gross * (Decimal::ONE - discount / Decimal::from(100))
        } else {
            gross
        };

        let vat_amount = if let Some(ref vat_id) = input.vat_rate_id {
            if vat_id.is_empty() {
                Decimal::ZERO
            } else {
                let vat = vat_rate::Entity::find_by_id(vat_id)
                    .one(db)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                    .ok_or_else(|| AppError::NotFound(format!("VAT rate {vat_id} not found")))?;
                line_total * vat.rate / Decimal::from(100)
            }
        } else {
            Decimal::ZERO
        };

        // Resolve account_id: use provided, or fall back to revenue_default
        let resolved_account_id = match &input.account_id {
            Some(aid) if !aid.is_empty() => aid.clone(),
            _ => {
                if revenue_default.is_none() {
                    revenue_default = default_account::Entity::find()
                        .filter(default_account::Column::SettingKey.eq("revenue_default"))
                        .one(db)
                        .await
                        .map_err(|e| AppError::Database(e.to_string()))?
                        .and_then(|da| da.account_id);
                }
                revenue_default.clone().ok_or_else(|| {
                    AppError::Validation(
                        "No account selected and no default revenue account configured".to_string(),
                    )
                })?
            }
        };

        // Verify account exists
        AccountRepo::find_by_id(db, &resolved_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::NotFound(format!("Account {} not found", resolved_account_id))
            })?;

        subtotal += line_total;
        total_vat += vat_amount;
        lines.push(ComputedLine {
            line_total,
            vat_amount,
            discount_percent: input.discount_percent,
            account_id: resolved_account_id,
        });
    }

    Ok(ComputedTotals {
        subtotal,
        total_vat,
        total: subtotal + total_vat,
        lines,
    })
}
