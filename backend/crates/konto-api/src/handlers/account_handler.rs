use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::account_service::{AccountNodeWithBalance, AccountService};
use konto_core::services::audit_service::AuditService;

use crate::dto::account::{
    AccountResponse, AccountTreeNode, CreateAccountRequest, UpdateAccountRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/accounts",
    params(PaginationParams),
    responses((status = 200, body = Vec<AccountResponse>))
)]
pub async fn list_accounts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<AccountResponse>>, AppError> {
    let (accounts, total) = AccountService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.search.as_deref(),
    )
    .await?;

    let data = accounts.into_iter().map(AccountResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    post, path = "/api/v1/accounts",
    request_body = CreateAccountRequest,
    responses((status = 201, body = AccountResponse))
)]
pub async fn create_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    let account = AccountService::create(
        &state.db,
        body.number,
        &body.name,
        body.description,
        body.parent_id,
        body.currency_id,
    )
    .await?;

    let resp = AccountResponse::from(account.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "account",
        Some(&account.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/accounts/{id}",
    responses((status = 200, body = AccountResponse))
)]
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AccountResponse>, AppError> {
    let account = AccountService::get_by_id(&state.db, &id).await?;
    Ok(Json(AccountResponse::from(account)))
}

#[utoipa::path(
    put, path = "/api/v1/accounts/{id}",
    request_body = UpdateAccountRequest,
    responses((status = 200, body = AccountResponse))
)]
pub async fn update_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    let account = AccountService::update(
        &state.db,
        &id,
        body.name,
        body.description,
        body.is_active,
        body.parent_id,
        body.currency_id,
    )
    .await?;

    let resp = AccountResponse::from(account.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "account",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/accounts/{id}",
    responses((status = 204))
)]
pub async fn delete_account(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    AccountService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "account",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    get, path = "/api/v1/accounts/tree",
    responses((status = 200, body = Vec<AccountTreeNode>))
)]
pub async fn accounts_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<AccountTreeNode>>, AppError> {
    let accounts = AccountService::get_all(&state.db).await?;

    // Build tree by account type groupings
    let mut tree: Vec<AccountTreeNode> = Vec::new();
    for acct in &accounts {
        tree.push(AccountTreeNode {
            id: acct.id.clone(),
            number: acct.number,
            name: acct.name.clone(),
            account_type: acct.account_type.clone(),
            description: acct.description.clone(),
            is_active: acct.is_active,
            children: vec![],
        });
    }

    Ok(Json(tree))
}

#[utoipa::path(
    get, path = "/api/v1/accounts/tree-with-balances",
    responses((status = 200, body = Vec<serde_json::Value>)),
    security(("bearer" = []))
)]
pub async fn accounts_tree_with_balances(
    State(state): State<AppState>,
) -> Result<Json<Vec<AccountNodeWithBalance>>, AppError> {
    let tree = AccountService::get_tree_with_balances(&state.db).await?;
    Ok(Json(tree))
}
