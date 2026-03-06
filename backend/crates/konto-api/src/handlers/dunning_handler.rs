use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::dunning_service::DunningService;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use crate::dto::dunning::*;
use crate::state::AppState;

/// List all dunning levels
#[utoipa::path(
    get,
    path = "/api/v1/dunning/levels",
    tag = "dunning",
    responses((status = 200, body = Vec<DunningLevelResponse>))
)]
pub async fn list_dunning_levels(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<Vec<DunningLevelResponse>>, AppError> {
    let levels = DunningService::list_levels(&state.db).await?;
    Ok(Json(levels.into_iter().map(DunningLevelResponse::from).collect()))
}

/// Update a dunning level
#[utoipa::path(
    put,
    path = "/api/v1/dunning/levels/{id}",
    tag = "dunning",
    request_body = UpdateDunningLevelRequest,
    responses((status = 200, body = DunningLevelResponse))
)]
pub async fn update_dunning_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateDunningLevelRequest>,
) -> Result<Json<DunningLevelResponse>, AppError> {
    let fee = Decimal::from_f64(body.fee_amount).unwrap_or(Decimal::ZERO);
    let level = DunningService::update_level(
        &state.db,
        &id,
        body.days_after_due,
        fee,
        &body.subject_template,
        &body.body_template,
        body.is_active,
    )
    .await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "dunning_level",
        Some(&id), None, None,
    ).await;

    Ok(Json(DunningLevelResponse::from(level)))
}

/// Get dunning history for a specific invoice
#[utoipa::path(
    get,
    path = "/api/v1/invoices/{id}/dunning",
    tag = "dunning",
    responses((status = 200, body = Vec<DunningEntryResponse>))
)]
pub async fn get_invoice_dunning_history(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<DunningEntryResponse>>, AppError> {
    let details = DunningService::get_dunning_history(&state.db, &id).await?;

    let entries: Vec<DunningEntryResponse> = details
        .into_iter()
        .map(|d| DunningEntryResponse {
            id: d.entry.id,
            invoice_id: d.entry.invoice_id,
            dunning_level_id: d.entry.dunning_level_id,
            level_name: d.level_name,
            level_number: d.level_number,
            sent_at: d.entry.sent_at.to_string(),
            fee_amount: d.entry.fee_amount.to_string(),
            email_sent: d.entry.email_sent,
            journal_entry_id: d.entry.journal_entry_id,
            notes: d.entry.notes,
        })
        .collect();

    Ok(Json(entries))
}

/// Send a manual reminder for an invoice
#[utoipa::path(
    post,
    path = "/api/v1/invoices/{id}/dunning",
    tag = "dunning",
    request_body = SendReminderRequest,
    responses((status = 200, body = DunningEntryResponse))
)]
pub async fn send_reminder(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<SendReminderRequest>,
) -> Result<Json<DunningEntryResponse>, AppError> {
    let entry = DunningService::send_reminder(
        &state.db,
        &id,
        &body.dunning_level_id,
        body.send_email,
        Some(&claims.sub),
    )
    .await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "dunning_entry",
        Some(&entry.id), None, None,
    ).await;

    // Re-fetch to get level details
    let details = DunningService::get_dunning_history(&state.db, &id).await?;
    let detail = details
        .into_iter()
        .find(|d| d.entry.id == entry.id)
        .map(|d| DunningEntryResponse {
            id: d.entry.id,
            invoice_id: d.entry.invoice_id,
            dunning_level_id: d.entry.dunning_level_id,
            level_name: d.level_name,
            level_number: d.level_number,
            sent_at: d.entry.sent_at.to_string(),
            fee_amount: d.entry.fee_amount.to_string(),
            email_sent: d.entry.email_sent,
            journal_entry_id: d.entry.journal_entry_id,
            notes: d.entry.notes,
        })
        .unwrap_or_else(|| DunningEntryResponse::from(entry));

    Ok(Json(detail))
}

/// Run the automated dunning process
#[utoipa::path(
    post,
    path = "/api/v1/dunning/run",
    tag = "dunning",
    responses((status = 200, body = DunningRunResponse))
)]
pub async fn run_dunning(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<DunningRunResponse>, AppError> {
    let result = DunningService::run_dunning(&state.db).await?;

    let summary = format!("sent: {}, errors: {}", result.reminders_sent, result.errors.len());
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "run", "dunning",
        Some("batch"), None, Some(&summary),
    ).await;

    Ok(Json(DunningRunResponse {
        reminders_sent: result.reminders_sent,
        errors: result.errors,
    }))
}
