use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::currency_service::CurrencyService;

use crate::dto::currency::*;
use crate::state::AppState;

/// List all currencies.
#[utoipa::path(
    get, path = "/api/v1/currencies",
    responses((status = 200, body = Vec<CurrencyResponse>)),
    security(("bearer" = [])),
    tag = "currencies"
)]
pub async fn list_currencies(
    State(state): State<AppState>,
) -> Result<Json<Vec<CurrencyResponse>>, AppError> {
    let currencies = CurrencyService::list(&state.db).await?;
    let data = currencies.into_iter().map(CurrencyResponse::from).collect();
    Ok(Json(data))
}

/// Create a new currency.
#[utoipa::path(
    post, path = "/api/v1/currencies",
    request_body = CreateCurrencyRequest,
    responses((status = 201, body = CurrencyResponse)),
    security(("bearer" = [])),
    tag = "currencies"
)]
pub async fn create_currency(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateCurrencyRequest>,
) -> Result<Json<CurrencyResponse>, AppError> {
    let cur = CurrencyService::create(&state.db, &body.code, &body.name, &body.symbol).await?;

    let resp = CurrencyResponse::from(cur.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "currency",
        Some(&cur.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update an existing currency.
#[utoipa::path(
    put, path = "/api/v1/currencies/{id}",
    request_body = UpdateCurrencyRequest,
    responses((status = 200, body = CurrencyResponse)),
    security(("bearer" = [])),
    tag = "currencies"
)]
pub async fn update_currency(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCurrencyRequest>,
) -> Result<Json<CurrencyResponse>, AppError> {
    let cur = CurrencyService::update(&state.db, &id, &body.code, &body.name, &body.symbol).await?;

    let resp = CurrencyResponse::from(cur);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "currency",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}
