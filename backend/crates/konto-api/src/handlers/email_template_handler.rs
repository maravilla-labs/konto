use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::UserRole;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::email_template_service::EmailTemplateService;

use crate::dto::email_template::{
    EmailTemplatePreviewResponse, EmailTemplateResponse, UpdateEmailTemplateRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/email-templates",
    responses((status = 200, body = Vec<EmailTemplateResponse>)),
    security(("bearer" = []))
)]
pub async fn list_email_templates(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmailTemplateResponse>>, AppError> {
    let templates = EmailTemplateService::list(&state.db).await?;
    let data = templates
        .into_iter()
        .map(EmailTemplateResponse::from)
        .collect();
    Ok(Json(data))
}

#[utoipa::path(
    get, path = "/api/v1/email-templates/{id}",
    responses((status = 200, body = EmailTemplateResponse)),
    security(("bearer" = []))
)]
pub async fn get_email_template(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EmailTemplateResponse>, AppError> {
    let tmpl = EmailTemplateService::get(&state.db, &id).await?;
    Ok(Json(EmailTemplateResponse::from(tmpl)))
}

#[utoipa::path(
    put, path = "/api/v1/email-templates/{id}",
    request_body = UpdateEmailTemplateRequest,
    responses((status = 200, body = EmailTemplateResponse)),
    security(("bearer" = []))
)]
pub async fn update_email_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateEmailTemplateRequest>,
) -> Result<Json<EmailTemplateResponse>, AppError> {
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    let old = EmailTemplateService::get(&state.db, &id).await?;
    let old_vals = serde_json::to_string(&EmailTemplateResponse::from(old)).ok();

    let updated =
        EmailTemplateService::update(&state.db, &id, &body.subject, &body.body_html).await?;

    let resp = EmailTemplateResponse::from(updated);
    let new_vals = serde_json::to_string(&resp).ok();

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "email_template",
        Some(&id),
        old_vals.as_deref(),
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/email-templates/{id}/preview",
    responses((status = 200, body = EmailTemplatePreviewResponse)),
    security(("bearer" = []))
)]
pub async fn preview_email_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<EmailTemplatePreviewResponse>, AppError> {
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    let (rendered_subject, rendered_body_html) =
        EmailTemplateService::preview(&state.db, &id).await?;

    Ok(Json(EmailTemplatePreviewResponse {
        rendered_subject,
        rendered_body_html,
    }))
}
