use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::depreciation_service::DepreciationService;
use konto_core::services::fixed_asset_service::FixedAssetService;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::dto::fixed_asset::*;
use crate::state::AppState;

/// List all fixed assets.
#[utoipa::path(
    get, path = "/api/v1/fixed-assets",
    responses((status = 200, body = Vec<FixedAssetResponse>)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn list_fixed_assets(
    State(state): State<AppState>,
) -> Result<Json<Vec<FixedAssetResponse>>, AppError> {
    let assets = FixedAssetService::list(&state.db).await?;
    let data = assets.into_iter().map(FixedAssetResponse::from).collect();
    Ok(Json(data))
}

/// Get a fixed asset by ID.
#[utoipa::path(
    get, path = "/api/v1/fixed-assets/{id}",
    responses((status = 200, body = FixedAssetResponse)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn get_fixed_asset(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FixedAssetResponse>, AppError> {
    let asset = FixedAssetService::get_by_id(&state.db, &id).await?;
    Ok(Json(FixedAssetResponse::from(asset)))
}

/// Create a new fixed asset.
#[utoipa::path(
    post, path = "/api/v1/fixed-assets",
    request_body = CreateFixedAssetRequest,
    responses((status = 201, body = FixedAssetResponse)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn create_fixed_asset(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateFixedAssetRequest>,
) -> Result<Json<FixedAssetResponse>, AppError> {
    let acquisition_date = NaiveDate::parse_from_str(&body.acquisition_date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid acquisition_date format, expected YYYY-MM-DD".into()))?;
    let acquisition_cost = Decimal::from_str(&body.acquisition_cost.to_string())
        .map_err(|_| AppError::BadRequest("Invalid acquisition_cost".into()))?;
    let residual_value = Decimal::from_str(&body.residual_value.to_string())
        .map_err(|_| AppError::BadRequest("Invalid residual_value".into()))?;
    let declining_rate = body.declining_rate
        .map(|r| Decimal::from_str(&r.to_string()))
        .transpose()
        .map_err(|_| AppError::BadRequest("Invalid declining_rate".into()))?;

    let asset = FixedAssetService::create(
        &state.db,
        &body.name,
        body.description,
        &body.account_id,
        &body.depreciation_account_id,
        acquisition_date,
        acquisition_cost,
        residual_value,
        body.useful_life_years,
        &body.depreciation_method,
        declining_rate,
    )
    .await?;

    let resp = FixedAssetResponse::from(asset.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "fixed_asset",
        Some(&asset.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Update a fixed asset.
#[utoipa::path(
    put, path = "/api/v1/fixed-assets/{id}",
    request_body = UpdateFixedAssetRequest,
    responses((status = 200, body = FixedAssetResponse)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn update_fixed_asset(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateFixedAssetRequest>,
) -> Result<Json<FixedAssetResponse>, AppError> {
    let acquisition_date = NaiveDate::parse_from_str(&body.acquisition_date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid acquisition_date format".into()))?;
    let acquisition_cost = Decimal::from_str(&body.acquisition_cost.to_string())
        .map_err(|_| AppError::BadRequest("Invalid acquisition_cost".into()))?;
    let residual_value = Decimal::from_str(&body.residual_value.to_string())
        .map_err(|_| AppError::BadRequest("Invalid residual_value".into()))?;
    let declining_rate = body.declining_rate
        .map(|r| Decimal::from_str(&r.to_string()))
        .transpose()
        .map_err(|_| AppError::BadRequest("Invalid declining_rate".into()))?;
    let disposed_date = body.disposed_date.as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| AppError::BadRequest("Invalid disposed_date format".into()))?;

    let asset = FixedAssetService::update(
        &state.db,
        &id,
        &body.name,
        body.description,
        &body.account_id,
        &body.depreciation_account_id,
        acquisition_date,
        acquisition_cost,
        residual_value,
        body.useful_life_years,
        &body.depreciation_method,
        declining_rate,
        &body.status,
        disposed_date,
    )
    .await?;

    let resp = FixedAssetResponse::from(asset);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "fixed_asset",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Delete a fixed asset.
#[utoipa::path(
    delete, path = "/api/v1/fixed-assets/{id}",
    responses((status = 200)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn delete_fixed_asset(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    FixedAssetService::delete(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "fixed_asset",
        Some(&id), None, None,
    ).await;

    Ok(Json(serde_json::json!({ "ok": true })))
}

/// Get depreciation schedule for a fixed asset.
#[utoipa::path(
    get, path = "/api/v1/fixed-assets/{id}/schedule",
    responses((status = 200, body = Vec<DepreciationEntryResponse>)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn get_depreciation_schedule(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<DepreciationEntryResponse>>, AppError> {
    let entries = DepreciationService::get_schedule(&state.db, &id).await?;
    let data = entries.into_iter().map(DepreciationEntryResponse::from).collect();
    Ok(Json(data))
}

/// Run depreciation for all active assets in a fiscal year.
#[utoipa::path(
    post, path = "/api/v1/fixed-assets/run-depreciation",
    request_body = RunDepreciationRequest,
    responses((status = 200, body = Vec<DepreciationEntryResponse>)),
    security(("bearer" = [])),
    tag = "fixed-assets"
)]
pub async fn run_depreciation(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<RunDepreciationRequest>,
) -> Result<Json<Vec<DepreciationEntryResponse>>, AppError> {
    let entries = DepreciationService::run_depreciation(
        &state.db,
        &body.fiscal_year_id,
        &claims.sub,
    )
    .await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "run_depreciation", "fixed_asset",
        None, None, Some(&format!("fiscal_year_id={}", body.fiscal_year_id)),
    ).await;

    let data = entries.into_iter().map(DepreciationEntryResponse::from).collect();
    Ok(Json(data))
}
