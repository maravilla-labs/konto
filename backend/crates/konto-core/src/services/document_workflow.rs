use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::DocumentStatus;
use konto_db::entities::document;
use konto_db::repository::document_repo::DocumentRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::document_service::DocumentService;

/// Send a document: assign doc_number, set status to "sent", set issued_at.
pub async fn send_document(
    db: &DatabaseConnection,
    id: &str,
    _user_id: &str,
) -> Result<document::Model, AppError> {
    let existing = DocumentService::get_document_model(db, id).await?;
    if existing.status != DocumentStatus::Draft.as_str() {
        return Err(AppError::Validation(
            "Only draft documents can be sent".to_string(),
        ));
    }

    let doc_number = DocumentRepo::next_doc_number(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let now = Utc::now().naive_utc();
    let today = now.date();

    let mut model: document::ActiveModel = existing.into();
    model.doc_number = Set(Some(doc_number));
    model.status = Set(DocumentStatus::Sent.to_string());
    model.issued_at = Set(Some(today));
    model.updated_at = Set(now);

    DocumentRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Accept/sign a document.
/// Quote/Offer → "accepted", SOW/Contract → "signed".
pub async fn accept_document(
    db: &DatabaseConnection,
    id: &str,
    _user_id: &str,
) -> Result<document::Model, AppError> {
    let existing = DocumentService::get_document_model(db, id).await?;
    if existing.status != DocumentStatus::Sent.as_str() {
        return Err(AppError::Validation(
            "Only sent documents can be accepted/signed".to_string(),
        ));
    }

    let new_status = match existing.doc_type.as_str() {
        "quote" | "offer" => "accepted",
        "sow" | "contract" => "signed",
        _ => "accepted",
    };

    let now = Utc::now().naive_utc();
    let today = now.date();

    let mut model: document::ActiveModel = existing.into();
    model.status = Set(new_status.to_string());
    model.signed_at = Set(Some(today));
    model.updated_at = Set(now);

    DocumentRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Reject a document (set status to "rejected").
pub async fn reject_document(
    db: &DatabaseConnection,
    id: &str,
    _user_id: &str,
) -> Result<document::Model, AppError> {
    let existing = DocumentService::get_document_model(db, id).await?;
    if existing.status != DocumentStatus::Sent.as_str() {
        return Err(AppError::Validation(
            "Only sent documents can be rejected".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: document::ActiveModel = existing.into();
    model.status = Set(DocumentStatus::Rejected.to_string());
    model.updated_at = Set(now);

    DocumentRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Convert a document to a new type (e.g. offer→sow, sow→contract).
/// Creates a new document of target_type with converted_from set.
pub async fn convert_document(
    db: &DatabaseConnection,
    id: &str,
    target_type: &str,
    user_id: &str,
) -> Result<document::Model, AppError> {
    let source = DocumentService::get_document_model(db, id).await?;

    // Validate conversion paths
    let valid = matches!(
        (source.doc_type.as_str(), target_type),
        ("offer", "sow") | ("sow", "contract") | ("quote", "sow")
    );
    if !valid {
        return Err(AppError::Validation(format!(
            "Cannot convert {} to {target_type}",
            source.doc_type
        )));
    }

    // Copy lines from source
    let source_lines = DocumentRepo::find_lines_by_document(db, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let now = Utc::now().naive_utc();
    let new_id = Uuid::new_v4().to_string();

    let model = document::ActiveModel {
        id: Set(new_id.clone()),
        doc_type: Set(target_type.to_string()),
        doc_number: Set(None),
        title: Set(source.title.clone()),
        status: Set(DocumentStatus::Draft.to_string()),
        contact_id: Set(source.contact_id.clone()),
        project_id: Set(source.project_id.clone()),
        template_id: Set(source.template_id.clone()),
        content_json: Set(source.content_json.clone()),
        language: Set(source.language.clone()),
        currency_id: Set(source.currency_id.clone()),
        subtotal: Set(source.subtotal),
        vat_rate: Set(source.vat_rate),
        vat_amount: Set(source.vat_amount),
        total: Set(source.total),
        valid_until: Set(None),
        issued_at: Set(None),
        signed_at: Set(None),
        converted_from: Set(Some(id.to_string())),
        created_by: Set(Some(user_id.to_string())),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let new_doc = DocumentRepo::create(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Copy line items
    for line in &source_lines {
        use konto_db::entities::document_line_item;
        let line_model = document_line_item::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            document_id: Set(new_id.clone()),
            position: Set(line.position),
            description: Set(line.description.clone()),
            quantity: Set(line.quantity),
            unit: Set(line.unit.clone()),
            unit_price: Set(line.unit_price),
            discount_pct: Set(line.discount_pct),
            total: Set(line.total),
            created_at: Set(now),
        };
        DocumentRepo::create_line(db, line_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    Ok(new_doc)
}
