use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use konto_common::enums::{TokenType, UserRole};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;

use crate::state::AppState;

/// Extract JWT claims from the Authorization header.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

    let claims = state.jwt.verify_token(token)?;

    if claims.token_type != TokenType::Access {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

/// Extract claims from request extensions.
pub fn get_claims(request: &Request) -> Result<&Claims, AppError> {
    request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized("Not authenticated".to_string()))
}

/// Middleware: require admin role. Apply AFTER auth_middleware.
pub async fn require_admin(request: Request, next: Next) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Forbidden("Not authenticated".to_string()))?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    Ok(next.run(request).await)
}

/// Middleware: require admin or auditor role. Apply AFTER auth_middleware.
pub async fn require_admin_or_auditor(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Forbidden("Not authenticated".to_string()))?;
    if claims.role != UserRole::Admin && claims.role != UserRole::Auditor {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    Ok(next.run(request).await)
}
