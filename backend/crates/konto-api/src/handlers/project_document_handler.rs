use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_document_service::ProjectDocumentService;

use crate::dto::project_document::ProjectDocumentResponse;
use crate::middleware::upload::validate_document_upload;
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/projects/{project_id}/documents",
    responses((status = 200, body = Vec<ProjectDocumentResponse>))
)]
pub async fn list_project_files(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ProjectDocumentResponse>>, AppError> {
    let docs =
        ProjectDocumentService::list_for_project(&state.db, &project_id).await?;
    let data = docs.into_iter().map(ProjectDocumentResponse::from).collect();
    Ok(Json(data))
}

#[utoipa::path(
    post, path = "/api/v1/projects/{project_id}/documents",
    responses((status = 201, body = ProjectDocumentResponse))
)]
pub async fn upload_project_file(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(project_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ProjectDocumentResponse>, AppError> {
    let mut file_name = String::new();
    let mut mime_type = String::from("application/octet-stream");
    let mut data = Vec::new();
    let mut project_item_id: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid upload: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                file_name = field
                    .file_name()
                    .unwrap_or("document")
                    .to_string();
                mime_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::Validation(format!("Failed to read file: {e}")))?
                    .to_vec();
            }
            "project_item_id" => {
                let val = field
                    .text()
                    .await
                    .map_err(|e| AppError::Validation(format!("Invalid field: {e}")))?;
                if !val.is_empty() {
                    project_item_id = Some(val);
                }
            }
            _ => {}
        }
    }

    if data.is_empty() {
        return Err(AppError::Validation("No file provided".to_string()));
    }

    validate_document_upload(&file_name, &mime_type, &data)?;

    let doc = ProjectDocumentService::upload(
        &state.db,
        state.storage.as_ref(),
        &project_id,
        project_item_id.as_deref(),
        &file_name,
        &mime_type,
        &data,
        Some(&claims.sub),
    )
    .await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "project_document",
        Some(&doc.id),
        None,
        None,
    )
    .await?;

    Ok(Json(ProjectDocumentResponse::from(doc)))
}

#[utoipa::path(
    get, path = "/api/v1/project-documents/{id}/download",
    responses((status = 200))
)]
pub async fn download_project_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (doc, data) =
        ProjectDocumentService::download(&state.db, state.storage.as_ref(), &id).await?;

    let content_type = doc
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Ok((
        [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", doc.file_name),
            ),
        ],
        data,
    ))
}

#[utoipa::path(
    delete, path = "/api/v1/project-documents/{id}",
    responses((status = 204))
)]
pub async fn delete_project_file(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ProjectDocumentService::delete(&state.db, state.storage.as_ref(), &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "project_document",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}
