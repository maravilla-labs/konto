use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::expense_receipt_service::ExpenseReceiptService;
use serde::Serialize;
use utoipa::ToSchema;

use crate::middleware::upload::validate_document_upload;
use crate::state::AppState;

#[derive(Debug, Serialize, ToSchema)]
pub struct ExpenseReceiptResponse {
    pub id: String,
    pub expense_id: String,
    pub line_id: Option<String>,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_at: String,
}

impl From<konto_db::entities::expense_receipt::Model> for ExpenseReceiptResponse {
    fn from(m: konto_db::entities::expense_receipt::Model) -> Self {
        Self {
            id: m.id,
            expense_id: m.expense_id,
            line_id: m.line_id,
            file_name: m.file_name,
            file_size: m.file_size,
            mime_type: m.mime_type,
            uploaded_at: m.uploaded_at.to_rfc3339(),
        }
    }
}

pub async fn list_receipts(
    State(state): State<AppState>,
    Path(expense_id): Path<String>,
) -> Result<Json<Vec<ExpenseReceiptResponse>>, AppError> {
    let receipts = ExpenseReceiptService::list_by_expense(&state.db, &expense_id).await?;
    let data = receipts.into_iter().map(ExpenseReceiptResponse::from).collect();
    Ok(Json(data))
}

pub async fn upload_receipt(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(expense_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ExpenseReceiptResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid upload: {e}")))?
        .ok_or_else(|| AppError::Validation("No file provided".to_string()))?;

    let file_name = field.file_name().unwrap_or("receipt").to_string();
    let mime_type = field.content_type().unwrap_or("application/octet-stream").to_string();
    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::Validation(format!("Failed to read file: {e}")))?;

    validate_document_upload(&file_name, &mime_type, &data)?;

    let receipt = ExpenseReceiptService::upload(
        &state.db,
        state.storage.as_ref(),
        &expense_id,
        &file_name,
        &data,
        &mime_type,
    )
    .await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "create", "expense_receipt",
        Some(&receipt.id), None, None,
    )
    .await?;

    Ok(Json(ExpenseReceiptResponse::from(receipt)))
}

pub async fn download_receipt(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (receipt, data) =
        ExpenseReceiptService::download(&state.db, state.storage.as_ref(), &id).await?;

    Ok((
        [
            (header::CONTENT_TYPE, receipt.mime_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", receipt.file_name),
            ),
        ],
        data,
    ))
}

pub async fn delete_receipt(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ExpenseReceiptService::delete(&state.db, state.storage.as_ref(), &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "expense_receipt",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}
