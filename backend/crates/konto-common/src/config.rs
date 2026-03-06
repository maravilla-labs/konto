use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expires_secs: u64,
    pub jwt_refresh_expires_secs: u64,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origin: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:./hope.db?mode=rwc".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| {
                use std::collections::hash_map::RandomState;
                use std::hash::{BuildHasher, Hasher};
                let mut parts = [0u64; 4];
                for part in &mut parts {
                    let s = RandomState::new();
                    let mut h = s.build_hasher();
                    h.write_u128(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos(),
                    );
                    *part = h.finish();
                }
                tracing::warn!("JWT_SECRET not set — using random secret. Tokens will not survive restart.");
                format!("{:016x}{:016x}{:016x}{:016x}", parts[0], parts[1], parts[2], parts[3])
            }),
            jwt_access_expires_secs: std::env::var("JWT_ACCESS_EXPIRES_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(900),
            jwt_refresh_expires_secs: std::env::var("JWT_REFRESH_EXPIRES_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(604800),
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        }
    }
}
