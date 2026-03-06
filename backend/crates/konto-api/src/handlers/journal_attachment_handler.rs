use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::journal_attachment_service::JournalAttachmentService;

use crate::dto::journal_attachment::JournalAttachmentResponse;
use crate::middleware::upload::validate_document_upload;
use crate::state::AppState;

pub async fn list_attachments(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
) -> Result<Json<Vec<JournalAttachmentResponse>>, AppError> {
    let attachments =
        JournalAttachmentService::list_by_entry(&state.db, &entry_id).await?;
    let data = attachments.into_iter().map(JournalAttachmentResponse::from).collect();
    Ok(Json(data))
}

pub async fn upload_attachment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(entry_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<JournalAttachmentResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid upload: {e}")))?
        .ok_or_else(|| AppError::Validation("No file provided".to_string()))?;

    let file_name = field
        .file_name()
        .unwrap_or("attachment")
        .to_string();
    let mime_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();
    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::Validation(format!("Failed to read file: {e}")))?;

    validate_document_upload(&file_name, &mime_type, &data)?;

    let att = JournalAttachmentService::upload(
        &state.db,
        state.storage.as_ref(),
        &entry_id,
        &file_name,
        &data,
        &mime_type,
        Some(&claims.sub),
    )
    .await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "journal_attachment",
        Some(&att.id),
        None,
        None,
    )
    .await?;

    Ok(Json(JournalAttachmentResponse::from(att)))
}

pub async fn download_attachment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (att, data) =
        JournalAttachmentService::download(&state.db, state.storage.as_ref(), &id).await?;

    Ok((
        [
            (header::CONTENT_TYPE, att.mime_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", att.file_name),
            ),
        ],
        data,
    ))
}

pub async fn preview_attachment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (att, data) =
        JournalAttachmentService::download(&state.db, state.storage.as_ref(), &id).await?;

    Ok((
        [
            (header::CONTENT_TYPE, att.mime_type),
            (
                header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{}\"", att.file_name),
            ),
        ],
        data,
    ))
}

pub async fn delete_attachment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    JournalAttachmentService::delete(&state.db, state.storage.as_ref(), &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "journal_attachment",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}
