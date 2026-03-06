use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::{InvoiceStatus, TimeEntryStatus};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::export_service::ExportService;
use konto_core::services::invoice_service::{InvoiceService, LineInput};

use crate::dto::invoice::{
    CreateInvoiceFromTimeEntriesRequest, CreateInvoiceRequest, InvoiceDetailResponse,
    InvoiceLineResponse, InvoiceListParams, InvoiceResponse, PayInvoiceRequest,
    UpdateInvoiceRequest,
};
use crate::state::AppState;

fn round_minutes(minutes: i32, method: &str, factor: i32) -> i32 {
    if factor <= 0 { return minutes; }
    match method {
        "up" => ((minutes + factor - 1) / factor) * factor,
        "down" => (minutes / factor) * factor,
        "nearest" => ((minutes + factor / 2) / factor) * factor,
        _ => minutes,
    }
}

#[utoipa::path(
    get, path = "/api/v1/invoices",
    params(InvoiceListParams),
    responses((status = 200, body = Vec<InvoiceResponse>)),
    security(("bearer" = []))
)]
pub async fn list_invoices(
    State(state): State<AppState>,
    Query(params): Query<InvoiceListParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (invoices, _) = InvoiceService::list(
            &state.db, 1, u64::MAX,
            params.status.as_deref(),
            params.contact_id.as_deref(),
            params.project_id.as_deref(),
            params.search.as_deref(),
        ).await?;
        let data: Vec<InvoiceResponse> = invoices.into_iter().map(InvoiceResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"invoices.csv\"".to_string())],
            csv_bytes,
        ).into_response());
    }

    let (invoices, total) = InvoiceService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.status.as_deref(),
        params.contact_id.as_deref(),
        params.project_id.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    let data = invoices.into_iter().map(InvoiceResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/invoices/{id}",
    responses((status = 200, body = InvoiceDetailResponse)),
    security(("bearer" = []))
)]
pub async fn get_invoice(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceDetailResponse>, AppError> {
    let detail = InvoiceService::get_by_id(&state.db, &id).await?;
    let total = detail.invoice.total;
    let amount_paid = InvoiceService::amount_paid(&state.db, &id).await.unwrap_or_default();
    let amount_remaining = (total - amount_paid).max(rust_decimal::Decimal::ZERO);
    let payments = InvoiceService::list_payments(&state.db, &id).await.unwrap_or_default();

    Ok(Json(InvoiceDetailResponse {
        invoice: InvoiceResponse::from(detail.invoice),
        lines: detail.lines.into_iter().map(InvoiceLineResponse::from).collect(),
        contact_name: detail.contact_name,
        project_name: detail.project_name,
        contact_person_name: detail.contact_person_name,
        amount_paid,
        amount_remaining,
        payments: payments.into_iter().map(crate::dto::invoice_payment::InvoicePaymentResponse::from).collect(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/invoices",
    request_body = CreateInvoiceRequest,
    responses((status = 201, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn create_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let issue_date = parse_date(&body.issue_date)?;
    let due_date = parse_date(&body.due_date)?;
    let lines = map_line_inputs(body.lines);

    let inv = InvoiceService::create(
        &state.db,
        &body.contact_id,
        body.project_id,
        issue_date,
        due_date,
        body.language,
        body.currency_id,
        body.notes,
        body.payment_terms,
        lines,
        Some(claims.sub.clone()),
        body.header_text,
        body.footer_text,
        body.contact_person_id,
        body.bank_account_id,
    )
    .await?;

    let resp = InvoiceResponse::from(inv.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "invoice",
        Some(&inv.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/invoices/{id}",
    request_body = UpdateInvoiceRequest,
    responses((status = 200, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn update_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let issue_date = parse_date(&body.issue_date)?;
    let due_date = parse_date(&body.due_date)?;
    let lines = map_line_inputs(body.lines);

    let inv = InvoiceService::update(
        &state.db, &id, &body.contact_id, body.project_id,
        issue_date, due_date, body.language, body.currency_id, body.notes, body.payment_terms, lines,
        body.header_text, body.footer_text, body.contact_person_id, body.bank_account_id,
    )
    .await?;

    let resp = InvoiceResponse::from(inv.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "invoice",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/invoices/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    InvoiceService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "invoice",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/send",
    responses((status = 200, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn send_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let inv = InvoiceService::send_invoice(&state.db, &id, &claims.sub).await?;

    let resp = InvoiceResponse::from(inv.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "send", "invoice",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/pay",
    request_body = PayInvoiceRequest,
    responses((status = 200, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn pay_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<PayInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let payment_date = parse_date(&body.payment_date)?;

    let inv = InvoiceService::mark_paid(
        &state.db, &id, payment_date, &body.payment_account_id, &claims.sub,
    )
    .await?;

    let resp = InvoiceResponse::from(inv.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "pay", "invoice",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/cancel",
    responses((status = 200, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn cancel_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let inv = InvoiceService::cancel_invoice(&state.db, &id, &claims.sub).await?;

    let resp = InvoiceResponse::from(inv.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "cancel", "invoice",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/email",
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn email_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let detail = InvoiceService::get_by_id(&state.db, &id).await?;

    if detail.invoice.status == InvoiceStatus::Draft.as_str() {
        return Err(AppError::BadRequest("Cannot email a draft invoice".into()));
    }

    let contact_email = detail.contact_email.unwrap_or_default();
    if contact_email.is_empty() {
        return Err(AppError::BadRequest(
            "Contact has no email address".into(),
        ));
    }

    let invoice_number = detail
        .invoice
        .invoice_number
        .as_deref()
        .unwrap_or("DRAFT");

    // Build template variables
    let settings = konto_core::services::settings_service::SettingsService::get_or_create(&state.db).await?;
    let company_name = settings.trade_name.unwrap_or(settings.legal_name);

    let template_language = konto_core::services::language::normalize_or_default(
        detail
            .invoice
            .language
            .as_deref()
            .or(detail.contact_language.as_deref()),
        &settings.ui_language,
    );

    let mut vars = std::collections::HashMap::new();
    vars.insert("company_name".into(), company_name);
    vars.insert("contact_name".into(), detail.contact_name.unwrap_or_default());
    vars.insert("contact_email".into(), contact_email.clone());
    vars.insert("invoice_number".into(), invoice_number.to_string());
    vars.insert("amount".into(), detail.invoice.total.to_string());
    vars.insert("currency".into(), "CHF".into());
    vars.insert("due_date".into(), detail.invoice.due_date.to_string());
    vars.insert("today".into(), chrono::Utc::now().format("%Y-%m-%d").to_string());
    vars.insert("invoice_date".into(), detail.invoice.issue_date.to_string());

    // Try to render from email template; fall back to hardcoded
    let (subject, body) = match konto_core::services::email_template_service::EmailTemplateService::render(
        &state.db, "invoice_send", &template_language, &vars,
    ).await {
        Ok(rendered) => rendered,
        Err(_) => {
            let (s, b) = match template_language.as_str() {
                "de" => (
                    format!("Rechnung {invoice_number}"),
                    format!(
                        "Guten Tag,\n\nim Anhang finden Sie die Rechnung {invoice_number}.\n\nFreundliche Gruesse"
                    ),
                ),
                "fr" => (
                    format!("Facture {invoice_number}"),
                    format!(
                        "Bonjour,\n\nVeuillez trouver en piece jointe la facture {invoice_number}.\n\nCordialement"
                    ),
                ),
                "it" => (
                    format!("Fattura {invoice_number}"),
                    format!(
                        "Buongiorno,\n\nIn allegato trova la fattura {invoice_number}.\n\nCordiali saluti"
                    ),
                ),
                _ => (
                    format!("Invoice {invoice_number}"),
                    format!(
                        "Dear Customer,\n\nPlease find attached invoice {invoice_number}.\n\nBest regards"
                    ),
                ),
            };
            (s, b)
        }
    };

    let pdf_bytes =
        konto_core::services::pdf_invoice::PdfInvoiceService::generate(&state.db, &id).await?;
    let filename = format!("{invoice_number}.pdf");

    konto_core::services::email_service::EmailService::send_email(
        &state.db,
        &contact_email,
        &subject,
        &body,
        vec![(&filename, &pdf_bytes)],
    )
    .await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "email",
        "invoice",
        Some(&id),
        None,
        Some(&format!("{{\"to\":\"{contact_email}\"}}")),
    )
    .await?;

    Ok(Json(serde_json::json!({"message": "Invoice emailed successfully"})))
}

#[utoipa::path(
    get, path = "/api/v1/invoices/{id}/pdf",
    responses((status = 200, content_type = "application/pdf")),
    security(("bearer" = []))
)]
pub async fn download_invoice_pdf(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pdf_bytes =
        konto_core::services::pdf_invoice::PdfInvoiceService::generate(&state.db, &id).await?;
    let inv = InvoiceService::get_by_id(&state.db, &id).await?;
    let number = inv.invoice.invoice_number.as_deref().unwrap_or("draft");
    let filename = format!("invoice-{number}.pdf");
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

#[utoipa::path(
    post, path = "/api/v1/invoices/from-time-entries",
    request_body = CreateInvoiceFromTimeEntriesRequest,
    responses((status = 201, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn create_invoice_from_time_entries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<crate::dto::invoice::CreateInvoiceFromTimeEntriesRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    use konto_core::services::activity_type_service::ActivityTypeService;
    use konto_core::services::project_activity_type_service::ProjectActivityTypeService;
    use konto_core::services::project_service::ProjectService;
    use konto_core::services::time_entry_workflow::TimeEntryWorkflow;
    use rust_decimal::Decimal;

    if body.time_entry_ids.is_empty() {
        return Err(AppError::Validation("No time entries selected".into()));
    }

    // Load project rounding settings if available
    let project = if let Some(ref pid) = body.project_id {
        ProjectService::get_by_id(&state.db, pid).await.ok()
    } else {
        None
    };
    let rounding_method = project.as_ref().and_then(|p| p.rounding_method.clone());
    let rounding_factor = project.as_ref().and_then(|p| p.rounding_factor_minutes);

    // Pre-load project activity types for rate lookup
    let project_activity_types = if let Some(ref pid) = body.project_id {
        ProjectActivityTypeService::list_for_project(&state.db, pid).await.unwrap_or_default()
    } else {
        Vec::new()
    };

    // Collect time entries and build invoice lines with smart rate resolution
    let mut lines = Vec::new();
    let mut invoiceable_ids = Vec::new();
    for te_id in &body.time_entry_ids {
        let entry = konto_core::services::time_entry_service::TimeEntryService::get_by_id(
            &state.db, te_id,
        ).await?;

        // Only invoice entries that are status="done" AND billable=true
        if entry.status != TimeEntryStatus::Done.as_str() || !entry.billable {
            continue;
        }
        invoiceable_ids.push(te_id.clone());

        // Look up activity type if the entry references one
        let activity_type = if let Some(ref at_id) = entry.activity_type_id {
            ActivityTypeService::get_by_id(&state.db, at_id).await.ok()
        } else {
            None
        };

        let unit_type = activity_type.as_ref().map(|at| at.unit_type.as_str()).unwrap_or("hour");

        // Resolve rate: project_activity_type.rate → activity_type.default_rate → body.hourly_rate
        let pat_rate = entry.activity_type_id.as_ref().and_then(|at_id| {
            project_activity_types.iter()
                .find(|pat| pat.activity_type_id == *at_id)
                .and_then(|pat| pat.rate)
        });
        let at_default_rate = activity_type.as_ref().and_then(|at| at.default_rate);
        let rate = pat_rate
            .or(at_default_rate)
            .or(body.hourly_rate)
            .ok_or_else(|| AppError::Validation(
                format!("No rate found for time entry {} — set activity type rate, project rate override, or provide hourly_rate fallback", te_id)
            ))?;

        // Apply rounding to minutes if project has rounding settings
        let actual_minutes = if let (Some(method), Some(factor)) = (&rounding_method, rounding_factor) {
            round_minutes(entry.actual_minutes, method, factor)
        } else {
            entry.actual_minutes
        };

        // Resolve quantity based on unit type
        let quantity = match unit_type {
            "hour" => Decimal::from(actual_minutes) / Decimal::from(60),
            "fixed" => Decimal::from(1),
            _ => entry.quantity.ok_or_else(|| AppError::Validation(
                format!("Time entry {} has unit type '{}' but no quantity set", te_id, unit_type)
            ))?,
        };

        let at_name = activity_type.as_ref().map(|at| at.name.as_str()).unwrap_or("");
        let description = entry.description.unwrap_or_else(|| {
            if at_name.is_empty() {
                format!("{} — {} min", entry.date, actual_minutes)
            } else {
                format!("{} — {} ({})", entry.date, at_name, actual_minutes)
            }
        });

        lines.push(LineInput {
            description,
            quantity,
            unit_price: rate,
            vat_rate_id: None,
            account_id: Some(body.account_id.clone()),
            discount_percent: None,
        });
    }

    if lines.is_empty() {
        return Err(AppError::Validation(
            "No billable 'done' time entries found in selection".into(),
        ));
    }

    let today = chrono::Utc::now().date_naive();
    let due_date = today + chrono::Duration::days(30);

    let inv = InvoiceService::create(
        &state.db,
        &body.contact_id,
        body.project_id,
        today,
        due_date,
        body.language,
        None,
        None,
        None,
        lines,
        Some(claims.sub.clone()),
        None,
        None,
        None,
        None,
    ).await?;

    // Transition invoiceable entries to "invoiced" status
    TimeEntryWorkflow::mark_invoiced(&state.db, &invoiceable_ids).await?;

    let resp = InvoiceResponse::from(inv.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "create_from_time_entries", "invoice",
        Some(&inv.id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

/// Record a partial or full payment on an invoice.
#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/payments",
    request_body = crate::dto::invoice_payment::RecordPaymentRequest,
    responses((status = 201, body = crate::dto::invoice_payment::InvoicePaymentResponse)),
    security(("bearer" = []))
)]
pub async fn record_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<crate::dto::invoice_payment::RecordPaymentRequest>,
) -> Result<Json<crate::dto::invoice_payment::InvoicePaymentResponse>, AppError> {
    let payment_date = parse_date(&body.payment_date)?;

    let payment = InvoiceService::record_payment(
        &state.db, &id, body.amount, payment_date,
        &body.payment_account_id, body.payment_method, body.reference, &claims.sub,
    ).await?;

    let resp = crate::dto::invoice_payment::InvoicePaymentResponse::from(payment.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "record_payment", "invoice",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

/// List payments for an invoice.
#[utoipa::path(
    get, path = "/api/v1/invoices/{id}/payments",
    responses((status = 200, body = Vec<crate::dto::invoice_payment::InvoicePaymentResponse>)),
    security(("bearer" = []))
)]
pub async fn list_payments(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::dto::invoice_payment::InvoicePaymentResponse>>, AppError> {
    let payments = InvoiceService::list_payments(&state.db, &id).await?;
    let data = payments.into_iter().map(crate::dto::invoice_payment::InvoicePaymentResponse::from).collect();
    Ok(Json(data))
}

#[utoipa::path(
    post, path = "/api/v1/invoices/{id}/duplicate",
    responses((status = 201, body = InvoiceResponse)),
    security(("bearer" = []))
)]
pub async fn duplicate_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let inv = InvoiceService::duplicate(&state.db, &id, Some(claims.sub.clone())).await?;

    let resp = InvoiceResponse::from(inv.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "duplicate", "invoice",
        Some(&inv.id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("Invalid date format: {s}")))
}

fn map_line_inputs(lines: Vec<crate::dto::invoice::CreateInvoiceLineRequest>) -> Vec<LineInput> {
    lines
        .into_iter()
        .map(|l| LineInput {
            description: l.description,
            quantity: l.quantity,
            unit_price: l.unit_price,
            vat_rate_id: l.vat_rate_id,
            account_id: l.account_id,
            discount_percent: l.discount_percent,
        })
        .collect()
}
