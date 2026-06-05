use axum::http::HeaderValue;
use axum::Router;
use konto_api::middleware::rate_limit::RateLimiter;
use konto_api::state::AppState;
use konto_common::config::AppConfig;
use konto_core::auth::jwt::JwtService;
use konto_core::services::dunning_service::DunningService;
use konto_core::services::recurring_invoice_service::RecurringInvoiceService;
use konto_core::services::scheduler_service::SchedulerService;
use konto_core::services::storage::StorageService;
use konto_crypto::{Dek, KeyResolver, ENV_MASTER_KEY};
use konto_db::connection::{establish_connection, establish_sqlite};
use konto_db::encrypt_migrate::{encrypt_existing_database, is_plaintext_sqlite};
use konto_migration::Migrator;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::ApiDoc;
use crate::router::build_router;

/// Extract the file path from a `sqlite:` URL, or `None` for in-memory or
/// non-SQLite (e.g. PostgreSQL) URLs.
fn sqlite_file_path(database_url: &str) -> Option<PathBuf> {
    let rest = database_url.strip_prefix("sqlite:")?;
    let rest = rest.strip_prefix("//").unwrap_or(rest);
    let path = rest.split('?').next().unwrap_or(rest);
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(PathBuf::from(path))
}

/// Establish the database connection for a given config, honoring SQLCipher
/// encryption at rest. For `sqlite:` file URLs the DEK is resolved from
/// `KONTO_MASTER_KEY` / keychain (via the file's parent dir) when available; a
/// missing key falls back to an unencrypted connection. Non-SQLite URLs connect
/// unchanged (encryption handled by the server/infra).
#[allow(clippy::expect_used)]
pub async fn connect_database(config: &AppConfig) -> DatabaseConnection {
    match sqlite_file_path(&config.database_url) {
        Some(path) => {
            // Only attempt key resolution on the server path when an explicit
            // env key is provided; otherwise stay unencrypted for compatibility.
            let dek: Option<Dek> = if std::env::var(ENV_MASTER_KEY).is_ok() {
                let dir = path.parent().map(PathBuf::from).unwrap_or_default();
                Some(
                    KeyResolver::new(dir)
                        .resolve(None)
                        .expect("Failed to resolve database encryption key"),
                )
            } else {
                None
            };
            // Encrypt a pre-existing plaintext database on first keyed launch.
            if let Some(dek) = dek.as_ref() {
                if is_plaintext_sqlite(&path) {
                    encrypt_existing_database(&path, dek)
                        .await
                        .expect("Failed to encrypt existing database");
                }
            }
            establish_sqlite(&path, dek.as_ref())
                .await
                .expect("Failed to connect to SQLite database")
        }
        None => establish_connection(&config.database_url)
            .await
            .expect("Failed to connect to database"),
    }
}

/// Build the application state from an established DB connection: run
/// migrations and init JWT. The caller establishes the connection (encrypted or
/// not) so it controls key resolution; see [`connect_database`].
#[allow(clippy::expect_used)]
pub async fn build_state(
    config: &AppConfig,
    storage: Arc<dyn StorageService>,
    db: DatabaseConnection,
) -> AppState {
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations completed successfully");

    let jwt = Arc::new(JwtService::new(
        &config.jwt_secret,
        config.jwt_access_expires_secs,
        config.jwt_refresh_expires_secs,
    ));

    const LOGIN_MAX_REQUESTS: usize = 10;
    const LOGIN_WINDOW_SECS: u64 = 60;
    const SETUP_MAX_REQUESTS: usize = 3;
    const SETUP_WINDOW_SECS: u64 = 60;

    AppState {
        db,
        jwt,
        config: Arc::new(config.clone()),
        storage,
        login_limiter: RateLimiter::new(LOGIN_MAX_REQUESTS, LOGIN_WINDOW_SECS),
        setup_limiter: RateLimiter::new(SETUP_MAX_REQUESTS, SETUP_WINDOW_SECS),
    }
}

/// Build the Axum router with CORS, tracing, and Swagger UI.
pub fn build_app(state: AppState, cors_origin: &str) -> Router {
    let origins: Vec<axum::http::HeaderValue> = cors_origin
        .split(',')
        .filter_map(|o| o.trim().parse().ok())
        .collect();

    // SAFETY: this is a compile-time-known valid header value
    const DEFAULT_ORIGIN: HeaderValue = HeaderValue::from_static("http://localhost:5173");

    let cors = if origins.is_empty() {
        CorsLayer::new()
            .allow_origin(DEFAULT_ORIGIN)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    build_router(state)
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

/// Spawn the 3 background scheduler tasks.
pub fn spawn_schedulers(db: &DatabaseConnection) {
    let scheduler_db = db.clone();
    let overdue_db = db.clone();
    let dunning_db = db.clone();

    const RECURRING_INVOICE_INTERVAL: Duration = Duration::from_secs(3600);
    const OVERDUE_CHECK_INTERVAL: Duration = Duration::from_secs(86_400);
    const DUNNING_INTERVAL: Duration = Duration::from_secs(86_400);

    // Recurring invoice scheduler (hourly)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(RECURRING_INVOICE_INTERVAL);
        loop {
            interval.tick().await;
            match RecurringInvoiceService::generate_due_invoices(&scheduler_db).await {
                Ok(count) if count > 0 => {
                    tracing::info!("Generated {count} recurring invoice(s)");
                }
                Err(e) => {
                    tracing::error!("Recurring invoice generation failed: {e}");
                }
                _ => {}
            }
        }
    });

    // Overdue invoice detection scheduler (daily)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(OVERDUE_CHECK_INTERVAL);
        loop {
            interval.tick().await;
            match SchedulerService::check_overdue(&overdue_db).await {
                Ok(count) if count > 0 => {
                    tracing::info!("Marked {count} invoice(s) as overdue");
                }
                Err(e) => {
                    tracing::error!("Overdue check failed: {e}");
                }
                _ => {}
            }
        }
    });

    // Dunning scheduler (daily)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(DUNNING_INTERVAL);
        interval.tick().await; // skip first immediate tick
        loop {
            interval.tick().await;
            match DunningService::run_dunning(&dunning_db).await {
                Ok(result) if result.reminders_sent > 0 => {
                    tracing::info!("Dunning run: {} reminders sent", result.reminders_sent);
                }
                Err(e) => {
                    tracing::error!("Dunning run failed: {e}");
                }
                _ => {}
            }
        }
    });
}

/// Bind the app to a TCP listener and serve. Returns the actual port.
#[allow(clippy::expect_used, clippy::unwrap_used)]
pub async fn serve(app: Router, host: &str, port: u16) -> u16 {
    let addr = format!("{host}:{port}");
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    let actual_port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("Server error");
    });

    actual_port
}

/// Run the server in standalone mode (blocks until shutdown).
#[allow(clippy::expect_used)]
pub async fn run_standalone() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env();
    let storage = Arc::new(
        konto_core::services::storage::local::LocalStorage::new("./data/uploads"),
    );

    let db = connect_database(&config).await;
    let state = build_state(&config, storage, db).await;
    spawn_schedulers(&state.db);

    let app = build_app(state, &config.cors_origin);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
