use axum::extract::{Multipart, Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::expense_service::ExpenseService;
use konto_core::services::export_service::ExportService;
use std::path::Path as StdPath;
use uuid::Uuid;

use crate::dto::expense::{
    CreateExpenseRequest, ExpenseDetailResponse, ExpenseListParams,
    ExpenseResponse, PayExpenseRequest, UpdateExpenseRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/expenses",
    params(ExpenseListParams),
    responses((status = 200, body = Vec<ExpenseResponse>)),
    security(("bearer" = []))
)]
pub async fn list_expenses(
    State(state): State<AppState>,
    Query(params): Query<ExpenseListParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (expenses, _) = ExpenseService::list(
            &state.db, 1, u64::MAX,
            params.status.as_deref(),
            params.category_id.as_deref(),
            params.contact_id.as_deref(),
            params.date_from.as_deref(),
            params.date_to.as_deref(),
            params.search.as_deref(),
        ).await?;
        let data: Vec<ExpenseResponse> = expenses.into_iter().map(ExpenseResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"expenses.csv\"".to_string())],
            csv_bytes,
        ).into_response());
    }

    let (expenses, total) = ExpenseService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.status.as_deref(),
        params.category_id.as_deref(),
        params.contact_id.as_deref(),
        params.date_from.as_deref(),
        params.date_to.as_deref(),
        params.search.as_deref(),
    ).await?;

    let data = expenses.into_iter().map(ExpenseResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/expenses/{id}",
    responses((status = 200, body = ExpenseDetailResponse)),
    security(("bearer" = []))
)]
pub async fn get_expense(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ExpenseDetailResponse>, AppError> {
    let detail = ExpenseService::get_by_id(&state.db, &id).await?;
    Ok(Json(ExpenseDetailResponse {
        expense: ExpenseResponse::from(detail.expense),
        contact_name: detail.contact_name,
        category_name: detail.category_name,
        project_name: detail.project_name,
    }))
}

#[utoipa::path(
    post, path = "/api/v1/expenses",
    request_body = CreateExpenseRequest,
    responses((status = 201, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn create_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let expense_date = parse_date(&body.expense_date)?;
    let due_date = body.due_date.as_deref().map(parse_date).transpose()?;

    let exp = ExpenseService::create(
        &state.db,
        body.contact_id,
        body.category_id,
        &body.description,
        body.amount,
        &body.currency_id,
        body.vat_rate_id,
        expense_date,
        due_date,
        body.project_id,
        Some(claims.sub.clone()),
    ).await?;

    let resp = ExpenseResponse::from(exp.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "expense",
        Some(&exp.id), None, new_vals.as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/expenses/{id}",
    request_body = UpdateExpenseRequest,
    responses((status = 200, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn update_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let expense_date = parse_date(&body.expense_date)?;
    let due_date = body.due_date.as_deref().map(parse_date).transpose()?;

    let exp = ExpenseService::update(
        &state.db, &id,
        body.contact_id,
        body.category_id,
        &body.description,
        body.amount,
        &body.currency_id,
        body.vat_rate_id,
        expense_date,
        due_date,
        body.project_id,
    ).await?;

    let resp = ExpenseResponse::from(exp.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "expense",
        Some(&id), None, new_vals.as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/expenses/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ExpenseService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "expense",
        Some(&id), None, None,
    ).await?;

    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/expenses/{id}/approve",
    responses((status = 200, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn approve_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let exp = ExpenseService::approve(&state.db, &id, &claims.sub).await?;

    let resp = ExpenseResponse::from(exp.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "approve", "expense",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/expenses/{id}/pay",
    request_body = PayExpenseRequest,
    responses((status = 200, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn pay_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<PayExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let exp = ExpenseService::pay(
        &state.db, &id, &body.payment_account_id, &claims.sub,
    ).await?;

    let resp = ExpenseResponse::from(exp.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "pay", "expense",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/expenses/{id}/cancel",
    responses((status = 200, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn cancel_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let exp = ExpenseService::cancel(&state.db, &id, &claims.sub).await?;

    let resp = ExpenseResponse::from(exp.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "cancel", "expense",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    ).await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/expenses/{id}/receipt",
    responses((status = 200, body = ExpenseResponse)),
    security(("bearer" = []))
)]
pub async fn upload_receipt(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ExpenseResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart: {e}")))?
        .ok_or_else(|| AppError::BadRequest("No file uploaded".into()))?;

    let file_name = field
        .file_name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "receipt.pdf".to_string());

    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read file data: {e}")))?;

    let upload_dir = StdPath::new("uploads").join("receipts");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create uploads dir: {e}")))?;

    let ext = StdPath::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("pdf");
    let stored_name = format!("{}.{ext}", Uuid::new_v4());
    let file_path = upload_dir.join(&stored_name);

    tokio::fs::write(&file_path, &data)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to save file: {e}")))?;

    let receipt_url = format!("/uploads/receipts/{stored_name}");
    let exp = ExpenseService::upload_receipt(&state.db, &id, &receipt_url).await?;

    let resp = ExpenseResponse::from(exp.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "upload_receipt", "expense",
        Some(&id), None, Some(&format!("{{\"receipt_url\":\"{receipt_url}\"}}")),
    ).await?;

    Ok(Json(resp))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("Invalid date format: {s}")))
}
