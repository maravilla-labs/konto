use axum::extract::{Multipart, State};
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::auth_service::AuthService;
use std::path::Path;
use tokio::fs;

use crate::dto::auth::{
    LoginRequest, MeResponse, RefreshRequest, TokenResponse, UpdateMyLanguageRequest,
    UpdateMyProfileRequest,
};
use crate::middleware::upload::validate_image_upload;
use crate::state::AppState;

#[utoipa::path(
    post, path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses((status = 200, body = TokenResponse))
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    state.login_limiter.check(&body.email).await?;

    let tokens = AuthService::login(&state.db, &state.jwt, &body.email, &body.password).await?;

    Ok(Json(TokenResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses((status = 200, body = TokenResponse))
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let tokens = AuthService::refresh(&state.db, &state.jwt, &body.refresh_token).await?;

    Ok(Json(TokenResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
    }))
}

#[utoipa::path(
    get, path = "/api/v1/auth/me",
    responses((status = 200, body = MeResponse)),
    security(("bearer" = []))
)]
pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, AppError> {
    let user = AuthService::get_current_user(&state.db, &claims.sub).await?;
    Ok(Json(to_me_response(user)))
}

#[utoipa::path(
    put, path = "/api/v1/auth/me",
    request_body = UpdateMyProfileRequest,
    responses((status = 200, body = MeResponse)),
    security(("bearer" = []))
)]
pub async fn update_my_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateMyProfileRequest>,
) -> Result<Json<MeResponse>, AppError> {
    let user =
        AuthService::update_profile(&state.db, &claims.sub, body.full_name, body.language).await?;
    Ok(Json(to_me_response(user)))
}

#[utoipa::path(
    put, path = "/api/v1/auth/me/language",
    request_body = UpdateMyLanguageRequest,
    responses((status = 200, body = MeResponse)),
    security(("bearer" = []))
)]
pub async fn update_my_language(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateMyLanguageRequest>,
) -> Result<Json<MeResponse>, AppError> {
    let user = AuthService::set_language(&state.db, &claims.sub, &body.language).await?;
    Ok(Json(to_me_response(user)))
}

#[utoipa::path(
    post, path = "/api/v1/auth/me/avatar",
    responses((status = 200, body = MeResponse)),
    security(("bearer" = []))
)]
pub async fn upload_my_avatar(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<MeResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart: {e}")))?
        .ok_or_else(|| AppError::BadRequest("No file uploaded".to_string()))?;

    let file_name = field
        .file_name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "avatar.png".to_string());

    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read file data: {e}")))?;

    let mime_type = file_name
        .rsplit('.')
        .next()
        .map(|ext| match ext.to_lowercase().as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "webp" => "image/webp",
            _ => "application/octet-stream",
        })
        .unwrap_or("application/octet-stream");
    validate_image_upload(&file_name, mime_type, &data)?;

    let upload_dir = Path::new("uploads").join("avatars");
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create uploads dir: {e}")))?;

    let ext = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let stored_name = format!("{}.{}", claims.sub, ext);
    let file_path = upload_dir.join(&stored_name);

    fs::write(&file_path, &data)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to save avatar: {e}")))?;

    let avatar_url = format!("/uploads/avatars/{stored_name}");
    let user = AuthService::update_avatar_url(&state.db, &claims.sub, Some(avatar_url)).await?;
    Ok(Json(to_me_response(user)))
}

fn to_me_response(user: konto_core::services::auth_service::AuthUser) -> MeResponse {
    MeResponse {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        language: user.language,
        avatar_url: user.avatar_url,
        role: user.role,
    }
}
