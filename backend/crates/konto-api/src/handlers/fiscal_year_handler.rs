use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::fiscal_year_service::FiscalYearService;

use crate::dto::fiscal_year::{
    CreateFiscalYearRequest, FiscalPeriodResponse, FiscalYearDetailResponse,
    FiscalYearResponse, UpdateFiscalYearRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/fiscal-years",
    params(PaginationParams),
    responses((status = 200, body = Vec<FiscalYearResponse>))
)]
pub async fn list_fiscal_years(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<FiscalYearResponse>>, AppError> {
    let (years, total) = FiscalYearService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.search.as_deref(),
    )
    .await?;

    let data = years.into_iter().map(FiscalYearResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    get, path = "/api/v1/fiscal-years/{id}",
    responses((status = 200, body = FiscalYearDetailResponse))
)]
pub async fn get_fiscal_year(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FiscalYearDetailResponse>, AppError> {
    let fy = FiscalYearService::get_by_id(&state.db, &id).await?;
    let periods = FiscalYearService::get_periods(&state.db, &id).await?;

    Ok(Json(FiscalYearDetailResponse {
        fiscal_year: FiscalYearResponse::from(fy),
        periods: periods
            .into_iter()
            .map(FiscalPeriodResponse::from)
            .collect(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/fiscal-years",
    request_body = CreateFiscalYearRequest,
    responses((status = 201, body = FiscalYearResponse))
)]
pub async fn create_fiscal_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateFiscalYearRequest>,
) -> Result<Json<FiscalYearResponse>, AppError> {
    let start = chrono::NaiveDate::parse_from_str(&body.start_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid start_date format".to_string()))?;
    let end = chrono::NaiveDate::parse_from_str(&body.end_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid end_date format".to_string()))?;

    let fy = FiscalYearService::create(&state.db, &body.name, start, end).await?;

    let new_vals = serde_json::to_string(&FiscalYearResponse::from(fy.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "fiscal_year",
        Some(&fy.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(FiscalYearResponse::from(fy)))
}

#[utoipa::path(
    put, path = "/api/v1/fiscal-years/{id}",
    request_body = UpdateFiscalYearRequest,
    responses((status = 200, body = FiscalYearResponse))
)]
pub async fn update_fiscal_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateFiscalYearRequest>,
) -> Result<Json<FiscalYearResponse>, AppError> {
    let start = body
        .start_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid start_date".to_string()))
        })
        .transpose()?;
    let end = body
        .end_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid end_date".to_string()))
        })
        .transpose()?;

    let fy =
        FiscalYearService::update(&state.db, &id, body.name, start, end).await?;

    let new_vals = serde_json::to_string(&FiscalYearResponse::from(fy.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "fiscal_year",
        Some(&id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(FiscalYearResponse::from(fy)))
}

#[utoipa::path(
    post, path = "/api/v1/fiscal-years/{id}/close",
    responses((status = 200, body = FiscalYearResponse))
)]
pub async fn close_fiscal_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<FiscalYearResponse>, AppError> {
    let fy = FiscalYearService::close(&state.db, &id, &claims.sub).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "fiscal_year",
        Some(&id),
        None,
        Some("closed"),
    )
    .await?;

    Ok(Json(FiscalYearResponse::from(fy)))
}
