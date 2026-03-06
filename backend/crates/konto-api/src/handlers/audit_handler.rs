use axum::extract::{Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::UserRole;
use konto_common::pagination::PaginatedResponse;
use konto_core::auth::jwt::Claims;
use konto_db::repository::audit_repo::AuditRepo;

use crate::dto::audit::{AuditLogParams, AuditLogResponse};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/audit-log",
    params(AuditLogParams),
    responses((status = 200, body = Vec<AuditLogResponse>)),
    security(("bearer" = []))
)]
pub async fn list_audit_logs(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<AuditLogParams>,
) -> Result<Json<PaginatedResponse<AuditLogResponse>>, AppError> {
    if claims.role != UserRole::Admin && claims.role != UserRole::Auditor {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let from_date = params
        .from
        .as_deref()
        .and_then(|s| chrono::NaiveDateTime::parse_from_str(
            &format!("{s} 00:00:00"),
            "%Y-%m-%d %H:%M:%S",
        ).ok());

    let to_date = params
        .to
        .as_deref()
        .and_then(|s| chrono::NaiveDateTime::parse_from_str(
            &format!("{s} 23:59:59"),
            "%Y-%m-%d %H:%M:%S",
        ).ok());

    let (logs, total) = AuditRepo::find_filtered(
        &state.db,
        params.page(),
        params.per_page(),
        params.entity_type.as_deref(),
        params.action.as_deref(),
        params.user_id.as_deref(),
        from_date,
        to_date,
    )
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let data = logs.into_iter().map(AuditLogResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}
