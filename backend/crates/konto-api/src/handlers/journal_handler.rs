use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::export_service::ExportService;
use konto_core::services::journal_service::{JournalLineInput, JournalService};
use serde::Deserialize;

use crate::dto::journal::{
    BulkPostRequest, BulkPostResponse, CreateJournalEntryRequest, JournalDetailResponse,
    JournalEntryResponse, JournalLineResponse,
};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct JournalQueryParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search: Option<String>,
    pub format: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl JournalQueryParams {
    fn page(&self) -> u64 { self.page.unwrap_or(1).max(1) }
    fn per_page(&self) -> u64 { self.per_page.unwrap_or(50).clamp(1, 200) }
}

#[utoipa::path(
    get, path = "/api/v1/journal",
    params(PaginationParams),
    responses((status = 200, body = Vec<JournalEntryResponse>))
)]
pub async fn list_journal(
    State(state): State<AppState>,
    Query(params): Query<JournalQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (entries, _) = JournalService::list(
            &state.db, 1, u64::MAX, params.search.as_deref(),
            params.date_from.as_deref(), params.date_to.as_deref(),
            params.sort_by.as_deref(), params.sort_order.as_deref(),
        ).await?;
        let data: Vec<JournalEntryResponse> = entries.into_iter().map(JournalEntryResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"journal.csv\"".to_string())],
            csv_bytes,
        ).into_response());
    }

    let (entries, total) = JournalService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.search.as_deref(),
        params.date_from.as_deref(),
        params.date_to.as_deref(),
        params.sort_by.as_deref(),
        params.sort_order.as_deref(),
    )
    .await?;

    let data = entries
        .into_iter()
        .map(JournalEntryResponse::from)
        .collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/journal/{id}",
    responses((status = 200, body = JournalDetailResponse))
)]
pub async fn get_journal_entry(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<JournalDetailResponse>, AppError> {
    let (entry, lines) = JournalService::get_by_id(&state.db, &id).await?;

    Ok(Json(JournalDetailResponse {
        entry: JournalEntryResponse::from(entry),
        lines: lines
            .into_iter()
            .map(JournalLineResponse::from)
            .collect(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/journal",
    request_body = CreateJournalEntryRequest,
    responses((status = 201, body = JournalDetailResponse))
)]
pub async fn create_journal_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateJournalEntryRequest>,
) -> Result<Json<JournalDetailResponse>, AppError> {
    let date = chrono::NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid date format, use YYYY-MM-DD".to_string()))?;

    let lines: Vec<JournalLineInput> = body
        .lines
        .into_iter()
        .map(|l| JournalLineInput {
            account_id: l.account_id,
            debit_amount: l.debit_amount,
            credit_amount: l.credit_amount,
            description: l.description,
            vat_rate_id: l.vat_rate_id,
        })
        .collect();

    let (entry, created_lines) = JournalService::create(
        &state.db,
        date,
        &body.description,
        body.reference,
        body.currency_id,
        body.exchange_rate,
        Some(claims.sub.clone()),
        lines,
    )
    .await?;

    let resp = JournalDetailResponse {
        entry: JournalEntryResponse::from(entry.clone()),
        lines: created_lines
            .into_iter()
            .map(JournalLineResponse::from)
            .collect(),
    };

    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "journal_entry",
        Some(&entry.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/journal/{id}/post",
    responses((status = 200, body = JournalEntryResponse))
)]
pub async fn post_journal_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<JournalEntryResponse>, AppError> {
    let entry = JournalService::post_entry(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "journal_entry",
        Some(&id),
        Some("draft"),
        Some("posted"),
    )
    .await?;

    Ok(Json(JournalEntryResponse::from(entry)))
}

#[utoipa::path(
    post, path = "/api/v1/journal/{id}/reverse",
    responses((status = 200, body = JournalDetailResponse))
)]
pub async fn reverse_journal_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<JournalDetailResponse>, AppError> {
    let (entry, lines) =
        JournalService::reverse_entry(&state.db, &id, &claims.sub).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "journal_entry",
        Some(&id),
        Some("posted"),
        Some("reversed"),
    )
    .await?;

    Ok(Json(JournalDetailResponse {
        entry: JournalEntryResponse::from(entry),
        lines: lines
            .into_iter()
            .map(JournalLineResponse::from)
            .collect(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/journal/bulk-post",
    request_body = BulkPostRequest,
    responses((status = 200, body = BulkPostResponse))
)]
pub async fn bulk_post_journal(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<BulkPostRequest>,
) -> Result<Json<BulkPostResponse>, AppError> {
    let posted = JournalService::bulk_post(&state.db, body.entry_ids, body.all_drafts).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "bulk_post",
        "journal_entry",
        None,
        None,
        Some(&format!("{posted} entries posted")),
    )
    .await?;

    Ok(Json(BulkPostResponse { posted }))
}
