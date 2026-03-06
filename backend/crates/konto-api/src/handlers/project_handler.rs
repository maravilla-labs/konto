use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::project_service::ProjectService;

use konto_core::services::project_budget_service::ProjectBudgetService;

use crate::dto::project::{CreateProjectRequest, ProjectResponse, ProjectSummaryResponse, UpdateProjectRequest};
use crate::dto::project_budget::*;
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/projects",
    params(PaginationParams),
    responses((status = 200, body = Vec<ProjectResponse>))
)]
pub async fn list_projects(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ProjectResponse>>, AppError> {
    let (projects, total) = ProjectService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.search.as_deref(),
    )
    .await?;

    let data = projects.into_iter().map(ProjectResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    post, path = "/api/v1/projects",
    request_body = CreateProjectRequest,
    responses((status = 201, body = ProjectResponse))
)]
pub async fn create_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, AppError> {
    let start = body.start_date.as_deref().and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    let end = body.end_date.as_deref().and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    let project = ProjectService::create(
        &state.db,
        &body.name,
        body.number,
        body.contact_id,
        body.language,
        start,
        end,
        body.description,
        body.budget_hours,
        body.budget_amount,
        body.hourly_rate,
        body.soft_budget_hours,
        body.hard_budget_hours,
        body.soft_budget_amount,
        body.hard_budget_amount,
        body.contact_person_id,
        body.invoicing_method,
        body.currency,
        body.rounding_method,
        body.rounding_factor_minutes,
        body.flat_rate_total,
        body.owner_id,
    )
    .await?;

    let resp = ProjectResponse::from(project.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "project",
        Some(&project.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/projects/{id}",
    responses((status = 200, body = ProjectResponse))
)]
pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, AppError> {
    let project = ProjectService::get_by_id(&state.db, &id).await?;
    Ok(Json(ProjectResponse::from(project)))
}

#[utoipa::path(
    put, path = "/api/v1/projects/{id}",
    request_body = UpdateProjectRequest,
    responses((status = 200, body = ProjectResponse))
)]
pub async fn update_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, AppError> {
    let start = body.start_date.map(|opt| {
        opt.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
    });
    let end = body.end_date.map(|opt| {
        opt.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
    });

    let project = ProjectService::update(
        &state.db, &id, body.name, body.status,
        body.contact_id, body.number, start, end,
        body.language, body.description,
        body.budget_hours, body.budget_amount, body.hourly_rate,
        body.soft_budget_hours, body.hard_budget_hours,
        body.soft_budget_amount, body.hard_budget_amount,
        body.contact_person_id,
        body.invoicing_method, body.currency,
        body.rounding_method, body.rounding_factor_minutes,
        body.flat_rate_total, body.sub_status_id,
        body.owner_id,
    ).await?;

    let resp = ProjectResponse::from(project.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "project",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/projects/{id}/summary",
    responses((status = 200, body = ProjectSummaryResponse)),
    security(("bearer" = []))
)]
pub async fn get_project_summary(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectSummaryResponse>, AppError> {
    let summary = ProjectService::get_summary(&state.db, &id).await?;
    Ok(Json(ProjectSummaryResponse {
        project: ProjectResponse::from(summary.project),
        contact_name: summary.contact_name,
        total_hours: summary.total_hours,
        billable_hours: summary.billable_hours,
        budget_hours_remaining: summary.budget_hours_remaining,
        total_invoiced: summary.total_invoiced,
    }))
}

#[utoipa::path(
    delete, path = "/api/v1/projects/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ProjectService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "project",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    get, path = "/api/v1/projects/{id}/budget-analytics",
    responses((status = 200, body = BudgetAnalyticsResponse)),
    security(("bearer" = []))
)]
pub async fn get_budget_analytics(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BudgetAnalyticsResponse>, AppError> {
    let analytics = ProjectBudgetService::get_budget_analytics(&state.db, &id).await?;

    Ok(Json(BudgetAnalyticsResponse {
        total_hours: analytics.total_hours,
        billable_hours: analytics.billable_hours,
        non_billable_hours: analytics.non_billable_hours,
        billed_hours: analytics.billed_hours,
        unbilled_hours: analytics.unbilled_hours,
        budget_hours: analytics.budget_hours,
        actual_amount: analytics.actual_amount,
        invoiced_amount: analytics.invoiced_amount,
        per_member: analytics.per_member.into_iter().map(|m| MemberBudgetRow {
            user_id: m.user_id,
            user_name: m.user_name,
            budget_hours: m.budget_hours,
            actual_hours: m.actual_hours,
            rate: m.rate,
            actual_amount: m.actual_amount,
        }).collect(),
        per_activity: analytics.per_activity.into_iter().map(|a| ActivityBudgetRow {
            activity_type_id: a.activity_type_id,
            activity_name: a.activity_name,
            budget_hours: a.budget_hours,
            actual_hours: a.actual_hours,
            chargeable: a.chargeable,
            rate: a.rate,
            actual_amount: a.actual_amount,
        }).collect(),
        timeline: analytics.timeline.into_iter().map(|t| WeeklyBudgetPoint {
            week_start: t.week_start,
            cumulative_budget: t.cumulative_budget,
            cumulative_actual: t.cumulative_actual,
        }).collect(),
    }))
}
