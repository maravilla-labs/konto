use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_db::entities::contact_person;
use konto_db::repository::contact_person_repo::ContactPersonRepo;
use sea_orm::Set;
use uuid::Uuid;

use crate::dto::contact_person::{
    ContactPersonResponse, CreateContactPersonRequest, UpdateContactPersonRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/contacts/{id}/persons",
    responses((status = 200, body = Vec<ContactPersonResponse>)),
    security(("bearer" = []))
)]
pub async fn list_by_contact(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<ContactPersonResponse>>, AppError> {
    let persons = ContactPersonRepo::find_by_contact_id(&state.db, &id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(Json(persons.into_iter().map(ContactPersonResponse::from).collect()))
}

#[utoipa::path(
    post, path = "/api/v1/contacts/{id}/persons",
    request_body = CreateContactPersonRequest,
    responses((status = 201, body = ContactPersonResponse)),
    security(("bearer" = []))
)]
pub async fn create_person(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<CreateContactPersonRequest>,
) -> Result<Json<ContactPersonResponse>, AppError> {
    let model = contact_person::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        contact_id: Set(id.clone()),
        first_name: Set(body.first_name),
        last_name: Set(body.last_name),
        email: Set(body.email),
        phone: Set(body.phone),
        department: Set(body.department),
        position: Set(body.position),
    };

    let person = ContactPersonRepo::create(&state.db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "create", "contact_person",
        Some(&person.id), None, None,
    ).await?;

    Ok(Json(ContactPersonResponse::from(person)))
}

#[utoipa::path(
    put, path = "/api/v1/contacts/{id}/persons/{person_id}",
    request_body = UpdateContactPersonRequest,
    responses((status = 200, body = ContactPersonResponse)),
    security(("bearer" = []))
)]
pub async fn update_person(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_id, person_id)): Path<(String, String)>,
    Json(body): Json<UpdateContactPersonRequest>,
) -> Result<Json<ContactPersonResponse>, AppError> {
    let existing = ContactPersonRepo::find_by_id(&state.db, &person_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Contact person not found".to_string()))?;

    let mut model: contact_person::ActiveModel = existing.into();
    if let Some(f) = body.first_name { model.first_name = Set(f); }
    if let Some(l) = body.last_name { model.last_name = Set(l); }
    if let Some(e) = body.email { model.email = Set(e); }
    if let Some(p) = body.phone { model.phone = Set(p); }
    if let Some(d) = body.department { model.department = Set(d); }
    if let Some(p) = body.position { model.position = Set(p); }

    let person = ContactPersonRepo::update(&state.db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "update", "contact_person",
        Some(&person_id), None, None,
    ).await?;

    Ok(Json(ContactPersonResponse::from(person)))
}

#[utoipa::path(
    delete, path = "/api/v1/contacts/{id}/persons/{person_id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_person(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_id, person_id)): Path<(String, String)>,
) -> Result<(), AppError> {
    ContactPersonRepo::find_by_id(&state.db, &person_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Contact person not found".to_string()))?;

    ContactPersonRepo::delete(&state.db, &person_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "contact_person",
        Some(&person_id), None, None,
    ).await?;

    Ok(())
}
