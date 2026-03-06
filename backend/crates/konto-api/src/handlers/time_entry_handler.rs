use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::UserRole;
use konto_common::pagination::PaginatedResponse;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::time_entry_service::TimeEntryService;
use konto_core::services::time_entry_workflow::TimeEntryWorkflow;

use crate::dto::time_entry::{
    CreateTimeEntryRequest, TimeEntryListParams, TimeEntryResponse, TransitionTimeEntryRequest,
    UpdateTimeEntryRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/time-entries",
    params(TimeEntryListParams),
    responses((status = 200, body = Vec<TimeEntryResponse>))
)]
pub async fn list_time_entries(
    State(state): State<AppState>,
    Query(params): Query<TimeEntryListParams>,
) -> Result<Json<PaginatedResponse<TimeEntryResponse>>, AppError> {
    let (entries, total) =
        TimeEntryService::list(&state.db, params.page(), params.per_page(), params.project_id.as_deref(), params.billed, params.status.as_deref(), params.billable).await?;

    let data = entries
        .into_iter()
        .map(TimeEntryResponse::from)
        .collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    get, path = "/api/v1/time-entries/{id}",
    responses((status = 200, body = TimeEntryResponse))
)]
pub async fn get_time_entry(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TimeEntryResponse>, AppError> {
    let entry = TimeEntryService::get_by_id(&state.db, &id).await?;
    Ok(Json(TimeEntryResponse::from(entry)))
}

#[utoipa::path(
    post, path = "/api/v1/time-entries",
    request_body = CreateTimeEntryRequest,
    responses((status = 201, body = TimeEntryResponse))
)]
pub async fn create_time_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateTimeEntryRequest>,
) -> Result<Json<TimeEntryResponse>, AppError> {
    let date = chrono::NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid date format".to_string()))?;

    // Admin can track time for other users; non-admin always uses own id.
    let effective_user_id = if claims.role == UserRole::Admin {
        body.user_id.unwrap_or_else(|| claims.sub.clone())
    } else {
        claims.sub.clone()
    };

    let entry = TimeEntryService::create(
        &state.db,
        body.project_id,
        body.contact_id,
        Some(effective_user_id),
        body.activity_type_id,
        date,
        body.actual_minutes,
        body.estimated_minutes,
        body.description,
        body.flat_amount,
        body.travel_minutes,
        body.travel_flat_rate,
        body.travel_distance,
        body.task_id,
        body.quantity,
        body.billable,
        body.start_time.clone(),
        body.end_time.clone(),
    )
    .await?;

    let new_vals = serde_json::to_string(&TimeEntryResponse::from(entry.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "time_entry",
        Some(&entry.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(TimeEntryResponse::from(entry)))
}

#[utoipa::path(
    put, path = "/api/v1/time-entries/{id}",
    request_body = UpdateTimeEntryRequest,
    responses((status = 200, body = TimeEntryResponse))
)]
pub async fn update_time_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTimeEntryRequest>,
) -> Result<Json<TimeEntryResponse>, AppError> {
    let date = body
        .date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid date".to_string()))
        })
        .transpose()?;

    let entry = TimeEntryService::update(
        &state.db,
        &id,
        body.project_id,
        body.contact_id,
        body.activity_type_id,
        date,
        body.actual_minutes,
        body.estimated_minutes,
        body.description,
        body.flat_amount,
        body.travel_minutes,
        body.task_id,
        body.quantity,
        body.billable,
        body.start_time.clone(),
        body.end_time.clone(),
    )
    .await?;

    let new_vals = serde_json::to_string(&TimeEntryResponse::from(entry.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "time_entry",
        Some(&id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(TimeEntryResponse::from(entry)))
}

#[utoipa::path(
    delete, path = "/api/v1/time-entries/{id}",
    responses((status = 204))
)]
pub async fn delete_time_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    TimeEntryService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "time_entry",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}

/// Transition a time entry's status.
#[utoipa::path(
    put, path = "/api/v1/time-entries/{id}/transition",
    request_body = TransitionTimeEntryRequest,
    responses((status = 200, body = TimeEntryResponse)),
    security(("bearer" = [])),
    tag = "time-entries"
)]
pub async fn transition_time_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<TransitionTimeEntryRequest>,
) -> Result<Json<TimeEntryResponse>, AppError> {
    let entry = TimeEntryWorkflow::transition(&state.db, &id, &body.status).await?;

    let new_vals = serde_json::to_string(&TimeEntryResponse::from(entry.clone())).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "transition", "time_entry",
        Some(&id), None, new_vals.as_deref(),
    ).await?;

    Ok(Json(TimeEntryResponse::from(entry)))
}
