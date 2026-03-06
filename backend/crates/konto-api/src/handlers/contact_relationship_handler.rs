use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::contact_relationship_service::ContactRelationshipService;
use konto_core::services::contact_service::ContactService;

use crate::dto::contact_relationship::{
    ContactRelationshipResponse, CreateContactRelationshipRequest,
    UpdateContactRelationshipRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/contacts/{contact_id}/relationships",
    responses((status = 200, body = Vec<ContactRelationshipResponse>))
)]
pub async fn list_relationships(
    State(state): State<AppState>,
    Path(contact_id): Path<String>,
) -> Result<Json<Vec<ContactRelationshipResponse>>, AppError> {
    let relationships =
        ContactRelationshipService::list_for_contact(&state.db, &contact_id).await?;

    let mut results = Vec::with_capacity(relationships.len());
    for rel in relationships {
        let person = ContactService::get_by_id(&state.db, &rel.person_contact_id).await?;
        let org = ContactService::get_by_id(&state.db, &rel.org_contact_id).await?;
        results.push(ContactRelationshipResponse::from_model(
            rel,
            person.name1,
            org.name1,
        ));
    }

    Ok(Json(results))
}

#[utoipa::path(
    post, path = "/api/v1/contacts/{contact_id}/relationships",
    request_body = CreateContactRelationshipRequest,
    responses((status = 201, body = ContactRelationshipResponse))
)]
pub async fn create_relationship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(_contact_id): Path<String>,
    Json(body): Json<CreateContactRelationshipRequest>,
) -> Result<Json<ContactRelationshipResponse>, AppError> {
    let rel = ContactRelationshipService::create(
        &state.db,
        &body.person_contact_id,
        &body.org_contact_id,
        body.role,
        body.position,
        body.department,
        body.is_primary,
        body.notes,
    )
    .await?;

    let person = ContactService::get_by_id(&state.db, &rel.person_contact_id).await?;
    let org = ContactService::get_by_id(&state.db, &rel.org_contact_id).await?;
    let resp =
        ContactRelationshipResponse::from_model(rel.clone(), person.name1, org.name1);

    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "contact_relationship",
        Some(&rel.id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/contact-relationships/{id}",
    request_body = UpdateContactRelationshipRequest,
    responses((status = 200, body = ContactRelationshipResponse))
)]
pub async fn update_relationship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateContactRelationshipRequest>,
) -> Result<Json<ContactRelationshipResponse>, AppError> {
    let rel = ContactRelationshipService::update(
        &state.db,
        &id,
        body.role,
        body.position,
        body.department,
        body.is_primary,
        body.notes,
    )
    .await?;

    let person = ContactService::get_by_id(&state.db, &rel.person_contact_id).await?;
    let org = ContactService::get_by_id(&state.db, &rel.org_contact_id).await?;
    let resp =
        ContactRelationshipResponse::from_model(rel.clone(), person.name1, org.name1);

    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "update",
        "contact_relationship",
        Some(&id),
        None,
        new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/contact-relationships/{id}",
    responses((status = 204))
)]
pub async fn delete_relationship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ContactRelationshipService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "delete",
        "contact_relationship",
        Some(&id),
        None,
        None,
    )
    .await?;

    Ok(())
}
