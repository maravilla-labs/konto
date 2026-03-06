use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::rate_function_service::RateFunctionService;

use crate::dto::rate_function::*;
use crate::state::AppState;

/// List all rate functions.
#[utoipa::path(
    get, path = "/api/v1/rate-functions",
    responses((status = 200, body = Vec<RateFunctionResponse>)),
    security(("bearer" = [])),
    tag = "rate-functions"
)]
pub async fn list_rate_functions(
    State(state): State<AppState>,
) -> Result<Json<Vec<RateFunctionResponse>>, AppError> {
    let functions = RateFunctionService::list(&state.db).await?;
    let data = functions.into_iter().map(RateFunctionResponse::from).collect();
    Ok(Json(data))
}

/// Create a new rate function.
#[utoipa::path(
    post, path = "/api/v1/rate-functions",
    request_body = CreateRateFunctionRequest,
    responses((status = 201, body = RateFunctionResponse)),
    security(("bearer" = [])),
    tag = "rate-functions"
)]
pub async fn create_rate_function(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateRateFunctionRequest>,
) -> Result<Json<RateFunctionResponse>, AppError> {
    let rf = RateFunctionService::create(
        &state.db,
        &body.name,
        body.description,
        body.hourly_rate,
        body.sort_order.unwrap_or(0),
    )
    .await?;

    let resp = RateFunctionResponse::from(rf.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "rate_function",
        Some(&rf.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update an existing rate function.
#[utoipa::path(
    put, path = "/api/v1/rate-functions/{id}",
    request_body = UpdateRateFunctionRequest,
    responses((status = 200, body = RateFunctionResponse)),
    security(("bearer" = [])),
    tag = "rate-functions"
)]
pub async fn update_rate_function(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateRateFunctionRequest>,
) -> Result<Json<RateFunctionResponse>, AppError> {
    let rf = RateFunctionService::update(
        &state.db,
        &id,
        &body.name,
        body.description,
        body.hourly_rate,
        body.is_active,
        body.sort_order,
    )
    .await?;

    let resp = RateFunctionResponse::from(rf);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "rate_function",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Delete a rate function.
#[utoipa::path(
    delete, path = "/api/v1/rate-functions/{id}",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "rate-functions"
)]
pub async fn delete_rate_function(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    RateFunctionService::delete(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "rate_function",
        Some(&id), None, None,
    ).await;

    Ok(())
}
