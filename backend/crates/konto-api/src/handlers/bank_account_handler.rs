use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::bank_account_service::BankAccountService;

use crate::dto::settings::{
    BankAccountResponse, CreateBankAccountRequest, UpdateBankAccountRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/bank-accounts",
    responses((status = 200, body = Vec<BankAccountResponse>)),
    security(("bearer" = []))
)]
pub async fn list_bank_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<BankAccountResponse>>, AppError> {
    let accounts = BankAccountService::list(&state.db).await?;
    let data = accounts.into_iter().map(BankAccountResponse::from).collect();
    Ok(Json(data))
}

#[utoipa::path(
    post, path = "/api/v1/bank-accounts",
    request_body = CreateBankAccountRequest,
    responses((status = 201, body = BankAccountResponse)),
    security(("bearer" = []))
)]
pub async fn create_bank_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateBankAccountRequest>,
) -> Result<Json<BankAccountResponse>, AppError> {
    let account = BankAccountService::create(
        &state.db,
        &body.name,
        &body.bank_name,
        &body.iban,
        body.bic,
        body.currency_id,
        body.account_id,
        body.qr_iban,
        body.is_default,
    )
    .await?;

    let resp = BankAccountResponse::from(account.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "bank_account",
        Some(&account.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/bank-accounts/{id}",
    request_body = UpdateBankAccountRequest,
    responses((status = 200, body = BankAccountResponse)),
    security(("bearer" = []))
)]
pub async fn update_bank_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateBankAccountRequest>,
) -> Result<Json<BankAccountResponse>, AppError> {
    let account = BankAccountService::update(
        &state.db,
        &id,
        &body.name,
        &body.bank_name,
        &body.iban,
        body.bic,
        body.currency_id,
        body.account_id,
        body.qr_iban,
        body.is_default,
    )
    .await?;

    let resp = BankAccountResponse::from(account.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "bank_account",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/bank-accounts/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_bank_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    BankAccountService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "bank_account",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}
