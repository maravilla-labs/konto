use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_member_service::ProjectMemberService;
use konto_core::services::rate_function_service::RateFunctionService;
use konto_db::repository::user_repo::UserRepo;

use crate::dto::project_member::*;
use crate::state::AppState;

/// List all members of a project.
#[utoipa::path(
    get, path = "/api/v1/projects/{id}/members",
    responses((status = 200, body = Vec<ProjectMemberResponse>)),
    security(("bearer" = [])),
    tag = "project-members"
)]
pub async fn list_project_members(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ProjectMemberResponse>>, AppError> {
    let members = ProjectMemberService::list_for_project(&state.db, &project_id).await?;

    let mut responses = Vec::with_capacity(members.len());
    for m in members {
        let user_name = UserRepo::find_by_id(&state.db, &m.user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map(|u| u.full_name);

        let rate_function_name = if let Some(ref rf_id) = m.rate_function_id {
            RateFunctionService::get_by_id(&state.db, rf_id)
                .await
                .ok()
                .map(|rf| rf.name)
        } else {
            None
        };

        let resolved_rate = ProjectMemberService::resolve_rate(
            &state.db, &m.project_id, &m.user_id,
        ).await?;

        responses.push(ProjectMemberResponse {
            id: m.id,
            project_id: m.project_id,
            user_id: m.user_id,
            user_name,
            rate_function_id: m.rate_function_id,
            rate_function_name,
            hourly_rate: m.hourly_rate,
            resolved_rate,
            role_label: m.role_label,
            budget_hours: m.budget_hours,
            joined_at: m.joined_at.to_string(),
            left_at: m.left_at.map(|d| d.to_string()),
        });
    }

    Ok(Json(responses))
}

/// Add a member to a project.
#[utoipa::path(
    post, path = "/api/v1/projects/{id}/members",
    request_body = CreateProjectMemberRequest,
    responses((status = 201, body = ProjectMemberResponse)),
    security(("bearer" = [])),
    tag = "project-members"
)]
pub async fn add_project_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateProjectMemberRequest>,
) -> Result<Json<ProjectMemberResponse>, AppError> {
    let member = ProjectMemberService::add_member(
        &state.db,
        &project_id,
        &body.user_id,
        body.rate_function_id,
        body.hourly_rate,
        body.role_label,
        body.budget_hours,
    )
    .await?;

    let user_name = UserRepo::find_by_id(&state.db, &member.user_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .map(|u| u.full_name);

    let rate_function_name = if let Some(ref rf_id) = member.rate_function_id {
        RateFunctionService::get_by_id(&state.db, rf_id)
            .await
            .ok()
            .map(|rf| rf.name)
    } else {
        None
    };

    let resolved_rate = ProjectMemberService::resolve_rate(
        &state.db, &member.project_id, &member.user_id,
    ).await?;

    let resp = ProjectMemberResponse {
        id: member.id.clone(),
        project_id: member.project_id,
        user_id: member.user_id,
        user_name,
        rate_function_id: member.rate_function_id,
        rate_function_name,
        hourly_rate: member.hourly_rate,
        resolved_rate,
        role_label: member.role_label,
        budget_hours: member.budget_hours,
        joined_at: member.joined_at.to_string(),
        left_at: member.left_at.map(|d| d.to_string()),
    };

    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "project_member",
        Some(&member.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update a project member.
#[utoipa::path(
    put, path = "/api/v1/projects/{id}/members/{member_id}",
    request_body = UpdateProjectMemberRequest,
    responses((status = 200, body = ProjectMemberResponse)),
    security(("bearer" = [])),
    tag = "project-members"
)]
pub async fn update_project_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_project_id, member_id)): Path<(String, String)>,
    Json(body): Json<UpdateProjectMemberRequest>,
) -> Result<Json<ProjectMemberResponse>, AppError> {
    let member = ProjectMemberService::update_member(
        &state.db,
        &member_id,
        body.rate_function_id,
        body.hourly_rate,
        body.role_label,
        body.budget_hours,
    )
    .await?;

    let user_name = UserRepo::find_by_id(&state.db, &member.user_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .map(|u| u.full_name);

    let rate_function_name = if let Some(ref rf_id) = member.rate_function_id {
        RateFunctionService::get_by_id(&state.db, rf_id)
            .await
            .ok()
            .map(|rf| rf.name)
    } else {
        None
    };

    let resolved_rate = ProjectMemberService::resolve_rate(
        &state.db, &member.project_id, &member.user_id,
    ).await?;

    let resp = ProjectMemberResponse {
        id: member.id.clone(),
        project_id: member.project_id,
        user_id: member.user_id,
        user_name,
        rate_function_id: member.rate_function_id,
        rate_function_name,
        hourly_rate: member.hourly_rate,
        resolved_rate,
        role_label: member.role_label,
        budget_hours: member.budget_hours,
        joined_at: member.joined_at.to_string(),
        left_at: member.left_at.map(|d| d.to_string()),
    };

    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "project_member",
        Some(&member_id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Remove a member from a project.
#[utoipa::path(
    delete, path = "/api/v1/projects/{id}/members/{member_id}",
    responses((status = 204)),
    security(("bearer" = [])),
    tag = "project-members"
)]
pub async fn remove_project_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_project_id, member_id)): Path<(String, String)>,
) -> Result<(), AppError> {
    ProjectMemberService::remove_member(&state.db, &member_id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "project_member",
        Some(&member_id), None, None,
    ).await;

    Ok(())
}
