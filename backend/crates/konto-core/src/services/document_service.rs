use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::DocumentStatus;
use konto_db::entities::{document, document_line_item};
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::document_repo::DocumentRepo;
use konto_db::repository::project_repo::ProjectRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::language::{normalize_language, normalize_or_default};
use super::document_workflow;

pub struct DocumentDetail {
    pub document: document::Model,
    pub lines: Vec<document_line_item::Model>,
    pub contact_name: Option<String>,
    pub project_name: Option<String>,
}

pub struct DocumentService;

impl DocumentService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        doc_type: Option<&str>,
        status: Option<&str>,
        contact_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<document::Model>, u64), AppError> {
        DocumentRepo::find_paginated(db, page, per_page, doc_type, status, contact_id, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<DocumentDetail, AppError> {
        let doc = Self::get_document_model(db, id).await?;
        let lines = DocumentRepo::find_lines_by_document(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let contact_name = ContactRepo::find_by_id(db, &doc.contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map(|c| c.name1);

        let project_name = if let Some(ref pid) = doc.project_id {
            konto_db::repository::project_repo::ProjectRepo::find_by_id(db, pid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|p| p.name)
        } else {
            None
        };

        Ok(DocumentDetail { document: doc, lines, contact_name, project_name })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        doc_type: &str,
        title: &str,
        contact_id: &str,
        project_id: Option<String>,
        template_id: Option<String>,
        content_json: &str,
        language: Option<String>,
        currency_id: Option<String>,
        valid_until: Option<chrono::NaiveDate>,
        line_inputs: Vec<LineInput>,
        user_id: Option<String>,
    ) -> Result<document::Model, AppError> {
        ContactRepo::find_by_id(db, contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".to_string()))?;
        let resolved_language = resolve_document_language(
            db,
            language.as_deref(),
            Some(contact_id),
            project_id.as_deref(),
        )
        .await?;

        let now = Utc::now().naive_utc();
        let doc_id = Uuid::new_v4().to_string();
        let totals = compute_totals(&line_inputs);

        let model = document::ActiveModel {
            id: Set(doc_id.clone()),
            doc_type: Set(doc_type.to_string()),
            doc_number: Set(None),
            title: Set(title.to_string()),
            status: Set(DocumentStatus::Draft.to_string()),
            contact_id: Set(contact_id.to_string()),
            project_id: Set(project_id),
            template_id: Set(template_id),
            content_json: Set(content_json.to_string()),
            language: Set(Some(resolved_language)),
            currency_id: Set(currency_id),
            subtotal: Set(totals.subtotal),
            vat_rate: Set(totals.vat_rate),
            vat_amount: Set(totals.vat_amount),
            total: Set(totals.total),
            valid_until: Set(valid_until),
            issued_at: Set(None),
            signed_at: Set(None),
            converted_from: Set(None),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let doc = DocumentRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        create_lines(db, &doc_id, &line_inputs, now).await?;
        Ok(doc)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        title: &str,
        contact_id: &str,
        project_id: Option<String>,
        template_id: Option<String>,
        content_json: &str,
        language: Option<String>,
        currency_id: Option<String>,
        valid_until: Option<chrono::NaiveDate>,
        line_inputs: Vec<LineInput>,
    ) -> Result<document::Model, AppError> {
        let existing = Self::get_document_model(db, id).await?;
        if existing.status != DocumentStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft documents can be updated".into()));
        }
        let resolved_language = resolve_document_language(
            db,
            language.as_deref(),
            Some(contact_id),
            project_id.as_deref(),
        )
        .await?;

        let now = Utc::now().naive_utc();
        let totals = compute_totals(&line_inputs);

        DocumentRepo::delete_lines_by_document(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        create_lines(db, id, &line_inputs, now).await?;

        let mut model: document::ActiveModel = existing.into();
        model.title = Set(title.to_string());
        model.contact_id = Set(contact_id.to_string());
        model.project_id = Set(project_id);
        model.template_id = Set(template_id);
        model.content_json = Set(content_json.to_string());
        model.language = Set(Some(resolved_language));
        model.currency_id = Set(currency_id);
        model.subtotal = Set(totals.subtotal);
        model.vat_rate = Set(totals.vat_rate);
        model.vat_amount = Set(totals.vat_amount);
        model.total = Set(totals.total);
        model.valid_until = Set(valid_until);
        model.updated_at = Set(now);

        DocumentRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_document_model(db, id).await?;
        if existing.status != DocumentStatus::Draft.as_str() {
            return Err(AppError::Validation("Only draft documents can be deleted".into()));
        }
        DocumentRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn send_document(
        db: &DatabaseConnection, id: &str, user_id: &str,
    ) -> Result<document::Model, AppError> {
        document_workflow::send_document(db, id, user_id).await
    }

    pub async fn accept_document(
        db: &DatabaseConnection, id: &str, user_id: &str,
    ) -> Result<document::Model, AppError> {
        document_workflow::accept_document(db, id, user_id).await
    }

    pub async fn reject_document(
        db: &DatabaseConnection, id: &str, user_id: &str,
    ) -> Result<document::Model, AppError> {
        document_workflow::reject_document(db, id, user_id).await
    }

    pub async fn convert_document(
        db: &DatabaseConnection, id: &str, target_type: &str, user_id: &str,
    ) -> Result<document::Model, AppError> {
        document_workflow::convert_document(db, id, target_type, user_id).await
    }

    pub(crate) async fn get_document_model(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<document::Model, AppError> {
        DocumentRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Document not found".to_string()))
    }
}

async fn resolve_document_language(
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
    pub unit: Option<String>,
    pub unit_price: Decimal,
    pub discount_pct: Decimal,
}

struct Totals {
    subtotal: Decimal,
    vat_rate: Decimal,
    vat_amount: Decimal,
    total: Decimal,
}

fn compute_totals(lines: &[LineInput]) -> Totals {
    let mut subtotal = Decimal::ZERO;
    for line in lines {
        let line_total = line.quantity * line.unit_price;
        let discount = line_total * line.discount_pct / Decimal::from(100);
        subtotal += line_total - discount;
    }
    // VAT computed at document level (vat_rate stored on document, not per-line)
    Totals {
        subtotal,
        vat_rate: Decimal::ZERO,
        vat_amount: Decimal::ZERO,
        total: subtotal,
    }
}

async fn create_lines(
    db: &DatabaseConnection,
    doc_id: &str,
    inputs: &[LineInput],
    now: chrono::NaiveDateTime,
) -> Result<(), AppError> {
    for (pos, input) in inputs.iter().enumerate() {
        let line_total = input.quantity * input.unit_price;
        let discount = line_total * input.discount_pct / Decimal::from(100);

        let model = document_line_item::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            document_id: Set(doc_id.to_string()),
            position: Set((pos + 1) as i32),
            description: Set(input.description.clone()),
            quantity: Set(input.quantity),
            unit: Set(input.unit.clone()),
            unit_price: Set(input.unit_price),
            discount_pct: Set(input.discount_pct),
            total: Set(line_total - discount),
            created_at: Set(now),
        };
        DocumentRepo::create_line(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }
    Ok(())
}
