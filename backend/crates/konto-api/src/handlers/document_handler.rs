use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::document_service::{DocumentService, LineInput};
use konto_core::services::pdf_document::PdfDocumentService;

use crate::dto::document::*;
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/documents",
    params(DocumentListParams),
    responses((status = 200, body = Vec<DocumentResponse>)),
    security(("bearer" = []))
)]
pub async fn list_documents(
    State(state): State<AppState>,
    Query(params): Query<DocumentListParams>,
) -> Result<Json<konto_common::pagination::PaginatedResponse<DocumentResponse>>, AppError> {
    let (docs, total) = DocumentService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.doc_type.as_deref(),
        params.status.as_deref(),
        params.contact_id.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    let data = docs.into_iter().map(DocumentResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data, total, params.page(), params.per_page(),
    )))
}

#[utoipa::path(
    get, path = "/api/v1/documents/{id}",
    responses((status = 200, body = DocumentDetailResponse)),
    security(("bearer" = []))
)]
pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DocumentDetailResponse>, AppError> {
    let detail = DocumentService::get_by_id(&state.db, &id).await?;
    Ok(Json(DocumentDetailResponse {
        content_json: detail.document.content_json.clone(),
        document: DocumentResponse::from(detail.document),
        lines: detail.lines.into_iter().map(DocumentLineItemResponse::from).collect(),
        contact_name: detail.contact_name,
        project_name: detail.project_name,
    }))
}

#[utoipa::path(
    post, path = "/api/v1/documents",
    request_body = CreateDocumentRequest,
    responses((status = 201, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn create_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateDocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    let valid_until = body.valid_until.as_deref().map(parse_date).transpose()?;
    let lines = map_line_inputs(body.lines);

    let doc = DocumentService::create(
        &state.db, &body.doc_type, &body.title, &body.contact_id,
        body.project_id, body.template_id, &body.content_json,
        body.language, body.currency_id, valid_until, lines, Some(claims.sub.clone()),
    )
    .await?;

    let resp = DocumentResponse::from(doc.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "document",
        Some(&doc.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/documents/{id}",
    request_body = UpdateDocumentRequest,
    responses((status = 200, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn update_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateDocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    let valid_until = body.valid_until.as_deref().map(parse_date).transpose()?;
    let lines = map_line_inputs(body.lines);

    let doc = DocumentService::update(
        &state.db, &id, &body.title, &body.contact_id,
        body.project_id, body.template_id, &body.content_json,
        body.language, body.currency_id, valid_until, lines,
    )
    .await?;

    let resp = DocumentResponse::from(doc.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "document",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/documents/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    DocumentService::delete(&state.db, &id).await?;
    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "document",
        Some(&id), None, None,
    )
    .await?;
    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/documents/{id}/send",
    responses((status = 200, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn send_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<DocumentResponse>, AppError> {
    let doc = DocumentService::send_document(&state.db, &id, &claims.sub).await?;
    let resp = DocumentResponse::from(doc.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "send", "document",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;
    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/documents/{id}/accept",
    responses((status = 200, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn accept_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<DocumentResponse>, AppError> {
    let doc = DocumentService::accept_document(&state.db, &id, &claims.sub).await?;
    let resp = DocumentResponse::from(doc.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "accept", "document",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;
    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/documents/{id}/reject",
    responses((status = 200, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn reject_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<DocumentResponse>, AppError> {
    let doc = DocumentService::reject_document(&state.db, &id, &claims.sub).await?;
    let resp = DocumentResponse::from(doc.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "reject", "document",
        Some(&id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;
    Ok(Json(resp))
}

#[utoipa::path(
    post, path = "/api/v1/documents/{id}/convert",
    request_body = ConvertDocumentRequest,
    responses((status = 201, body = DocumentResponse)),
    security(("bearer" = []))
)]
pub async fn convert_document(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<ConvertDocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    let doc = DocumentService::convert_document(
        &state.db, &id, &body.target_type, &claims.sub,
    )
    .await?;

    let resp = DocumentResponse::from(doc.clone());
    AuditService::log(
        &state.db, Some(&claims.sub), "convert", "document",
        Some(&doc.id), None, serde_json::to_string(&resp).ok().as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/documents/{id}/pdf",
    responses((status = 200, description = "PDF file", content_type = "application/pdf")),
    security(("bearer" = []))
)]
pub async fn get_document_pdf(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pdf_bytes = PdfDocumentService::generate(&state.db, &id).await?;
    let detail = DocumentService::get_by_id(&state.db, &id).await?;
    let doc_number = detail.document.doc_number.as_deref().unwrap_or("draft");
    let filename = format!("attachment; filename=\"document-{}.pdf\"", doc_number);
    Ok((
        [
            (axum::http::header::CONTENT_TYPE.as_str(), "application/pdf".to_string()),
            (axum::http::header::CONTENT_DISPOSITION.as_str(), filename),
        ],
        pdf_bytes,
    ))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, AppError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("Invalid date format: {s}")))
}

fn map_line_inputs(lines: Vec<CreateDocumentLineRequest>) -> Vec<LineInput> {
    lines
        .into_iter()
        .map(|l| LineInput {
            description: l.description,
            quantity: l.quantity,
            unit: l.unit,
            unit_price: l.unit_price,
            discount_pct: l.discount_pct,
        })
        .collect()
}
