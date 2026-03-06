use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::exchange_rate_service::ExchangeRateService;

use crate::dto::exchange_rate::{
    CreateExchangeRateRequest, ExchangeRateResponse, LatestRateQuery,
    UpdateExchangeRateRequest,
};
use crate::state::AppState;

async fn resolve_currency_id(
    db: &sea_orm::DatabaseConnection,
    input: &str,
) -> Result<String, AppError> {
    // If it looks like a UUID, return as-is
    if input.len() > 3 {
        return Ok(input.to_string());
    }
    // Otherwise treat as currency code and look up
    use konto_db::entities::currency;
    use sea_orm::*;
    let currency = currency::Entity::find()
        .filter(currency::Column::Code.eq(input.to_uppercase()))
        .one(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Currency code '{}' not found", input)))?;
    Ok(currency.id)
}

#[utoipa::path(
    get, path = "/api/v1/exchange-rates",
    params(PaginationParams),
    responses((status = 200, body = Vec<ExchangeRateResponse>))
)]
pub async fn list_exchange_rates(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ExchangeRateResponse>>, AppError> {
    let (rates, total) = ExchangeRateService::list(
        &state.db,
        params.page(),
        params.per_page(),
    )
    .await?;

    let data = rates.into_iter().map(ExchangeRateResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    get, path = "/api/v1/exchange-rates/latest",
    params(LatestRateQuery),
    responses((status = 200, body = ExchangeRateResponse))
)]
pub async fn get_latest_rate(
    State(state): State<AppState>,
    Query(query): Query<LatestRateQuery>,
) -> Result<Json<ExchangeRateResponse>, AppError> {
    let rate = ExchangeRateService::get_latest(
        &state.db,
        &query.from_currency_id,
        &query.to_currency_id,
    )
    .await?;
    Ok(Json(ExchangeRateResponse::from(rate)))
}

#[utoipa::path(
    get, path = "/api/v1/exchange-rates/{id}",
    responses((status = 200, body = ExchangeRateResponse))
)]
pub async fn get_exchange_rate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ExchangeRateResponse>, AppError> {
    let rate = ExchangeRateService::get_by_id(&state.db, &id).await?;
    Ok(Json(ExchangeRateResponse::from(rate)))
}

#[utoipa::path(
    post, path = "/api/v1/exchange-rates",
    request_body = CreateExchangeRateRequest,
    responses((status = 201, body = ExchangeRateResponse))
)]
pub async fn create_exchange_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateExchangeRateRequest>,
) -> Result<Json<ExchangeRateResponse>, AppError> {
    let valid_date = chrono::NaiveDate::parse_from_str(&body.valid_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid valid_date format".to_string()))?;

    let from_id = resolve_currency_id(&state.db, &body.from_currency_id).await?;
    let to_id = resolve_currency_id(&state.db, &body.to_currency_id).await?;

    let rate = ExchangeRateService::create(
        &state.db,
        &from_id,
        &to_id,
        body.rate,
        valid_date,
        body.source,
    )
    .await?;

    let new_vals = serde_json::to_string(&ExchangeRateResponse::from(rate.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "exchange_rate",
        Some(&rate.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(ExchangeRateResponse::from(rate)))
}

#[utoipa::path(
    put, path = "/api/v1/exchange-rates/{id}",
    request_body = UpdateExchangeRateRequest,
    responses((status = 200, body = ExchangeRateResponse))
)]
pub async fn update_exchange_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateExchangeRateRequest>,
) -> Result<Json<ExchangeRateResponse>, AppError> {
    let valid_date = body
        .valid_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid valid_date".to_string()))
        })
        .transpose()?;

    let rate = ExchangeRateService::update(
        &state.db,
        &id,
        body.rate,
        valid_date,
        body.source,
    )
    .await?;

    let new_vals = serde_json::to_string(&ExchangeRateResponse::from(rate.clone())).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "exchange_rate",
        Some(&id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(ExchangeRateResponse::from(rate)))
}

#[utoipa::path(
    delete, path = "/api/v1/exchange-rates/{id}",
    responses((status = 204))
)]
pub async fn delete_exchange_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ExchangeRateService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "exchange_rate",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}
