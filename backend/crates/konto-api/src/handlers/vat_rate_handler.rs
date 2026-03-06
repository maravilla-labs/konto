use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::vat_rate_service::VatRateService;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::dto::vat_rate::*;
use crate::state::AppState;

/// List all VAT rates.
#[utoipa::path(
    get, path = "/api/v1/vat-rates",
    responses((status = 200, body = Vec<VatRateResponse>)),
    security(("bearer" = [])),
    tag = "vat-rates"
)]
pub async fn list_vat_rates(
    State(state): State<AppState>,
) -> Result<Json<Vec<VatRateResponse>>, AppError> {
    let rates = VatRateService::list(&state.db).await?;
    let data = rates.into_iter().map(VatRateResponse::from).collect();
    Ok(Json(data))
}

/// Create a new VAT rate.
#[utoipa::path(
    post, path = "/api/v1/vat-rates",
    request_body = CreateVatRateRequest,
    responses((status = 201, body = VatRateResponse)),
    security(("bearer" = [])),
    tag = "vat-rates"
)]
pub async fn create_vat_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateVatRateRequest>,
) -> Result<Json<VatRateResponse>, AppError> {
    let rate = Decimal::from_str(&body.rate.to_string())
        .map_err(|_| AppError::BadRequest("Invalid rate".into()))?;
    let valid_from = parse_optional_date(&body.valid_from)?;
    let valid_to = parse_optional_date(&body.valid_to)?;

    let vat = VatRateService::create(&state.db, &body.code, &body.name, rate, &body.vat_type, body.vat_category.as_deref(), valid_from, valid_to).await?;

    let resp = VatRateResponse::from(vat.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "vat_rate",
        Some(&vat.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update an existing VAT rate.
#[utoipa::path(
    put, path = "/api/v1/vat-rates/{id}",
    request_body = UpdateVatRateRequest,
    responses((status = 200, body = VatRateResponse)),
    security(("bearer" = [])),
    tag = "vat-rates"
)]
pub async fn update_vat_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateVatRateRequest>,
) -> Result<Json<VatRateResponse>, AppError> {
    let rate = Decimal::from_str(&body.rate.to_string())
        .map_err(|_| AppError::BadRequest("Invalid rate".into()))?;
    let valid_from = parse_optional_date(&body.valid_from)?;
    let valid_to = parse_optional_date(&body.valid_to)?;

    let vat = VatRateService::update(
        &state.db, &id, &body.code, &body.name, rate, &body.vat_type, body.vat_category.as_deref(), body.is_active, valid_from, valid_to,
    ).await?;

    let resp = VatRateResponse::from(vat);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "vat_rate",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Deactivate a VAT rate (soft-delete).
#[utoipa::path(
    delete, path = "/api/v1/vat-rates/{id}",
    responses((status = 200, body = VatRateResponse)),
    security(("bearer" = [])),
    tag = "vat-rates"
)]
pub async fn deactivate_vat_rate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<VatRateResponse>, AppError> {
    let vat = VatRateService::deactivate(&state.db, &id).await?;

    let resp = VatRateResponse::from(vat);
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "deactivate", "vat_rate",
        Some(&id), None, None,
    ).await;

    Ok(Json(resp))
}

fn parse_optional_date(s: &Option<String>) -> Result<Option<NaiveDate>, AppError> {
    match s {
        Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
            .map(Some)
            .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into())),
        _ => Ok(None),
    }
}
