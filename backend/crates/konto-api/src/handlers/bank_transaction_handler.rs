use axum::extract::{Multipart, Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::bank_import_service::BankImportService;
use konto_core::services::bank_reconciliation_service::BankReconciliationService;
use konto_core::services::export_service::ExportService;
use konto_db::repository::bank_transaction_repo::BankTransactionRepo;

use crate::dto::bank_transaction::*;
use crate::state::AppState;

/// List bank transactions (paginated, filterable).
#[utoipa::path(
    get, path = "/api/v1/bank-transactions",
    params(BankTransactionListParams),
    responses((status = 200, body = Vec<BankTransactionResponse>)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn list_bank_transactions(
    State(state): State<AppState>,
    Query(params): Query<BankTransactionListParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (txs, _) = BankTransactionRepo::find_paginated(
            &state.db, 1, u64::MAX,
            params.bank_account_id.as_deref(),
            params.status.as_deref(),
        ).await.map_err(|e| AppError::Database(e.to_string()))?;
        let data: Vec<BankTransactionResponse> =
            txs.into_iter().map(BankTransactionResponse::from).collect();
        let csv = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"bank_transactions.csv\"".to_string())],
            csv,
        ).into_response());
    }

    let (txs, total) = BankTransactionRepo::find_paginated(
        &state.db,
        params.page(),
        params.per_page(),
        params.bank_account_id.as_deref(),
        params.status.as_deref(),
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    let data: Vec<BankTransactionResponse> =
        txs.into_iter().map(BankTransactionResponse::from).collect();
    let body = serde_json::json!({ "data": data, "total": total });
    Ok(Json(body).into_response())
}

/// Upload CAMT.053 XML and import transactions.
#[utoipa::path(
    post, path = "/api/v1/bank-transactions/import/{bank_account_id}",
    responses((status = 201, body = ImportResponse)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn import_camt053(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(bank_account_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            file_data = Some(
                field.bytes().await
                    .map_err(|e| AppError::BadRequest(format!("Failed to read file: {e}")))?
                    .to_vec(),
            );
        }
    }

    let data = file_data.ok_or_else(|| AppError::BadRequest("No file uploaded".into()))?;
    let xml = String::from_utf8(data)
        .map_err(|_| AppError::BadRequest("File is not valid UTF-8".into()))?;

    let parsed = BankImportService::parse_camt053(&xml)?;
    let count = BankImportService::import(&state.db, &bank_account_id, &parsed).await?;

    let _ = AuditService::log(
        &state.db,
        Some(&claims.sub),
        "import",
        "bank_transaction",
        Some(&bank_account_id),
        None,
        Some(&format!("Imported {count} transactions from CAMT.053")),
    ).await;

    Ok(Json(ImportResponse {
        imported_count: count,
        batch_id: bank_account_id,
    }))
}

/// Auto-match unmatched transactions for a bank account.
#[utoipa::path(
    post, path = "/api/v1/bank-transactions/auto-match/{bank_account_id}",
    responses((status = 200, body = AutoMatchResponse)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn auto_match(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(bank_account_id): Path<String>,
) -> Result<Json<AutoMatchResponse>, AppError> {
    let result = BankReconciliationService::auto_match(
        &state.db,
        &bank_account_id,
        &claims.sub,
    ).await?;

    let _ = AuditService::log(
        &state.db,
        Some(&claims.sub),
        "auto_match",
        "bank_transaction",
        Some(&bank_account_id),
        None,
        Some(&format!("Matched {} transactions", result.matched_count)),
    ).await;

    Ok(Json(AutoMatchResponse {
        matched_count: result.matched_count,
        unmatched_count: result.unmatched_count,
    }))
}

/// Manually match a transaction to an invoice or expense.
#[utoipa::path(
    post, path = "/api/v1/bank-transactions/{id}/match",
    request_body = ManualMatchRequest,
    responses((status = 200, body = BankTransactionResponse)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn manual_match(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<ManualMatchRequest>,
) -> Result<Json<BankTransactionResponse>, AppError> {
    let tx = BankReconciliationService::manual_match(
        &state.db, &id, &body.target_type, &body.target_id, &claims.sub,
    ).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "manual_match", "bank_transaction",
        Some(&id), None,
        Some(&format!("Matched to {} {}", body.target_type, body.target_id)),
    ).await;

    Ok(Json(BankTransactionResponse::from(tx)))
}

/// Create a journal entry from an unmatched transaction.
#[utoipa::path(
    post, path = "/api/v1/bank-transactions/{id}/journal",
    request_body = CreateJournalFromTxRequest,
    responses((status = 200, body = BankTransactionResponse)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn create_journal_from_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<CreateJournalFromTxRequest>,
) -> Result<Json<BankTransactionResponse>, AppError> {
    let tx = BankReconciliationService::create_journal_entry(
        &state.db, &id, &body.debit_account_id, &body.credit_account_id, &claims.sub,
    ).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create_journal", "bank_transaction",
        Some(&id), None, None,
    ).await;

    Ok(Json(BankTransactionResponse::from(tx)))
}

/// Ignore a transaction (mark as ignored).
#[utoipa::path(
    post, path = "/api/v1/bank-transactions/{id}/ignore",
    responses((status = 200, body = BankTransactionResponse)),
    security(("bearer" = [])),
    tag = "bank-transactions"
)]
pub async fn ignore_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<BankTransactionResponse>, AppError> {
    let tx = BankReconciliationService::ignore(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "ignore", "bank_transaction",
        Some(&id), None, None,
    ).await;

    Ok(Json(BankTransactionResponse::from(tx)))
}
