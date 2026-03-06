use axum::extract::{Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::employee_service::EmployeeService;
use konto_core::services::payroll_run_service::PayrollRunService;
use konto_core::services::pdf_payslip::PdfPayslipService;

use crate::dto::payroll_run::*;
use crate::state::AppState;

/// List all payroll runs.
#[utoipa::path(
    get, path = "/api/v1/payroll-runs",
    responses((status = 200, body = Vec<PayrollRunResponse>)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn list_payroll_runs(
    State(state): State<AppState>,
) -> Result<Json<Vec<PayrollRunResponse>>, AppError> {
    let runs = PayrollRunService::list(&state.db).await?;
    let data = runs.into_iter().map(PayrollRunResponse::from).collect();
    Ok(Json(data))
}

/// Get payroll run with lines.
#[utoipa::path(
    get, path = "/api/v1/payroll-runs/{id}",
    responses((status = 200, body = PayrollRunDetailResponse)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn get_payroll_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PayrollRunDetailResponse>, AppError> {
    let run = PayrollRunService::get(&state.db, &id).await?;
    let lines = PayrollRunService::get_lines(&state.db, &id).await?;

    let line_responses = resolve_employee_names(&state, lines).await;

    Ok(Json(PayrollRunDetailResponse {
        run: PayrollRunResponse::from(run),
        lines: line_responses,
    }))
}

/// Create a new payroll run.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs",
    request_body = CreatePayrollRunRequest,
    responses((status = 201, body = PayrollRunResponse)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn create_payroll_run(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreatePayrollRunRequest>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    let run = PayrollRunService::create(&state.db, body.month, body.year).await?;

    let resp = PayrollRunResponse::from(run.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "payroll_run",
        Some(&run.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Calculate payroll for all active employees.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/calculate",
    responses((status = 200, body = PayrollRunDetailResponse)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn calculate_payroll_run(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<PayrollRunDetailResponse>, AppError> {
    let (run, lines) = PayrollRunService::calculate(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "calculate", "payroll_run",
        Some(&id), None, None,
    ).await;

    let line_responses = resolve_employee_names(&state, lines).await;

    Ok(Json(PayrollRunDetailResponse {
        run: PayrollRunResponse::from(run),
        lines: line_responses,
    }))
}

/// Approve payroll run and create journal entry.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/approve",
    responses((status = 200, body = PayrollRunResponse)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn approve_payroll_run(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    let run = PayrollRunService::approve(&state.db, &id, &claims.sub).await?;

    let resp = PayrollRunResponse::from(run);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "approve", "payroll_run",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Mark payroll run as paid.
#[utoipa::path(
    post, path = "/api/v1/payroll-runs/{id}/pay",
    responses((status = 200, body = PayrollRunResponse)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn mark_payroll_run_paid(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    let run = PayrollRunService::mark_paid(&state.db, &id).await?;

    let resp = PayrollRunResponse::from(run);
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "mark_paid", "payroll_run",
        Some(&id), None, None,
    ).await;

    Ok(Json(resp))
}

/// Delete a payroll run (draft/calculated only).
#[utoipa::path(
    delete, path = "/api/v1/payroll-runs/{id}",
    responses((status = 200)),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn delete_payroll_run(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    PayrollRunService::delete(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "payroll_run",
        Some(&id), None, None,
    ).await;

    Ok(Json(serde_json::json!({"deleted": true})))
}

/// Download payslip PDF for a single employee.
#[utoipa::path(
    get, path = "/api/v1/payroll-runs/{id}/payslip/{employee_id}",
    responses((status = 200, content_type = "application/pdf")),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn download_payslip(
    State(state): State<AppState>,
    Path((id, employee_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let pdf = PdfPayslipService::generate(&state.db, &id, &employee_id).await?;
    let filename = format!("payslip-{id}-{employee_id}.pdf");
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        pdf,
    ))
}

/// Download all payslips as ZIP.
#[utoipa::path(
    get, path = "/api/v1/payroll-runs/{id}/payslips",
    responses((status = 200, content_type = "application/zip")),
    security(("bearer" = [])),
    tag = "payroll-runs"
)]
pub async fn download_payslips_zip(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let all = PdfPayslipService::generate_all(&state.db, &id).await?;
    let zip_bytes = create_zip(&all, &id)?;
    let filename = format!("payslips-{id}.zip");
    Ok((
        [
            (header::CONTENT_TYPE, "application/zip".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        zip_bytes,
    ))
}

fn create_zip(files: &[(String, Vec<u8>)], run_id: &str) -> Result<Vec<u8>, AppError> {
    use std::io::{Cursor, Write};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buf);
    let options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for (name, pdf) in files {
        let filename = format!("payslip-{run_id}-{name}.pdf");
        zip.start_file(filename, options)
            .map_err(|e| AppError::Internal(format!("ZIP error: {e}")))?;
        zip.write_all(pdf)
            .map_err(|e| AppError::Internal(format!("ZIP write error: {e}")))?;
    }

    let result = zip
        .finish()
        .map_err(|e| AppError::Internal(format!("ZIP finish error: {e}")))?;
    Ok(result.into_inner())
}

async fn resolve_employee_names(
    state: &AppState,
    lines: Vec<konto_db::entities::payroll_run_line::Model>,
) -> Vec<PayrollRunLineResponse> {
    let mut result = Vec::new();
    for line in lines {
        let name = EmployeeService::get(&state.db, &line.employee_id)
            .await
            .ok()
            .map(|e| format!("{} {}", e.first_name, e.last_name));
        result.push(PayrollRunLineResponse::from_model(line, name));
    }
    result
}
