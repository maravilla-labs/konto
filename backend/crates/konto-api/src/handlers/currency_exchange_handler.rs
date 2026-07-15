use axum::extract::{Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::currency_exchange_service::CurrencyExchangeService;

use crate::dto::currency_exchange::{CurrencyExchangeResponse, RecordTransferRequest};
use crate::state::AppState;

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid date format, expected YYYY-MM-DD".to_string()))
}

#[utoipa::path(
    get, path = "/api/v1/currency-exchanges",
    params(PaginationParams),
    responses((status = 200, body = Vec<CurrencyExchangeResponse>)),
    security(("bearer" = []))
)]
pub async fn list_currency_exchanges(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<CurrencyExchangeResponse>>, AppError> {
    let (items, total) =
        CurrencyExchangeService::list(&state.db, params.page(), params.per_page()).await?;
    let data = items.into_iter().map(CurrencyExchangeResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, params.page(), params.per_page())))
}

#[utoipa::path(
    post, path = "/api/v1/currency-exchanges",
    request_body = RecordTransferRequest,
    responses((status = 201, body = CurrencyExchangeResponse)),
    security(("bearer" = []))
)]
pub async fn record_transfer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<RecordTransferRequest>,
) -> Result<Json<CurrencyExchangeResponse>, AppError> {
    let date = parse_date(&body.date)?;

    let record = CurrencyExchangeService::record_transfer(
        &state.db,
        &body.from_bank_account_id,
        &body.to_bank_account_id,
        body.from_amount,
        body.to_amount,
        date,
        body.notes,
        &claims.sub,
    )
    .await?;

    let resp = CurrencyExchangeResponse::from(record);
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "currency_exchange",
        Some(&resp.id),
        None,
        serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}
