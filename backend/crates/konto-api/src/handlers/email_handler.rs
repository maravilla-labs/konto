use axum::extract::State;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::UserRole;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::email_service::EmailService;

use crate::dto::email::{EmailSettingsResponse, TestEmailRequest, UpdateEmailSettingsRequest};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/settings/email",
    responses((status = 200, body = EmailSettingsResponse)),
    security(("bearer" = []))
)]
pub async fn get_email_settings(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let settings = EmailService::get_settings(&state.db).await?;
    match settings {
        Some(s) => Ok(Json(serde_json::to_value(EmailSettingsResponse::from(s))
            .map_err(|e| AppError::Internal(format!("Failed to serialize email settings: {e}")))?)),
        None => Ok(Json(serde_json::json!(null))),
    }
}

#[utoipa::path(
    put, path = "/api/v1/settings/email",
    request_body = UpdateEmailSettingsRequest,
    responses((status = 200, body = EmailSettingsResponse)),
    security(("bearer" = []))
)]
pub async fn update_email_settings(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateEmailSettingsRequest>,
) -> Result<Json<EmailSettingsResponse>, AppError> {
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let settings = EmailService::update_settings(
        &state.db,
        &body.smtp_host,
        body.smtp_port,
        &body.smtp_username,
        body.smtp_password.as_deref(),
        &body.smtp_encryption,
        &body.from_email,
        &body.from_name,
        body.reply_to_email,
        body.bcc_email,
        body.is_active,
    )
    .await?;

    let resp = EmailSettingsResponse::from(settings.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "email_settings",
        Some(&settings.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/settings/email/test",
    request_body = TestEmailRequest,
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn send_test_email(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<TestEmailRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    if let Some(to) = body.to_email {
        EmailService::send_email(
            &state.db,
            &to,
            "Hope Test Email",
            "This is a test email from Hope Accounting.",
            vec![],
        )
        .await?;
    } else {
        EmailService::send_test_email(&state.db).await?;
    }

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "test_email",
        "email_settings",
        None,
        None,
        None,
    )
    .await?;

    Ok(Json(serde_json::json!({"message": "Test email sent"})))
}
