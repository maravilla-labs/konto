use konto_common::config::AppConfig;
use konto_core::services::storage::local::LocalStorage;
use konto_server::startup;
use std::path::PathBuf;
use std::sync::Arc;

/// Start the embedded Axum server for the Tauri desktop app.
/// Returns the port the server is listening on.
pub async fn start_embedded_server(app_data_dir: PathBuf) -> u16 {
    // Ensure data directory exists
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    let db_path = app_data_dir.join("maravilla.db");
    let uploads_dir = app_data_dir.join("uploads");
    std::fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");

    let secret_path = app_data_dir.join("secret.key");
    let jwt_secret = load_or_create_secret(&secret_path);

    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let config = AppConfig {
        database_url: db_url,
        jwt_secret,
        jwt_access_expires_secs: 900,
        jwt_refresh_expires_secs: 604800,
        server_host: "127.0.0.1".to_string(),
        server_port: 0, // OS picks free port
        cors_origin: "tauri://localhost,https://tauri.localhost,http://localhost:5173".to_string(),
    };

    let storage = Arc::new(LocalStorage::new(
        uploads_dir.to_str().unwrap_or("./data/uploads"),
    ));

    let state = startup::build_state(&config, storage).await;
    startup::spawn_schedulers(&state.db);

    let app = startup::build_app(state, &config.cors_origin);
    let port = startup::serve(app, "127.0.0.1", 0).await;

    tracing::info!("Embedded server started on port {port}");
    port
}

fn load_or_create_secret(path: &PathBuf) -> String {
    if let Ok(secret) = std::fs::read_to_string(path) {
        if !secret.trim().is_empty() {
            return secret.trim().to_string();
        }
    }

    use rand::Rng;
    let secret: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    std::fs::write(path, &secret).expect("Failed to write secret key");
    secret
}
