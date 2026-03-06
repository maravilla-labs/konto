// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod server;

use commands::{get_app_version, get_server_port, reset_database, set_server_port};
use tauri::Manager;
use tauri::PhysicalSize;

fn main() {
    // Initialize tracing for the Tauri process
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("maravilla_konto=info".parse().unwrap()),
        )
        .init();

    // Leak the runtime so it outlives the Tauri event loop.
    // The embedded Axum server runs as a spawned task that needs
    // the runtime alive for the entire application lifetime.
    let rt: &'static tokio::runtime::Runtime = Box::leak(Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_stack_size(8 * 1024 * 1024)
            .build()
            .expect("Failed to build tokio runtime"),
    ));

    let _guard = rt.enter();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                // Forward --experimental flag from second instance
                if args.iter().any(|a| a == "--experimental") {
                    let _ = window.eval("localStorage.setItem('konto_experimental','true');location.reload()");
                }
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![get_server_port, get_app_version, reset_database])
        .setup(move |app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Spawn the embedded Axum server
            let data_dir = app_data_dir.clone();
            let port = rt.block_on(server::start_embedded_server(data_dir));
            set_server_port(port);

            tracing::info!("Maravilla Konto desktop app started, server on port {port}");

            // Forward --experimental CLI flag to the frontend via localStorage
            let experimental = std::env::args().any(|a| a == "--experimental");

            if let Some(window) = app.get_webview_window("main") {
                if experimental {
                    let _ = window.eval("localStorage.setItem('konto_experimental','true')");
                }

                // On first launch, size window relative to monitor.
                // On subsequent launches, tauri_plugin_window_state restores saved size.
                let marker = app_data_dir.join(".window-initialized");
                if !marker.exists() {
                    resize_to_monitor(&window);
                    std::fs::write(&marker, "").ok();
                }

                apply_vibrancy(&window);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Maravilla Konto");
}

/// Resize the window to ~80% of the current monitor, clamped to sensible bounds.
fn resize_to_monitor(window: &tauri::WebviewWindow) {
    let monitor = match window.current_monitor() {
        Ok(Some(m)) => m,
        _ => return,
    };

    let PhysicalSize { width, height } = *monitor.size();
    let scale = monitor.scale_factor();

    // Convert to logical pixels
    let logical_w = width as f64 / scale;
    let logical_h = height as f64 / scale;

    // 80% of screen, clamped between min (960x600) and max (1600x1000)
    let w = (logical_w * 0.80).clamp(960.0, 1600.0);
    let h = (logical_h * 0.85).clamp(600.0, 1000.0);

    let _ = window.set_size(tauri::LogicalSize::new(w, h));
    let _ = window.center();
}

#[allow(unused_variables)]
fn apply_vibrancy(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        // Use native corner radius so the actual window surface is rounded,
        // not just the HTML content.
        let _ = apply_vibrancy(window, NSVisualEffectMaterial::Sidebar, None, Some(14.0));
    }

    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_mica;
        let _ = apply_mica(window, None);
    }
}
