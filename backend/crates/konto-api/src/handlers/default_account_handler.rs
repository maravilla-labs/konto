use axum::extract::State;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::default_account_service::DefaultAccountService;
use konto_db::repository::account_repo::AccountRepo;

use crate::dto::default_account::{
    DefaultAccountResponse, UpdateDefaultAccountsRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/settings/default-accounts",
    responses((status = 200, body = Vec<DefaultAccountResponse>)),
    security(("bearer" = []))
)]
pub async fn get_default_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<DefaultAccountResponse>>, AppError> {
    let defaults = DefaultAccountService::list(&state.db).await?;
    let mut results = Vec::new();

    for da in defaults {
        let (account_name, account_number) =
            if let Some(ref aid) = da.account_id {
                let acc = AccountRepo::find_by_id(&state.db, aid)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
                match acc {
                    Some(a) => (Some(a.name), Some(a.number.to_string())),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

        results.push(DefaultAccountResponse {
            id: da.id,
            setting_key: da.setting_key,
            account_id: da.account_id,
            account_name,
            account_number,
            description: da.description,
        });
    }

    Ok(Json(results))
}

#[utoipa::path(
    put, path = "/api/v1/settings/default-accounts",
    request_body = UpdateDefaultAccountsRequest,
    responses((status = 200, body = Vec<DefaultAccountResponse>)),
    security(("bearer" = []))
)]
pub async fn update_default_accounts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateDefaultAccountsRequest>,
) -> Result<Json<Vec<DefaultAccountResponse>>, AppError> {
    let updates: Vec<(String, Option<String>)> = body
        .settings
        .into_iter()
        .map(|s| (s.setting_key, s.account_id))
        .collect();

    DefaultAccountService::bulk_update(&state.db, updates).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "default_accounts",
        None,
        None,
        None,
    )
    .await?;

    // Return fresh data with resolved names
    get_default_accounts(State(state)).await
}
