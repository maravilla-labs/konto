use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::Utc;
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_db::entities::contact_tag;
use konto_db::repository::contact_tag_repo::ContactTagRepo;
use sea_orm::Set;
use uuid::Uuid;

use crate::dto::contact_tag::{ContactTagResponse, CreateContactTagRequest};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/contact-tags",
    responses((status = 200, body = Vec<ContactTagResponse>)),
    security(("bearer" = []))
)]
pub async fn list_tags(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContactTagResponse>>, AppError> {
    let tags = ContactTagRepo::find_all(&state.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(Json(tags.into_iter().map(ContactTagResponse::from).collect()))
}

#[utoipa::path(
    post, path = "/api/v1/contact-tags",
    request_body = CreateContactTagRequest,
    responses((status = 201, body = ContactTagResponse)),
    security(("bearer" = []))
)]
pub async fn create_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateContactTagRequest>,
) -> Result<Json<ContactTagResponse>, AppError> {
    let now = Utc::now().naive_utc();
    let model = contact_tag::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(body.name),
        color: Set(body.color),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let tag = ContactTagRepo::create(&state.db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "create", "contact_tag",
        Some(&tag.id), None, None,
    ).await?;

    Ok(Json(ContactTagResponse::from(tag)))
}

#[utoipa::path(
    delete, path = "/api/v1/contact-tags/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    ContactTagRepo::find_by_id(&state.db, &id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    ContactTagRepo::delete(&state.db, &id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "contact_tag",
        Some(&id), None, None,
    ).await?;

    Ok(())
}

#[utoipa::path(
    put, path = "/api/v1/contacts/{id}/tags/{tag_id}",
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn assign_tag_to_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, tag_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    ContactTagRepo::assign_tag(&state.db, &id, &tag_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "assign_tag", "contact",
        Some(&id), None, Some(&format!("{{\"tag_id\":\"{tag_id}\"}}")),
    ).await?;

    Ok(Json(serde_json::json!({"message": "Tag assigned"})))
}

#[utoipa::path(
    delete, path = "/api/v1/contacts/{id}/tags/{tag_id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn remove_tag_from_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((id, tag_id)): Path<(String, String)>,
) -> Result<(), AppError> {
    ContactTagRepo::remove_tag(&state.db, &id, &tag_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    AuditService::log(
        &state.db, Some(&claims.sub), "remove_tag", "contact",
        Some(&id), None, Some(&format!("{{\"tag_id\":\"{tag_id}\"}}")),
    ).await?;

    Ok(())
}
