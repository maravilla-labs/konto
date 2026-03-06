use axum::extract::{Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::pain_001_service::Pain001Service;
use konto_core::services::payout_service::PayoutService;

use crate::dto::payout_entry::*;
use crate::state::AppState;

/// List payout entries for a payroll run.
#[utoipa::path(
    get, path = "/api/v1/payroll-runs/{id}/payout-entries",
    responses((status = 200, body = Vec<PayoutEntryResponse>)),
    security(("bearer" = [])),
    tag = "payout"
)]
pub async fn list_payout_entries(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<PayoutEntryResponse>>, AppError> {
    let entries = PayoutService::list_by_run(&state.db, &id).await?;
    let data = entries.into_iter().map(PayoutEntryResponse::from).collect();
    Ok(Json(data))
}

/// Generate payout entries from an approved payroll run.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/generate-payouts",
    responses((status = 200, body = Vec<PayoutEntryResponse>)),
    security(("bearer" = [])),
    tag = "payout"
)]
pub async fn generate_payouts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<PayoutEntryResponse>>, AppError> {
    let entries = PayoutService::generate_payouts(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "generate_payouts", "payroll_run",
        Some(&id), None, None,
    ).await;

    let data = entries.into_iter().map(PayoutEntryResponse::from).collect();
    Ok(Json(data))
}

/// Export pain.001 XML for a payroll run.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/export-pain001",
    responses((status = 200, description = "pain.001 XML file", content_type = "application/xml")),
    security(("bearer" = [])),
    tag = "payout"
)]
pub async fn export_pain001(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let xml_bytes = Pain001Service::generate(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "export_pain001", "payroll_run",
        Some(&id), None, None,
    ).await;

    let filename = format!("pain001_payroll_{}.xml", id);

    Ok((
        [
            (header::CONTENT_TYPE, "application/xml".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        xml_bytes,
    ))
}

/// Mark a single payout entry as paid.
#[utoipa::path(
    put, path = "/api/v1/payout-entries/{id}/mark-paid",
    responses((status = 200, body = PayoutEntryResponse)),
    security(("bearer" = [])),
    tag = "payout"
)]
pub async fn mark_payout_paid(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<PayoutEntryResponse>, AppError> {
    let entry = PayoutService::mark_paid(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "mark_paid", "payout_entry",
        Some(&id), None, None,
    ).await;

    Ok(Json(PayoutEntryResponse::from(entry)))
}

/// Mark all payout entries for a run as paid.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/mark-all-paid",
    responses((status = 200)),
    security(("bearer" = [])),
    tag = "payout"
)]
pub async fn mark_all_payouts_paid(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    PayoutService::mark_all_paid(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "mark_all_paid", "payroll_run",
        Some(&id), None, None,
    ).await;

    Ok(Json(serde_json::json!({"success": true})))
}
