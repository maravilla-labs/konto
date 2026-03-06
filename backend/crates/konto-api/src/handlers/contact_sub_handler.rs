use axum::extract::{Path, Query, State};
use axum::Json;
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_db::entities::{document, invoice, time_entry};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::dto::invoice::InvoiceResponse;
use crate::dto::time_entry::TimeEntryResponse;
use crate::state::AppState;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct DocumentSummaryResponse {
    pub id: String,
    pub doc_type: String,
    pub doc_number: Option<String>,
    pub title: String,
    pub status: String,
    #[schema(value_type = String)]
    pub total: rust_decimal::Decimal,
}

impl From<document::Model> for DocumentSummaryResponse {
    fn from(m: document::Model) -> Self {
        Self {
            id: m.id,
            doc_type: m.doc_type,
            doc_number: m.doc_number,
            title: m.title,
            status: m.status,
            total: m.total,
        }
    }
}

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/invoices",
    params(PaginationParams),
    responses((status = 200, body = Vec<InvoiceResponse>)),
    security(("bearer" = []))
)]
pub async fn list_contact_invoices(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<InvoiceResponse>>, AppError> {
    let (items, total) = find_invoices_by_contact(
        &state.db, &id, params.page(), params.per_page(),
    ).await?;
    let data = items.into_iter().map(InvoiceResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, params.page(), params.per_page())))
}

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/documents",
    params(PaginationParams),
    responses((status = 200, body = Vec<DocumentSummaryResponse>)),
    security(("bearer" = []))
)]
pub async fn list_contact_documents(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<DocumentSummaryResponse>>, AppError> {
    let (items, total) = find_documents_by_contact(
        &state.db, &id, params.page(), params.per_page(),
    ).await?;
    let data = items.into_iter().map(DocumentSummaryResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, params.page(), params.per_page())))
}

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/time-entries",
    params(PaginationParams),
    responses((status = 200, body = Vec<TimeEntryResponse>)),
    security(("bearer" = []))
)]
pub async fn list_contact_time_entries(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<TimeEntryResponse>>, AppError> {
    let (items, total) = find_time_entries_by_contact(
        &state.db, &id, params.page(), params.per_page(),
    ).await?;
    let data = items.into_iter().map(TimeEntryResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, params.page(), params.per_page())))
}

async fn find_invoices_by_contact(
    db: &DatabaseConnection,
    contact_id: &str,
    page: u64,
    per_page: u64,
) -> Result<(Vec<invoice::Model>, u64), AppError> {
    let query = invoice::Entity::find()
        .filter(invoice::Column::ContactId.eq(contact_id))
        .order_by_desc(invoice::Column::IssueDate);
    let paginator = query.paginate(db, per_page);
    let total = paginator.num_items().await.map_err(|e| AppError::Database(e.to_string()))?;
    let items = paginator.fetch_page(page.saturating_sub(1)).await.map_err(|e| AppError::Database(e.to_string()))?;
    Ok((items, total))
}

async fn find_documents_by_contact(
    db: &DatabaseConnection,
    contact_id: &str,
    page: u64,
    per_page: u64,
) -> Result<(Vec<document::Model>, u64), AppError> {
    let query = document::Entity::find()
        .filter(document::Column::ContactId.eq(contact_id))
        .order_by_desc(document::Column::UpdatedAt);
    let paginator = query.paginate(db, per_page);
    let total = paginator.num_items().await.map_err(|e| AppError::Database(e.to_string()))?;
    let items = paginator.fetch_page(page.saturating_sub(1)).await.map_err(|e| AppError::Database(e.to_string()))?;
    Ok((items, total))
}

async fn find_time_entries_by_contact(
    db: &DatabaseConnection,
    contact_id: &str,
    page: u64,
    per_page: u64,
) -> Result<(Vec<time_entry::Model>, u64), AppError> {
    let query = time_entry::Entity::find()
        .filter(time_entry::Column::ContactId.eq(contact_id))
        .order_by_desc(time_entry::Column::Date);
    let paginator = query.paginate(db, per_page);
    let total = paginator.num_items().await.map_err(|e| AppError::Database(e.to_string()))?;
    let items = paginator.fetch_page(page.saturating_sub(1)).await.map_err(|e| AppError::Database(e.to_string()))?;
    Ok((items, total))
}
