use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::credit_note_service::{CreditNoteService, LineInput};
use konto_core::services::export_service::ExportService;

use crate::dto::credit_note::{
    CreateCreditNoteRequest, CreditNoteDetailResponse, CreditNoteLineResponse,
    CreditNoteListParams, CreditNoteResponse, UpdateCreditNoteRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/credit-notes",
    params(CreditNoteListParams),
    responses((status = 200, body = Vec<CreditNoteResponse>)),
    security(("bearer" = []))
)]
pub async fn list_credit_notes(
    State(state): State<AppState>,
    Query(params): Query<CreditNoteListParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (items, _) = CreditNoteService::list(
            &state.db, 1, u64::MAX,
            params.status.as_deref(),
            params.search.as_deref(),
        ).await?;
        let data: Vec<CreditNoteResponse> = items.into_iter().map(CreditNoteResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"credit-notes.csv\"".to_string())],
            csv_bytes,
        ).into_response());
    }

    let (items, total) = CreditNoteService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.status.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    let data = items.into_iter().map(CreditNoteResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/credit-notes/{id}",
    responses((status = 200, body = CreditNoteDetailResponse)),
    security(("bearer" = []))
)]
pub async fn get_credit_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CreditNoteDetailResponse>, AppError> {
    let detail = CreditNoteService::get_by_id(&state.db, &id).await?;
    Ok(Json(CreditNoteDetailResponse {
        credit_note: CreditNoteResponse::from(detail.credit_note),
        lines: detail.lines.into_iter().map(CreditNoteLineResponse::from).collect(),
        contact_name: detail.contact_name,
        invoice_number: detail.invoice_number,
    }))
}

#[utoipa::path(
    post, path = "/api/v1/credit-notes",
    request_body = CreateCreditNoteRequest,
    responses((status = 201, body = CreditNoteResponse)),
    security(("bearer" = []))
)]
pub async fn create_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateCreditNoteRequest>,
) -> Result<Json<CreditNoteResponse>, AppError> {
    let issue_date = parse_date(&body.issue_date)?;
    let lines = map_line_inputs(body.lines);

    let cn = CreditNoteService::create(
        &state.db,
        &body.contact_id,
        body.invoice_id,
        issue_date,
        body.currency_id,
        body.notes,
        lines,
        Some(claims.sub.clone()),
    )
    .await?;

    let resp = CreditNoteResponse::from(cn.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "credit_note",
        Some(&cn.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/credit-notes/{id}",
    request_body = UpdateCreditNoteRequest,
    responses((status = 200, body = CreditNoteResponse)),
    security(("bearer" = []))
)]
pub async fn update_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCreditNoteRequest>,
) -> Result<Json<CreditNoteResponse>, AppError> {
    let issue_date = parse_date(&body.issue_date)?;
    let lines = map_line_inputs(body.lines);

    let cn = CreditNoteService::update(
        &state.db, &id, &body.contact_id, body.invoice_id,
        issue_date, body.currency_id, body.notes, lines,
    )
    .await?;

    let resp = CreditNoteResponse::from(cn.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "credit_note",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/credit-notes/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    CreditNoteService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "credit_note",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/credit-notes/{id}/issue",
    responses((status = 200, body = CreditNoteResponse)),
    security(("bearer" = []))
)]
pub async fn issue_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<CreditNoteResponse>, AppError> {
    let cn = CreditNoteService::issue(&state.db, &id, &claims.sub).await?;

    let resp = CreditNoteResponse::from(cn.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "issue", "credit_note",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/credit-notes/{id}/apply",
    responses((status = 200, body = CreditNoteResponse)),
    security(("bearer" = []))
)]
pub async fn apply_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<CreditNoteResponse>, AppError> {
    let cn = CreditNoteService::apply(&state.db, &id).await?;

    let resp = CreditNoteResponse::from(cn.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "apply", "credit_note",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/credit-notes/{id}/cancel",
    responses((status = 200, body = CreditNoteResponse)),
    security(("bearer" = []))
)]
pub async fn cancel_credit_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<CreditNoteResponse>, AppError> {
    let cn = CreditNoteService::cancel(&state.db, &id, &claims.sub).await?;

    let resp = CreditNoteResponse::from(cn.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "cancel", "credit_note",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/credit-notes/{id}/pdf",
    responses((status = 200, content_type = "application/pdf")),
    security(("bearer" = []))
)]
pub async fn download_credit_note_pdf(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pdf_bytes =
        konto_core::services::pdf_credit_note::PdfCreditNoteService::generate(&state.db, &id).await?;
    let detail = CreditNoteService::get_by_id(&state.db, &id).await?;
    let number = detail.credit_note.credit_note_number.as_deref().unwrap_or("draft");
    let filename = format!("credit-note-{number}.pdf");
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        pdf_bytes,
    ))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("Invalid date format: {s}")))
}

fn map_line_inputs(
    lines: Vec<crate::dto::credit_note::CreateCreditNoteLineRequest>,
) -> Vec<LineInput> {
    lines
        .into_iter()
        .map(|l| LineInput {
            description: l.description,
            quantity: l.quantity,
            unit_price: l.unit_price,
            vat_rate_id: l.vat_rate_id,
            account_id: l.account_id,
        })
        .collect()
}
