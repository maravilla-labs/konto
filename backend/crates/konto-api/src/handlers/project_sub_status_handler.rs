use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_sub_status_service::ProjectSubStatusService;

use crate::dto::project_sub_status::{
    CreateProjectSubStatusRequest, ProjectSubStatusResponse, UpdateProjectSubStatusRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/project-sub-statuses",
    responses((status = 200, body = Vec<ProjectSubStatusResponse>)),
    security(("bearer" = [])),
    tag = "project-sub-statuses"
)]
pub async fn list_project_sub_statuses(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectSubStatusResponse>>, AppError> {
    let items = ProjectSubStatusService::list(&state.db).await?;
    Ok(Json(items.into_iter().map(ProjectSubStatusResponse::from).collect()))
}

#[utoipa::path(
    post, path = "/api/v1/project-sub-statuses",
    request_body = CreateProjectSubStatusRequest,
    responses((status = 201, body = ProjectSubStatusResponse)),
    security(("bearer" = [])),
    tag = "project-sub-statuses"
)]
pub async fn create_project_sub_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateProjectSubStatusRequest>,
) -> Result<Json<ProjectSubStatusResponse>, AppError> {
    let item = ProjectSubStatusService::create(
        &state.db, &body.name, body.sort_order, body.color,
    ).await?;

    let resp = ProjectSubStatusResponse::from(item.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "project_sub_status",
        Some(&item.id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/project-sub-statuses/{id}",
    request_body = UpdateProjectSubStatusRequest,
    responses((status = 200, body = ProjectSubStatusResponse)),
    security(("bearer" = [])),
    tag = "project-sub-statuses"
)]
pub async fn update_project_sub_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProjectSubStatusRequest>,
) -> Result<Json<ProjectSubStatusResponse>, AppError> {
    let item = ProjectSubStatusService::update(
        &state.db, &id, body.name, body.sort_order, body.color, body.is_active,
    ).await?;

    let resp = ProjectSubStatusResponse::from(item.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "project_sub_status",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/project-sub-statuses/{id}",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "project-sub-statuses"
)]
pub async fn delete_project_sub_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ProjectSubStatusService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "project_sub_status",
        Some(&id), None, None,
    ).await?;

    Ok(())
}
