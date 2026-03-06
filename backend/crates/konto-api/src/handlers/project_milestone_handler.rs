use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_milestone_service::ProjectMilestoneService;

use crate::dto::project_milestone::{
    CreateProjectMilestoneRequest, ProjectMilestoneResponse,
    UpdateProjectMilestoneRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/projects/{project_id}/milestones",
    responses((status = 200, body = Vec<ProjectMilestoneResponse>)),
    security(("bearer" = []))
)]
pub async fn list_project_milestones(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ProjectMilestoneResponse>>, AppError> {
    let milestones =
        ProjectMilestoneService::list_for_project(&state.db, &project_id).await?;
    let response: Vec<ProjectMilestoneResponse> = milestones
        .into_iter()
        .map(ProjectMilestoneResponse::from)
        .collect();
    Ok(Json(response))
}

#[utoipa::path(
    post, path = "/api/v1/projects/{project_id}/milestones",
    request_body = CreateProjectMilestoneRequest,
    responses((status = 201, body = ProjectMilestoneResponse)),
    security(("bearer" = []))
)]
pub async fn create_project_milestone(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateProjectMilestoneRequest>,
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    let target_date =
        chrono::NaiveDate::parse_from_str(&body.target_date, "%Y-%m-%d")
            .map_err(|_| AppError::Validation("Invalid target_date format".into()))?;

    let milestone = ProjectMilestoneService::create(
        &state.db,
        &project_id,
        body.project_item_id,
        &body.name,
        body.description,
        target_date,
    )
    .await?;

    let resp = ProjectMilestoneResponse::from(milestone.clone());
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "project_milestone",
        Some(&milestone.id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/project-milestones/{id}",
    request_body = UpdateProjectMilestoneRequest,
    responses((status = 200, body = ProjectMilestoneResponse)),
    security(("bearer" = []))
)]
pub async fn update_project_milestone(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProjectMilestoneRequest>,
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    let target_date = body
        .target_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid target_date format".into()))
        })
        .transpose()?;

    let milestone = ProjectMilestoneService::update(
        &state.db,
        &id,
        body.name,
        body.description,
        target_date,
        body.project_item_id,
    )
    .await?;

    let resp = ProjectMilestoneResponse::from(milestone);
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "project_milestone",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/project-milestones/{id}/reach",
    responses((status = 200, body = ProjectMilestoneResponse)),
    security(("bearer" = []))
)]
pub async fn reach_project_milestone(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    let milestone = ProjectMilestoneService::reach(&state.db, &id).await?;

    let resp = ProjectMilestoneResponse::from(milestone);
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "reach",
        "project_milestone",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/project-milestones/{id}",
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn delete_project_milestone(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    ProjectMilestoneService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "project_milestone",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(serde_json::json!({"deleted": true})))
}
