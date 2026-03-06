use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::template_service::TemplateService;

use crate::dto::template::{
    CreateTemplateRequest, TemplateListParams, TemplateResponse, UpdateTemplateRequest,
};
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/templates",
    params(TemplateListParams),
    responses((status = 200, body = Vec<TemplateResponse>)),
    security(("bearer" = []))
)]
pub async fn list_templates(
    State(state): State<AppState>,
    Query(params): Query<TemplateListParams>,
) -> Result<Json<konto_common::pagination::PaginatedResponse<TemplateResponse>>, AppError> {
    let (templates, total) = TemplateService::list(
        &state.db,
        params.page(),
        params.per_page(),
        params.template_type.as_deref(),
        params.search.as_deref(),
    )
    .await?;

    let data = templates.into_iter().map(TemplateResponse::from).collect();
    Ok(Json(konto_common::pagination::PaginatedResponse::new(
        data,
        total,
        params.page(),
        params.per_page(),
    )))
}

#[utoipa::path(
    get, path = "/api/v1/templates/{id}",
    responses((status = 200, body = TemplateResponse)),
    security(("bearer" = []))
)]
pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TemplateResponse>, AppError> {
    let template = TemplateService::get_by_id(&state.db, &id).await?;
    Ok(Json(TemplateResponse::from(template)))
}

#[utoipa::path(
    post, path = "/api/v1/templates",
    request_body = CreateTemplateRequest,
    responses((status = 201, body = TemplateResponse)),
    security(("bearer" = []))
)]
pub async fn create_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateTemplateRequest>,
) -> Result<Json<TemplateResponse>, AppError> {
    let template = TemplateService::create(
        &state.db,
        &body.name,
        &body.template_type,
        &body.content_json,
        body.header_json,
        body.footer_json,
        body.page_setup_json,
        body.is_default,
        Some(claims.sub.clone()),
    )
    .await?;

    let resp = TemplateResponse::from(template.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "template",
        Some(&template.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/templates/{id}",
    request_body = UpdateTemplateRequest,
    responses((status = 200, body = TemplateResponse)),
    security(("bearer" = []))
)]
pub async fn update_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTemplateRequest>,
) -> Result<Json<TemplateResponse>, AppError> {
    let template = TemplateService::update(
        &state.db,
        &id,
        &body.name,
        &body.template_type,
        &body.content_json,
        body.header_json,
        body.footer_json,
        body.page_setup_json,
        body.is_default,
    )
    .await?;

    let resp = TemplateResponse::from(template.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "template",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    delete, path = "/api/v1/templates/{id}",
    responses((status = 204)),
    security(("bearer" = []))
)]
pub async fn delete_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    TemplateService::delete(&state.db, &id).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "delete", "template",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    post, path = "/api/v1/templates/{id}/duplicate",
    responses((status = 201, body = TemplateResponse)),
    security(("bearer" = []))
)]
pub async fn duplicate_template(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<TemplateResponse>, AppError> {
    let template = TemplateService::duplicate(
        &state.db,
        &id,
        Some(claims.sub.clone()),
    )
    .await?;

    let resp = TemplateResponse::from(template.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "duplicate", "template",
        Some(&template.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}
