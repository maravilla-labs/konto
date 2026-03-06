use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::pagination::{PaginatedResponse, PaginationParams};
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::contact_service::ContactService;
use konto_core::services::contact_relationship_service::ContactRelationshipService;
use konto_core::services::export_service::ExportService;

use crate::dto::contact::{ContactResponse, CreateContactRequest, UpdateContactRequest};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/contacts",
    params(PaginationParams),
    responses((status = 200, body = Vec<ContactResponse>))
)]
pub async fn list_contacts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.format.as_deref() == Some("csv") {
        let (contacts, _) = ContactService::list(
            &state.db, 1, u64::MAX, params.search.as_deref(), params.category.as_deref(),
        ).await?;
        let data: Vec<ContactResponse> = contacts.into_iter().map(ContactResponse::from).collect();
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok((
            [(header::CONTENT_TYPE, "text/csv".to_string()),
             (header::CONTENT_DISPOSITION, "attachment; filename=\"contacts.csv\"".to_string())],
            csv_bytes,
        ).into_response());
    }

    let (contacts, total) = ContactService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.search.as_deref(),
        params.category.as_deref(),
    )
    .await?;

    let data = contacts.into_iter().map(ContactResponse::from).collect();
    Ok(Json(PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )).into_response())
}

#[utoipa::path(
    post, path = "/api/v1/contacts",
    request_body = CreateContactRequest,
    responses((status = 201, body = ContactResponse))
)]
pub async fn create_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    let contact = ContactService::create(
        &state.db,
        &body.contact_type,
        &body.name1,
        body.name2,
        body.email,
        body.phone,
        body.address,
        body.postal_code,
        body.city,
        body.country,
        body.language,
        body.notes,
        body.salutation,
        body.title,
        body.email2,
        body.phone2,
        body.mobile,
        body.fax,
        body.industry,
        body.birthday,
        body.employee_count,
        body.trade_register_number,
        body.salutation_form,
        body.website,
        body.vat_number,
        body.category,
        body.customer_number,
        body.vat_mode,
    )
    .await?;

    let resp = ContactResponse::from(contact.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "contact",
        Some(&contact.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}",
    responses((status = 200, body = ContactResponse))
)]
pub async fn get_contact(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ContactResponse>, AppError> {
    let contact = ContactService::get_by_id(&state.db, &id).await?;
    Ok(Json(ContactResponse::from(contact)))
}

#[utoipa::path(
    put, path = "/api/v1/contacts/{id}",
    request_body = UpdateContactRequest,
    responses((status = 200, body = ContactResponse))
)]
pub async fn update_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    let contact = ContactService::update(
        &state.db, &id, body.name1, body.contact_type, body.name2, body.email, body.phone,
        body.city, body.country, body.language, body.is_active, body.notes, body.address,
        body.postal_code, body.website, body.vat_number, body.salutation, body.title,
        body.email2, body.phone2, body.mobile, body.fax, body.industry, body.birthday,
        body.employee_count, body.trade_register_number, body.salutation_form, body.category,
        body.customer_number, body.vat_mode,
    )
    .await?;

    let resp = ContactResponse::from(contact.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "contact",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/contacts/{id}",
    responses((status = 204))
)]
pub async fn delete_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ContactService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "contact",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

/// List persons linked to a company contact via contact_relationships.
#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/persons-via-relationships",
    responses((status = 200, body = Vec<ContactResponse>)),
    security(("bearer" = [])),
    tag = "contacts"
)]
pub async fn list_contact_persons_via_relationships(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<ContactResponse>>, AppError> {
    // Verify contact exists
    ContactService::get_by_id(&state.db, &id).await?;

    let relationships = ContactRelationshipService::list_for_contact(&state.db, &id).await?;
    let mut persons = Vec::new();

    for rel in &relationships {
        if rel.org_contact_id == id
            && let Ok(person) = ContactService::get_by_id(&state.db, &rel.person_contact_id).await
        {
            persons.push(ContactResponse::from(person));
        }
    }

    Ok(Json(persons))
}
