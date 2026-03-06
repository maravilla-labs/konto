use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::activity_type_service::ActivityTypeService;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_activity_type_service::ProjectActivityTypeService;

use crate::dto::project_activity_type::*;
use crate::state::AppState;

/// List all activity types assigned to a project.
#[utoipa::path(
    get, path = "/api/v1/projects/{id}/activity-types",
    responses((status = 200, body = Vec<ProjectActivityTypeResponse>)),
    security(("bearer" = [])),
    tag = "project-activity-types"
)]
pub async fn list_project_activity_types(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ProjectActivityTypeResponse>>, AppError> {
    let pats = ProjectActivityTypeService::list_for_project(&state.db, &project_id).await?;

    let mut responses = Vec::with_capacity(pats.len());
    for pat in pats {
        let at = ActivityTypeService::get_by_id(&state.db, &pat.activity_type_id).await.ok();

        let activity_type_name = at.as_ref().map(|a| a.name.clone());
        let unit_type = at.as_ref().map(|a| a.unit_type.clone());
        let default_rate = at.as_ref().and_then(|a| a.default_rate);
        let effective_rate = pat.rate.or(default_rate);

        responses.push(ProjectActivityTypeResponse {
            id: pat.id,
            project_id: pat.project_id,
            activity_type_id: pat.activity_type_id,
            activity_type_name,
            unit_type,
            default_rate,
            rate: pat.rate,
            effective_rate,
            budget_hours: pat.budget_hours,
            chargeable: pat.chargeable,
            created_at: pat.created_at.to_string(),
            updated_at: pat.updated_at.to_string(),
        });
    }

    Ok(Json(responses))
}

/// Add an activity type to a project.
#[utoipa::path(
    post, path = "/api/v1/projects/{id}/activity-types",
    request_body = CreateProjectActivityTypeRequest,
    responses((status = 201, body = ProjectActivityTypeResponse)),
    security(("bearer" = [])),
    tag = "project-activity-types"
)]
pub async fn add_project_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateProjectActivityTypeRequest>,
) -> Result<Json<ProjectActivityTypeResponse>, AppError> {
    let pat = ProjectActivityTypeService::add(
        &state.db,
        &project_id,
        &body.activity_type_id,
        body.rate,
        body.budget_hours,
        body.chargeable,
    )
    .await?;

    let at = ActivityTypeService::get_by_id(&state.db, &pat.activity_type_id).await.ok();
    let activity_type_name = at.as_ref().map(|a| a.name.clone());
    let unit_type = at.as_ref().map(|a| a.unit_type.clone());
    let default_rate = at.as_ref().and_then(|a| a.default_rate);
    let effective_rate = pat.rate.or(default_rate);

    let resp = ProjectActivityTypeResponse {
        id: pat.id.clone(),
        project_id: pat.project_id,
        activity_type_id: pat.activity_type_id,
        activity_type_name,
        unit_type,
        default_rate,
        rate: pat.rate,
        effective_rate,
        budget_hours: pat.budget_hours,
        chargeable: pat.chargeable,
        created_at: pat.created_at.to_string(),
        updated_at: pat.updated_at.to_string(),
    };

    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "project_activity_type",
        Some(&pat.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update a project activity type (rate override).
#[utoipa::path(
    put, path = "/api/v1/projects/{id}/activity-types/{pat_id}",
    request_body = UpdateProjectActivityTypeRequest,
    responses((status = 200, body = ProjectActivityTypeResponse)),
    security(("bearer" = [])),
    tag = "project-activity-types"
)]
pub async fn update_project_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_project_id, pat_id)): Path<(String, String)>,
    Json(body): Json<UpdateProjectActivityTypeRequest>,
) -> Result<Json<ProjectActivityTypeResponse>, AppError> {
    let pat = ProjectActivityTypeService::update(&state.db, &pat_id, body.rate, body.budget_hours, body.chargeable).await?;

    let at = ActivityTypeService::get_by_id(&state.db, &pat.activity_type_id).await.ok();
    let activity_type_name = at.as_ref().map(|a| a.name.clone());
    let unit_type = at.as_ref().map(|a| a.unit_type.clone());
    let default_rate = at.as_ref().and_then(|a| a.default_rate);
    let effective_rate = pat.rate.or(default_rate);

    let resp = ProjectActivityTypeResponse {
        id: pat.id.clone(),
        project_id: pat.project_id,
        activity_type_id: pat.activity_type_id,
        activity_type_name,
        unit_type,
        default_rate,
        rate: pat.rate,
        effective_rate,
        budget_hours: pat.budget_hours,
        chargeable: pat.chargeable,
        created_at: pat.created_at.to_string(),
        updated_at: pat.updated_at.to_string(),
    };

    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "project_activity_type",
        Some(&pat_id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Remove an activity type from a project.
#[utoipa::path(
    delete, path = "/api/v1/projects/{id}/activity-types/{pat_id}",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "project-activity-types"
)]
pub async fn remove_project_activity_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_project_id, pat_id)): Path<(String, String)>,
) -> Result<(), AppError> {
    ProjectActivityTypeService::remove(&state.db, &pat_id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "project_activity_type",
        Some(&pat_id), None, None,
    ).await;

    Ok(())
}
