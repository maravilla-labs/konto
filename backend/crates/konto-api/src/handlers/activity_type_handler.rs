use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::activity_type_service::ActivityTypeService;
use konto_core::services::audit_service::AuditService;

use crate::dto::activity_type::*;
use crate::state::AppState;

/// List all activity types.
#[utoipa::path(
    get, path = "/api/v1/activity-types",
    responses((status = 200, body = Vec<ActivityTypeResponse>)),
    security(("bearer" = [])),
    tag = "activity-types"
)]
pub async fn list_activity_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<ActivityTypeResponse>>, AppError> {
    let types = ActivityTypeService::list(&state.db).await?;
    let data = types.into_iter().map(ActivityTypeResponse::from).collect();
    Ok(Json(data))
}

/// Create a new activity type.
#[utoipa::path(
    post, path = "/api/v1/activity-types",
    request_body = CreateActivityTypeRequest,
    responses((status = 201, body = ActivityTypeResponse)),
    security(("bearer" = [])),
    tag = "activity-types"
)]
pub async fn create_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateActivityTypeRequest>,
) -> Result<Json<ActivityTypeResponse>, AppError> {
    let at = ActivityTypeService::create(&state.db, &body.name, body.unit_type, body.default_rate).await?;

    let resp = ActivityTypeResponse::from(at.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "activity_type",
        Some(&at.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update an existing activity type.
#[utoipa::path(
    put, path = "/api/v1/activity-types/{id}",
    request_body = UpdateActivityTypeRequest,
    responses((status = 200, body = ActivityTypeResponse)),
    security(("bearer" = [])),
    tag = "activity-types"
)]
pub async fn update_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateActivityTypeRequest>,
) -> Result<Json<ActivityTypeResponse>, AppError> {
    let at = ActivityTypeService::update(&state.db, &id, &body.name, body.is_active, body.unit_type, body.default_rate).await?;

    let resp = ActivityTypeResponse::from(at);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "activity_type",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Delete an activity type.
#[utoipa::path(
    delete, path = "/api/v1/activity-types/{id}",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "activity-types"
)]
pub async fn delete_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ActivityTypeService::delete(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "activity_type",
        Some(&id), None, None,
    ).await;

    Ok(())
}
