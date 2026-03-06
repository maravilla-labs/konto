use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::export_service::ExportService;
use konto_core::services::recurring_invoice_service::{
    RecurringInvoiceService, TemplateData, TemplateLineItem,
};

use crate::dto::recurring_invoice::{
    CreateRecurringInvoiceRequest, RecurringInvoiceListParams,
    RecurringInvoiceResponse, UpdateRecurringInvoiceRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/recurring-invoices",
    params(RecurringInvoiceListParams),
    responses((status = 200, body = Vec<RecurringInvoiceResponse>)),
    security(("bearer" = []))
)]
pub async fn list_recurring_invoices(
    State(state): State<AppState>,
    Query(params): Query<RecurringInvoiceListParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (items, _) = RecurringInvoiceService::list(
            &state.db, 1, u64::MAX,
            params.is_active,
            params.search.as_deref(),
        ).await?;
        let data: Vec<RecurringInvoiceResponse> =
            items.into_iter().map(RecurringInvoiceResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [
                (header::CONTENT_TYPE, "text/csv".to_string()),
                (header::CONTENT_DISPOSITION, "attachment; filename=\"recurring-invoices.csv\"".to_string()),
            ],
            csv_bytes,
        ).into_response());
    }

    let (items, total) = RecurringInvoiceService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.is_active,
        params.search.as_deref(),
    ).await?;

    let data = items.into_iter().map(RecurringInvoiceResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data, total, params.page(), params.per_page(),
    )).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/recurring-invoices/{id}",
    responses((status = 200, body = RecurringInvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn get_recurring_invoice(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<RecurringInvoiceResponse>, AppError> {
    let model = RecurringInvoiceService::get_by_id(&state.db, &id).await?;
    Ok(Json(RecurringInvoiceResponse::from(model)))
}

#[utoipa::path(
    post, path = "/api/v1/recurring-invoices",
    request_body = CreateRecurringInvoiceRequest,
    responses((status = 201, body = RecurringInvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn create_recurring_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateRecurringInvoiceRequest>,
) -> Result<Json<RecurringInvoiceResponse>, AppError> {
    let next_run_date = parse_date(&body.next_run_date)?;
    let end_date = body.end_date.as_deref().map(parse_date).transpose()?;
    let tpl = build_template_data(
        &body.language,
        &body.currency_id,
        &body.notes,
        &body.payment_terms,
        &body.lines,
    );

    let model = RecurringInvoiceService::create(
        &state.db,
        &body.contact_id,
        body.project_id,
        &tpl,
        &body.frequency,
        body.interval_days,
        next_run_date,
        end_date,
        body.auto_send.unwrap_or(false),
        Some(claims.sub.clone()),
    ).await?;

    let resp = RecurringInvoiceResponse::from(model.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "recurring_invoice",
        Some(&model.id), None,
        serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/recurring-invoices/{id}",
    request_body = UpdateRecurringInvoiceRequest,
    responses((status = 200, body = RecurringInvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn update_recurring_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateRecurringInvoiceRequest>,
) -> Result<Json<RecurringInvoiceResponse>, AppError> {
    let next_run_date = parse_date(&body.next_run_date)?;
    let end_date = body.end_date.as_deref().map(parse_date).transpose()?;
    let tpl = build_template_data(
        &body.language,
        &body.currency_id,
        &body.notes,
        &body.payment_terms,
        &body.lines,
    );

    let model = RecurringInvoiceService::update(
        &state.db,
        &id,
        &body.contact_id,
        body.project_id,
        &tpl,
        &body.frequency,
        body.interval_days,
        next_run_date,
        end_date,
        body.auto_send.unwrap_or(false),
        body.is_active.unwrap_or(true),
    ).await?;

    let resp = RecurringInvoiceResponse::from(model);
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "recurring_invoice",
        Some(&id), None,
        serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/recurring-invoices/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_recurring_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    RecurringInvoiceService::delete(&state.db, &id).await?;
    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "recurring_invoice",
        Some(&id), None, None,
    ).await?;
    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/recurring-invoices/trigger",
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn trigger_recurring_invoices(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, AppError> {
    let count = RecurringInvoiceService::generate_due_invoices(&state.db).await?;
    AuditService::log(
        &state.db, Some(&claims.sub), "trigger", "recurring_invoice",
        None, None,
        Some(&format!("{{\"generated\":{count}}}")),
    ).await?;
    Ok(Json(serde_json::json!({ "generated": count })))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("Invalid date format: {s}")))
}

fn build_template_data(
    language: &Option<String>,
    currency_id: &Option<String>,
    notes: &Option<String>,
    payment_terms: &Option<String>,
    lines: &[crate::dto::recurring_invoice::RecurringInvoiceLineRequest],
) -> TemplateData {
    TemplateData {
        language: language.clone(),
        currency_id: currency_id.clone(),
        notes: notes.clone(),
        payment_terms: payment_terms.clone(),
        lines: lines.iter().map(|l| TemplateLineItem {
            description: l.description.clone(),
            quantity: l.quantity,
            unit_price: l.unit_price,
            vat_rate_id: l.vat_rate_id.clone(),
            account_id: l.account_id.clone(),
        }).collect(),
    }
}
