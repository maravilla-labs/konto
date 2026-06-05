use konto_common::config::AppConfig;
use konto_core::services::storage::encrypted::{encrypt_existing_dir, EncryptedStorage};
use konto_core::services::storage::local::LocalStorage;
use konto_crypto::secret::resolve_jwt_secret;
use konto_crypto::{Dek, Keychain, KeyResolver, KeyStatus, DEFAULT_SERVICE};
use konto_db::connection::establish_sqlite;
use konto_db::encrypt_migrate::{encrypt_existing_database, is_plaintext_sqlite};
use konto_server::startup;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// Holds what the unlock/management commands need after `setup()` returns:
/// the leaked tokio runtime (to start the server later) and the data dir.
struct CryptoManager {
    rt: &'static tokio::runtime::Runtime,
    app_data_dir: PathBuf,
}

static MANAGER: OnceLock<CryptoManager> = OnceLock::new();

/// Result of the initial boot-time key resolution.
pub enum InitOutcome {
    /// Server is up on this port (env/keychain mode or first run).
    Ready(u16),
    /// Master-password mode: the DB stays locked until `unlock()` succeeds.
    Locked,
}

/// Current encryption state, surfaced to the frontend so it can render the
/// unlock gate and the security settings.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoStatus {
    /// One of: `env`, `keychain`, `password`, `uninitialized`.
    pub mode: String,
    /// Whether the database has been unlocked (server running).
    pub unlocked: bool,
    /// Server port once unlocked, else 0.
    pub port: u16,
}

fn mode_str(status: KeyStatus) -> &'static str {
    match status {
        KeyStatus::Env => "env",
        KeyStatus::Keychain => "keychain",
        KeyStatus::Uninitialized => "uninitialized",
        KeyStatus::PasswordLocked => "password",
    }
}

/// Resolve the encryption key at boot. In env/keychain/first-run modes this
/// unlocks transparently and starts the embedded server. In master-password
/// mode it returns `Locked` without touching the database.
pub fn init(rt: &'static tokio::runtime::Runtime, app_data_dir: PathBuf) -> InitOutcome {
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    let _ = MANAGER.set(CryptoManager { rt, app_data_dir: app_data_dir.clone() });

    let resolver = KeyResolver::new(&app_data_dir);
    match resolver.status().expect("Failed to read key status") {
        KeyStatus::PasswordLocked => {
            tracing::info!("Database is locked (master password required)");
            InitOutcome::Locked
        }
        _ => {
            let dek = resolver
                .resolve(None)
                .expect("Failed to resolve encryption key");
            let port = rt.block_on(start_with_dek(&app_data_dir, dek));
            InitOutcome::Ready(port)
        }
    }
}

/// Unlock with a master password and start the server. Returns the port.
pub fn unlock(password: &str) -> Result<u16, String> {
    let mgr = MANAGER.get().ok_or("crypto not initialized")?;
    let resolver = KeyResolver::new(&mgr.app_data_dir);
    let dek = resolver
        .resolve(Some(password))
        .map_err(|e| e.to_string())?;
    let port = mgr.rt.block_on(start_with_dek(&mgr.app_data_dir, dek));
    Ok(port)
}

/// Report the current encryption mode and unlock state.
pub fn status() -> Result<CryptoStatus, String> {
    let mgr = MANAGER.get().ok_or("crypto not initialized")?;
    let resolver = KeyResolver::new(&mgr.app_data_dir);
    let status = resolver.status().map_err(|e| e.to_string())?;
    let port = crate::commands::get_server_port();
    Ok(CryptoStatus { mode: mode_str(status).to_string(), unlocked: port != 0, port })
}

/// Enable a master password (keychain → password mode).
pub fn enable_password(password: &str) -> Result<(), String> {
    with_resolver(|r| r.enable_password(password))
}

/// Change the master password (password mode only).
pub fn change_password(old: &str, new: &str) -> Result<(), String> {
    with_resolver(|r| r.change_password(old, new))
}

/// Disable the master password (password → keychain mode).
pub fn disable_password(password: &str) -> Result<(), String> {
    with_resolver(|r| r.disable_password(password))
}

fn with_resolver<F>(f: F) -> Result<(), String>
where
    F: FnOnce(&KeyResolver) -> konto_crypto::Result<()>,
{
    let mgr = MANAGER.get().ok_or("crypto not initialized")?;
    let resolver = KeyResolver::new(&mgr.app_data_dir);
    f(&resolver).map_err(|e| e.to_string())
}

/// Build the encrypted DB + storage with a resolved DEK, run migrations, and
/// start the embedded Axum server. Returns the listening port.
async fn start_with_dek(app_data_dir: &Path, dek: Dek) -> u16 {
    let db_path = app_data_dir.join("maravilla.db");

    // One-time: encrypt a pre-existing plaintext database (upgrade path).
    if is_plaintext_sqlite(&db_path) {
        encrypt_existing_database(&db_path, &dek)
            .await
            .expect("Failed to encrypt existing database");
    }

    let db = establish_sqlite(&db_path, Some(&dek))
        .await
        .expect("Failed to connect to encrypted database");

    // Encrypted file storage for uploads (receipts, attachments, avatars).
    let uploads_dir = app_data_dir.join("uploads");
    std::fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");
    if let Err(e) = encrypt_existing_dir(&uploads_dir, &dek) {
        tracing::error!("Failed to encrypt existing uploads: {e}");
    }
    let inner = Arc::new(LocalStorage::new(
        uploads_dir.to_str().unwrap_or("./data/uploads"),
    ));
    let storage = Arc::new(EncryptedStorage::new(inner, dek));

    // JWT secret lives in the OS keychain (migrated from secret.key if present).
    let keychain = Keychain::new(DEFAULT_SERVICE);
    let secret_path = app_data_dir.join("secret.key");
    let jwt_secret =
        resolve_jwt_secret(&keychain, &secret_path).expect("Failed to resolve JWT secret");

    let config = AppConfig {
        database_url: format!("sqlite:{}?mode=rwc", db_path.display()),
        jwt_secret,
        jwt_access_expires_secs: 900,
        jwt_refresh_expires_secs: 604800,
        server_host: "127.0.0.1".to_string(),
        server_port: 0,
        // Webview origin differs per platform:
        //   macOS/Linux: tauri://localhost
        //   Windows (WebView2): http://tauri.localhost
        //   iOS:        https://tauri.localhost
        // http://localhost:5173 is the Vite dev server.
        cors_origin: "tauri://localhost,http://tauri.localhost,https://tauri.localhost,http://localhost:5173".to_string(),
    };

    let state = startup::build_state(&config, storage, db).await;
    startup::spawn_schedulers(&state.db);

    let app = startup::build_app(state, &config.cors_origin);
    let port = startup::serve(app, "127.0.0.1", 0).await;

    tracing::info!("Embedded server started on port {port}");
    port
}
