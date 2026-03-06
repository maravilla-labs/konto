use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::annual_report_note_service::AnnualReportNoteService;
use konto_core::services::annual_report_service::AnnualReportService;
use konto_core::services::audit_service::AuditService;
use konto_core::services::pdf_annual_report::PdfAnnualReportService;
use konto_core::services::report_service::ReportService;
use konto_core::services::shareholder_service::ShareholderService;

use crate::dto::annual_report::*;
use crate::state::AppState;

// --- Shareholder CRUD ---

pub async fn list_shareholders(
    State(state): State<AppState>,
) -> Result<Json<Vec<ShareholderResponse>>, AppError> {
    let list = ShareholderService::list(&state.db).await?;
    Ok(Json(list.into_iter().map(ShareholderResponse::from).collect()))
}

pub async fn create_shareholder(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateShareholderRequest>,
) -> Result<Json<ShareholderResponse>, AppError> {
    let sh = ShareholderService::create(
        &state.db,
        &body.name,
        &body.city,
        &body.role,
        body.signing_rights,
        body.sort_order,
    )
    .await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "create", "shareholder",
        Some(&sh.id), None, None,
    ).await?;

    Ok(Json(ShareholderResponse::from(sh)))
}

pub async fn update_shareholder(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateShareholderRequest>,
) -> Result<Json<ShareholderResponse>, AppError> {
    let sh = ShareholderService::update(
        &state.db,
        &id,
        &body.name,
        &body.city,
        &body.role,
        body.signing_rights,
        body.sort_order,
    )
    .await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "update", "shareholder",
        Some(&id), None, None,
    ).await?;

    Ok(Json(ShareholderResponse::from(sh)))
}

pub async fn delete_shareholder(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    ShareholderService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "shareholder",
        Some(&id), None, None,
    ).await?;

    Ok(Json(serde_json::json!({"deleted": true})))
}

// --- Annual Report Notes ---

pub async fn list_notes(
    State(state): State<AppState>,
    Path(fy_id): Path<String>,
) -> Result<Json<Vec<AnnualReportNoteResponse>>, AppError> {
    // Ensure defaults are seeded
    AnnualReportNoteService::seed_defaults(&state.db, &fy_id).await?;
    let notes = AnnualReportNoteService::get_all_for_year(&state.db, &fy_id).await?;
    Ok(Json(notes.into_iter().map(AnnualReportNoteResponse::from).collect()))
}

pub async fn get_note(
    State(state): State<AppState>,
    Path((fy_id, section)): Path<(String, String)>,
) -> Result<Json<AnnualReportNoteResponse>, AppError> {
    let note = AnnualReportNoteService::get_section(&state.db, &fy_id, &section)
        .await?
        .ok_or_else(|| AppError::NotFound("Note section not found".into()))?;
    Ok(Json(AnnualReportNoteResponse::from(note)))
}

pub async fn update_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((fy_id, section)): Path<(String, String)>,
    Json(body): Json<UpdateNoteRequest>,
) -> Result<Json<AnnualReportNoteResponse>, AppError> {
    let content_str = serde_json::to_string(&body.content)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let note = AnnualReportNoteService::upsert_section(
        &state.db,
        &fy_id,
        &section,
        &content_str,
        body.label.as_deref(),
        body.sort_order,
    )
    .await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "update", "annual_report_note",
        Some(&note.id), None, None,
    ).await?;

    Ok(Json(AnnualReportNoteResponse::from(note)))
}

pub async fn create_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(fy_id): Path<String>,
    Json(body): Json<CreateNoteRequest>,
) -> Result<Json<AnnualReportNoteResponse>, AppError> {
    let note = AnnualReportNoteService::create_custom(
        &state.db,
        &fy_id,
        &body.label,
        body.sort_order,
    )
    .await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "create", "annual_report_note",
        Some(&note.id), None, None,
    ).await?;

    Ok(Json(AnnualReportNoteResponse::from(note)))
}

pub async fn delete_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((fy_id, section)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    AnnualReportNoteService::delete_section(&state.db, &fy_id, &section).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "annual_report_note",
        None, None, None,
    ).await?;

    Ok(Json(serde_json::json!({"deleted": true})))
}

// --- Annual Report ---

pub async fn get_annual_report(
    State(state): State<AppState>,
    Path(fy_id): Path<String>,
) -> Result<Json<AnnualReportResponse>, AppError> {
    let report = AnnualReportService::get_or_create(&state.db, &fy_id).await?;
    Ok(Json(AnnualReportResponse::from(report)))
}

pub async fn generate_pdf(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(fy_id): Path<String>,
) -> Result<Json<AnnualReportResponse>, AppError> {
    let pdf_bytes = PdfAnnualReportService::generate(&state.db, &fy_id).await?;

    // Store via StorageService
    let filename = format!("jahresrechnung_{}.pdf", fy_id);
    let path = state.storage.upload(&filename, &pdf_bytes, "application/pdf").await?;

    AnnualReportService::set_pdf_path(&state.db, &fy_id, &path).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "generate", "annual_report",
        Some(&fy_id), None, None,
    ).await?;

    let report = AnnualReportService::get_or_create(&state.db, &fy_id).await?;
    Ok(Json(AnnualReportResponse::from(report)))
}

pub async fn download_pdf(
    State(state): State<AppState>,
    Path(fy_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pdf_bytes = PdfAnnualReportService::generate(&state.db, &fy_id).await?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"jahresrechnung.pdf\"",
            ),
        ],
        pdf_bytes,
    ))
}

pub async fn finalize_report(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(fy_id): Path<String>,
) -> Result<Json<AnnualReportResponse>, AppError> {
    let report = AnnualReportService::finalize(&state.db, &fy_id, &claims.sub).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "finalize", "annual_report",
        Some(&fy_id), None, None,
    ).await?;

    Ok(Json(AnnualReportResponse::from(report)))
}

// --- Swiss Reports ---

pub async fn swiss_balance_sheet(
    State(state): State<AppState>,
    Query(params): Query<BalanceSheetParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let as_of = chrono::NaiveDate::parse_from_str(&params.as_of, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid as_of date".into()))?;
    let bs = ReportService::swiss_balance_sheet(&state.db, as_of).await?;
    Ok(Json(serde_json::to_value(bs)
        .map_err(|e| AppError::Internal(format!("Failed to serialize balance sheet: {e}")))?))
}

pub async fn swiss_income_statement(
    State(state): State<AppState>,
    Query(params): Query<IncomeStatementParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&params.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".into()))?;
    let to = chrono::NaiveDate::parse_from_str(&params.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".into()))?;
    let is = ReportService::swiss_income_statement(&state.db, from, to).await?;
    Ok(Json(serde_json::to_value(is)
        .map_err(|e| AppError::Internal(format!("Failed to serialize income statement: {e}")))?))
}
