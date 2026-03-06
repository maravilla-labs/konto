use axum::extract::{Multipart, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::settings_service::SettingsService;
use std::path::Path;
use tokio::fs;

use crate::dto::settings::{CompanySettingsResponse, UpdateCompanySettingsRequest};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/settings",
    responses((status = 200, body = CompanySettingsResponse)),
    security(("bearer" = []))
)]
pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<CompanySettingsResponse>, AppError> {
    let settings = SettingsService::get_or_create(&state.db).await?;
    Ok(Json(CompanySettingsResponse::from(settings)))
}

#[utoipa::path(
    put, path = "/api/v1/settings",
    request_body = UpdateCompanySettingsRequest,
    responses((status = 200, body = CompanySettingsResponse)),
    security(("bearer" = []))
)]
pub async fn update_settings(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateCompanySettingsRequest>,
) -> Result<Json<CompanySettingsResponse>, AppError> {
    let settings = SettingsService::update(
        &state.db,
        &body.legal_name,
        body.trade_name,
        &body.street,
        &body.postal_code,
        &body.city,
        &body.country,
        body.email,
        body.phone,
        body.website,
        body.vat_number,
        &body.vat_method,
        body.flat_rate_percentage,
        body.register_number,
        body.default_currency_id,
        body.date_format,
        body.number_format,
        body.ui_language,
        body.fiscal_year_start_month,
        body.tax_id_label,
        body.audit_optout,
        body.project_number_auto,
        body.project_number_prefix,
        body.project_number_restart_yearly,
        body.project_number_start,
        body.project_number_min_length,
        body.customer_number_auto,
        body.customer_number_prefix,
        body.customer_number_restart_yearly,
        body.customer_number_start,
        body.customer_number_min_length,
        body.employee_number_auto,
        body.employee_number_prefix,
        body.employee_number_restart_yearly,
        body.employee_number_start,
        body.employee_number_min_length,
    )
    .await?;

    let resp = CompanySettingsResponse::from(settings.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "company_settings",
        Some(&settings.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/settings/logo",
    responses((status = 200, body = CompanySettingsResponse)),
    security(("bearer" = []))
)]
pub async fn upload_logo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<CompanySettingsResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart: {e}")))?
        .ok_or_else(|| AppError::BadRequest("No file uploaded".to_string()))?;

    let file_name = field
        .file_name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "logo.png".to_string());

    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read file data: {e}")))?;

    // Ensure uploads directory exists
    let upload_dir = Path::new("uploads");
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create uploads dir: {e}")))?;

    let ext = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let stored_name = format!("logo.{ext}");
    let file_path = upload_dir.join(&stored_name);

    fs::write(&file_path, &data)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to save file: {e}")))?;

    let logo_url = format!("/uploads/{stored_name}");
    let settings = SettingsService::update_logo_url(&state.db, Some(logo_url)).await?;

    let resp = CompanySettingsResponse::from(settings.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "upload_logo", "company_settings",
        Some(&settings.id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}
