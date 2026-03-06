use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_item_service::ProjectItemService;

use crate::dto::project_item::{
    CreateProjectItemRequest, ProjectItemResponse, ProjectItemTreeResponse,
    ReorderProjectItemRequest, UpdateProjectItemRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/projects/{project_id}/items",
    responses((status = 200, body = Vec<ProjectItemTreeResponse>)),
    security(("bearer" = []))
)]
pub async fn list_project_items(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ProjectItemTreeResponse>>, AppError> {
    let tree = ProjectItemService::list_tree(&state.db, &project_id).await?;
    let response: Vec<ProjectItemTreeResponse> =
        tree.into_iter().map(ProjectItemTreeResponse::from).collect();
    Ok(Json(response))
}

#[utoipa::path(
    post, path = "/api/v1/projects/{project_id}/items",
    request_body = CreateProjectItemRequest,
    responses((status = 201, body = ProjectItemResponse)),
    security(("bearer" = []))
)]
pub async fn create_project_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateProjectItemRequest>,
) -> Result<Json<ProjectItemResponse>, AppError> {
    let start = body
        .start_date
        .as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    let due = body
        .due_date
        .as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    let item = ProjectItemService::create(
        &state.db,
        &project_id,
        body.parent_id,
        &body.item_type,
        &body.name,
        body.description,
        body.assignee_id,
        start,
        due,
        body.estimated_hours,
        body.budget_hours,
        body.budget_amount,
        body.sort_order,
        &claims.sub,
    )
    .await?;

    let resp = ProjectItemResponse::from(item.clone());
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "project_item",
        Some(&item.id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/project-items/{id}",
    responses((status = 200, body = ProjectItemResponse)),
    security(("bearer" = []))
)]
pub async fn get_project_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectItemResponse>, AppError> {
    let item = ProjectItemService::get_by_id(&state.db, &id).await?;
    Ok(Json(ProjectItemResponse::from(item)))
}

#[utoipa::path(
    put, path = "/api/v1/project-items/{id}",
    request_body = UpdateProjectItemRequest,
    responses((status = 200, body = ProjectItemResponse)),
    security(("bearer" = []))
)]
pub async fn update_project_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProjectItemRequest>,
) -> Result<Json<ProjectItemResponse>, AppError> {
    let start = body.start_date.map(|opt| {
        opt.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
    });
    let due = body.due_date.map(|opt| {
        opt.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
    });

    let item = ProjectItemService::update(
        &state.db,
        &id,
        body.name,
        body.description,
        body.status,
        body.assignee_id,
        start,
        due,
        body.estimated_hours,
        body.budget_hours,
        body.budget_amount,
        body.sort_order,
        &claims.sub,
    )
    .await?;

    let resp = ProjectItemResponse::from(item);
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "project_item",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/project-items/{id}",
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn delete_project_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    ProjectItemService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "project_item",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(serde_json::json!({"deleted": true})))
}

#[utoipa::path(
    post, path = "/api/v1/project-items/{id}/reorder",
    request_body = ReorderProjectItemRequest,
    responses((status = 200, body = ProjectItemResponse)),
    security(("bearer" = []))
)]
pub async fn reorder_project_item(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<ReorderProjectItemRequest>,
) -> Result<Json<ProjectItemResponse>, AppError> {
    let item = ProjectItemService::reorder(
        &state.db,
        &id,
        body.new_parent_id,
        body.new_sort_order,
        &claims.sub,
    )
    .await?;

    let resp = ProjectItemResponse::from(item);
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "reorder",
        "project_item",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}
