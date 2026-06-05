use std::sync::atomic::{AtomicU16, Ordering};
use tauri::Manager;

static SERVER_PORT: AtomicU16 = AtomicU16::new(0);

pub fn set_server_port(port: u16) {
    SERVER_PORT.store(port, Ordering::Relaxed);
}

#[tauri::command]
pub fn get_server_port() -> u16 {
    SERVER_PORT.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ---- Encryption at rest ----

/// Current encryption mode and unlock state (drives the unlock gate + settings).
#[tauri::command]
pub fn crypto_status() -> Result<crate::server::CryptoStatus, String> {
    crate::server::status()
}

/// Unlock the database with the master password and start the server.
#[tauri::command]
pub fn crypto_unlock(password: String) -> Result<u16, String> {
    let port = crate::server::unlock(&password)?;
    set_server_port(port);
    Ok(port)
}

/// Enable a master password (switches from transparent keychain mode).
#[tauri::command]
pub fn crypto_enable_password(password: String) -> Result<(), String> {
    crate::server::enable_password(&password)
}

/// Change the master password.
#[tauri::command]
pub fn crypto_change_password(old_password: String, new_password: String) -> Result<(), String> {
    crate::server::change_password(&old_password, &new_password)
}

/// Disable the master password (back to transparent keychain mode).
#[tauri::command]
pub fn crypto_disable_password(password: String) -> Result<(), String> {
    crate::server::disable_password(&password)
}

#[tauri::command]
pub fn reset_database(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    // Also remove SQLite WAL/SHM files if present
    for ext in &["", "-wal", "-shm"] {
        let p = app_data_dir.join(format!("maravilla.db{ext}"));
        if p.exists() {
            let _ = std::fs::remove_file(&p);
        }
    }

    // Restart the app so the server reinitializes with a fresh DB
    app.restart();
}
