use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::expense_category_service::ExpenseCategoryService;

use crate::dto::expense::{
    CreateExpenseCategoryRequest, ExpenseCategoryResponse, UpdateExpenseCategoryRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/expense-categories",
    responses((status = 200, body = Vec<ExpenseCategoryResponse>)),
    security(("bearer" = []))
)]
pub async fn list_expense_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExpenseCategoryResponse>>, AppError> {
    let categories = ExpenseCategoryService::list(&state.db).await?;
    let data = categories.into_iter().map(ExpenseCategoryResponse::from).collect();
    Ok(Json(data))
}

#[utoipa::path(
    post, path = "/api/v1/expense-categories",
    request_body = CreateExpenseCategoryRequest,
    responses((status = 201, body = ExpenseCategoryResponse)),
    security(("bearer" = []))
)]
pub async fn create_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateExpenseCategoryRequest>,
) -> Result<Json<ExpenseCategoryResponse>, AppError> {
    let cat = ExpenseCategoryService::create(
        &state.db,
        &body.name,
        body.account_id,
    ).await?;

    let resp = ExpenseCategoryResponse::from(cat.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "expense_category",
        Some(&cat.id), None, new_vals.as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/expense-categories/{id}",
    request_body = UpdateExpenseCategoryRequest,
    responses((status = 200, body = ExpenseCategoryResponse)),
    security(("bearer" = []))
)]
pub async fn update_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateExpenseCategoryRequest>,
) -> Result<Json<ExpenseCategoryResponse>, AppError> {
    let cat = ExpenseCategoryService::update(
        &state.db,
        &id,
        &body.name,
        body.account_id,
        body.is_active,
    ).await?;

    let resp = ExpenseCategoryResponse::from(cat.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "expense_category",
        Some(&id), None, new_vals.as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/expense-categories/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ExpenseCategoryService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "expense_category",
        Some(&id), None, None,
    ).await?;

    Ok(())
}
