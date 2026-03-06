use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::timesheet_service::TimesheetService;
use konto_core::services::timesheet_workflow;

use crate::dto::timesheet::{
    CreateTimesheetRequest, TimesheetResponse, UpdateTimesheetRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/timesheets",
    params(PaginationParams),
    responses((status = 200, body = Vec<TimesheetResponse>))
)]
pub async fn list_timesheets(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<TimesheetResponse>>, AppError> {
    let user_id_filter = params.search.as_deref();
    let (timesheets, total) = TimesheetService::list(
        &state.db,
        params.page(),
        params.per_page(),
        user_id_filter,
    )
    .await?;

    let data = timesheets
        .into_iter()
        .map(TimesheetResponse::from)
        .collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    post, path = "/api/v1/timesheets",
    request_body = CreateTimesheetRequest,
    responses((status = 201, body = TimesheetResponse))
)]
pub async fn create_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateTimesheetRequest>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let period_start = chrono::NaiveDate::parse_from_str(&body.period_start, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid period_start date format".to_string()))?;
    let period_end = chrono::NaiveDate::parse_from_str(&body.period_end, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid period_end date format".to_string()))?;

    let ts = TimesheetService::create(
        &state.db,
        &claims.sub,
        period_start,
        period_end,
        body.notes,
    )
    .await?;

    let new_vals = serde_json::to_string(&TimesheetResponse::from(ts.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "timesheet",
        Some(&ts.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    get, path = "/api/v1/timesheets/{id}",
    responses((status = 200, body = TimesheetResponse))
)]
pub async fn get_timesheet(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let ts = TimesheetService::get_by_id(&state.db, &id).await?;
    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    put, path = "/api/v1/timesheets/{id}",
    request_body = UpdateTimesheetRequest,
    responses((status = 200, body = TimesheetResponse))
)]
pub async fn update_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTimesheetRequest>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let period_start = body
        .period_start
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid period_start date".to_string()))
        })
        .transpose()?;

    let period_end = body
        .period_end
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid period_end date".to_string()))
        })
        .transpose()?;

    let ts = TimesheetService::update(
        &state.db,
        &id,
        body.notes,
        period_start,
        period_end,
    )
    .await?;

    let new_vals = serde_json::to_string(&TimesheetResponse::from(ts.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "timesheet",
        Some(&id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    post, path = "/api/v1/timesheets/{id}/submit",
    responses((status = 200, body = TimesheetResponse))
)]
pub async fn submit_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let ts = timesheet_workflow::submit(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "submit",
        "timesheet",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    post, path = "/api/v1/timesheets/{id}/approve",
    responses((status = 200, body = TimesheetResponse))
)]
pub async fn approve_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let ts = timesheet_workflow::approve(&state.db, &id, &claims.sub).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "approve",
        "timesheet",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    post, path = "/api/v1/timesheets/{id}/reject",
    responses((status = 200, body = TimesheetResponse))
)]
pub async fn reject_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<TimesheetResponse>, AppError> {
    let ts = timesheet_workflow::reject(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "reject",
        "timesheet",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(Json(TimesheetResponse::from(ts)))
}

#[utoipa::path(
    delete, path = "/api/v1/timesheets/{id}",
    responses((status = 204))
)]
pub async fn delete_timesheet(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    TimesheetService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "timesheet",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}
