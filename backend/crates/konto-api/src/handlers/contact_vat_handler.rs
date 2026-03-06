use axum::extract::{Path, State};
use axum::Json;
use konto_common::error::AppError;
use konto_core::services::contact_service::ContactService;
use konto_core::services::vat_resolution_service::VatResolutionService;
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Debug, Serialize, ToSchema)]
pub struct VatInfoResponse {
    pub vat_mode: String,
    pub resolved_mode: String,
    pub default_vat_rate_id: Option<String>,
}

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/vat-info",
    responses((status = 200, body = VatInfoResponse)),
    security(("bearer" = [])),
    tag = "contacts"
)]
pub async fn get_vat_info(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<VatInfoResponse>, AppError> {
    let contact = ContactService::get_by_id(&state.db, &id).await?;
    let resolved_mode = VatResolutionService::resolve_vat_mode(&contact);
    let default_vat_rate_id =
        VatResolutionService::default_vat_rate_for_mode(&state.db, &resolved_mode).await?;

    Ok(Json(VatInfoResponse {
        vat_mode: contact.vat_mode,
        resolved_mode,
        default_vat_rate_id,
    }))
}
