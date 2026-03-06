use axum::extract::State;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::payroll_settings_service::PayrollSettingsService;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::dto::payroll_settings::*;
use crate::state::AppState;

/// Get payroll settings.
#[utoipa::path(
    get, path = "/api/v1/payroll-settings",
    responses((status = 200, body = PayrollSettingsResponse)),
    security(("bearer" = [])),
    tag = "payroll-settings"
)]
pub async fn get_payroll_settings(
    State(state): State<AppState>,
) -> Result<Json<PayrollSettingsResponse>, AppError> {
    let settings = PayrollSettingsService::get(&state.db).await?;
    Ok(Json(PayrollSettingsResponse::from(settings)))
}

/// Update payroll settings.
#[utoipa::path(
    put, path = "/api/v1/payroll-settings",
    request_body = UpdatePayrollSettingsRequest,
    responses((status = 200, body = PayrollSettingsResponse)),
    security(("bearer" = [])),
    tag = "payroll-settings"
)]
pub async fn update_payroll_settings(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdatePayrollSettingsRequest>,
) -> Result<Json<PayrollSettingsResponse>, AppError> {
    let settings = PayrollSettingsService::update(
        &state.db,
        d(body.ahv_iv_eo_rate_employee)?,
        d(body.ahv_iv_eo_rate_employer)?,
        d(body.alv_rate_employee)?,
        d(body.alv_rate_employer)?,
        d(body.alv_salary_cap)?,
        d(body.bvg_coordination_deduction)?,
        d(body.bvg_entry_threshold)?,
        d(body.bvg_min_insured_salary)?,
        d(body.bvg_max_insured_salary)?,
        d(body.bvg_rate_25_34)?,
        d(body.bvg_rate_35_44)?,
        d(body.bvg_rate_45_54)?,
        d(body.bvg_rate_55_65)?,
        d(body.bvg_risk_rate)?,
        d(body.bvg_employer_share_pct)?,
        d(body.nbu_rate_employee)?,
        d(body.bu_rate_employer)?,
        d(body.ktg_rate_employee)?,
        d(body.ktg_rate_employer)?,
        d(body.fak_rate_employer)?,
        d(body.uvg_max_salary)?,
        body.payment_bank_account_id,
        body.company_clearing_number,
    ).await?;

    let resp = PayrollSettingsResponse::from(settings.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "payroll_settings",
        Some(&settings.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

fn d(v: f64) -> Result<Decimal, AppError> {
    Decimal::from_str(&v.to_string())
        .map_err(|_| AppError::BadRequest("Invalid decimal value".into()))
}
