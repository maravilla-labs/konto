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

#[tauri::command]
pub fn reset_database(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("maravilla.db");

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
