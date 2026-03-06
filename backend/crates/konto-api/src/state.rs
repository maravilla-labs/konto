use crate::middleware::rate_limit::RateLimiter;
use konto_common::config::AppConfig;
use konto_core::auth::jwt::JwtService;
use konto_core::services::storage::StorageService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt: Arc<JwtService>,
    pub config: Arc<AppConfig>,
    pub storage: Arc<dyn StorageService>,
    pub login_limiter: RateLimiter,
    pub setup_limiter: RateLimiter,
}
