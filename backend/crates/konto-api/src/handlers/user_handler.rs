use axum::extract::{Path, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_common::enums::UserRole;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::user_service::UserService;

use crate::dto::user::{
    ChangePasswordRequest, CreateUserRequest, RoleResponse, UpdateUserRequest, UserResponse,
};
use crate::state::AppState;

fn require_admin(claims: &Claims) -> Result<(), AppError> {
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    Ok(())
}

#[utoipa::path(
    get, path = "/api/v1/users",
    responses((status = 200, body = Vec<UserResponse>)),
    security(("bearer" = []))
)]
pub async fn list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    require_admin(&claims)?;
    let users = UserService::list(&state.db).await?;
    let data = users
        .into_iter()
        .map(|(u, role_name)| UserResponse::from_model(u, role_name))
        .collect();
    Ok(Json(data))
}

#[utoipa::path(
    get, path = "/api/v1/users/{id}",
    responses((status = 200, body = UserResponse)),
    security(("bearer" = []))
)]
pub async fn get_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, AppError> {
    require_admin(&claims)?;
    let (user, role_name) = UserService::get_by_id(&state.db, &id).await?;
    Ok(Json(UserResponse::from_model(user, role_name)))
}

#[utoipa::path(
    post, path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses((status = 201, body = UserResponse)),
    security(("bearer" = []))
)]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    require_admin(&claims)?;

    let user = UserService::create(
        &state.db,
        &body.email,
        &body.password,
        &body.full_name,
        &body.role_id,
        body.language.clone(),
    )
    .await?;

    let (user, role_name) = UserService::get_by_id(&state.db, &user.id).await?;
    let resp = UserResponse::from_model(user.clone(), role_name);
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "create", "user",
        Some(&user.id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/users/{id}",
    request_body = UpdateUserRequest,
    responses((status = 200, body = UserResponse)),
    security(("bearer" = []))
)]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    require_admin(&claims)?;

    let user = UserService::update(
        &state.db,
        &id,
        &body.email,
        &body.full_name,
        &body.role_id,
        body.is_active,
        body.language.clone(),
    )
    .await?;

    let (user, role_name) = UserService::get_by_id(&state.db, &user.id).await?;
    let resp = UserResponse::from_model(user, role_name);
    let new_vals = serde_json::to_string(&resp).ok();
    AuditService::log(
        &state.db, Some(&claims.sub), "update", "user",
        Some(&id), None, new_vals.as_deref(),
    )
    .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    put, path = "/api/v1/users/{id}/password",
    request_body = ChangePasswordRequest,
    responses((status = 200)),
    security(("bearer" = []))
)]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<(), AppError> {
    // Allow if admin or changing own password
    if claims.sub != id && claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Not authorized to change this password".to_string()));
    }

    UserService::change_password(&state.db, &id, &body.new_password).await?;

    AuditService::log(
        &state.db, Some(&claims.sub), "change_password", "user",
        Some(&id), None, None,
    )
    .await?;

    Ok(())
}

#[utoipa::path(
    get, path = "/api/v1/roles",
    responses((status = 200, body = Vec<RoleResponse>)),
    security(("bearer" = []))
)]
pub async fn list_roles(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<RoleResponse>>, AppError> {
    require_admin(&claims)?;
    let roles = UserService::list_roles(&state.db).await?;
    let data = roles.into_iter().map(RoleResponse::from).collect();
    Ok(Json(data))
}
