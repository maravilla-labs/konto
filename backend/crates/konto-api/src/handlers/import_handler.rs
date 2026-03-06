use axum::extract::{Multipart, Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::import::import_service::ImportService;
use konto_core::services::audit_service::AuditService;

use crate::dto::import::ImportJobResponse;
use crate::state::AppState;

#[utoipa::path(
    post, path = "/api/v1/import/upload",
    responses((status = 201, body = ImportJobResponse))
)]
pub async fn upload_import(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<ImportJobResponse>, AppError> {
    let mut import_type: Option<String> = None;
    let mut file_name = String::from("upload");
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "import_type" => {
                import_type = Some(
                    field.text().await
                        .map_err(|e| AppError::BadRequest(format!("Failed to read import_type: {e}")))?,
                );
            }
            "file" => {
                if let Some(fname) = field.file_name() {
                    file_name = fname.to_string();
                }
                file_data = Some(
                    field.bytes().await
                        .map_err(|e| AppError::BadRequest(format!("Failed to read file: {e}")))?
                        .to_vec(),
                );
            }
            _ => {}
        }
    }

    let import_type = import_type
        .ok_or_else(|| AppError::BadRequest("Missing import_type field".to_string()))?;
    let file_data = file_data
        .ok_or_else(|| AppError::BadRequest("No file uploaded".to_string()))?;

    let job = ImportService::upload(
        &state.db,
        &import_type,
        &file_name,
        file_data,
        Some(claims.sub),
    )
    .await?;

    Ok(Json(ImportJobResponse::from(job)))
}

#[utoipa::path(
    post, path = "/api/v1/import/{id}/preview",
    responses((status = 200))
)]
pub async fn preview_import(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let preview = ImportService::preview(&state.db, &id).await?;
    Ok(Json(preview))
}

#[utoipa::path(
    post, path = "/api/v1/import/{id}/execute",
    responses((status = 200, body = ImportJobResponse))
)]
pub async fn execute_import(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<ImportJobResponse>, AppError> {
    let job = ImportService::execute(&state.db, &id).await?;

    let resp = ImportJobResponse::from(job.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "execute", "import",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}
