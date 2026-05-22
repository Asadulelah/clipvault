#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod commands;
mod db;
mod tray;
mod window;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub type DbState = Arc<Mutex<rusqlite::Connection>>;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            // Initialize database
            let conn = db::connection::init_db();
            db::migrations::run(&conn);
            let db_state: DbState = Arc::new(Mutex::new(conn));
            app.manage(db_state.clone());

            // Create system tray
            tray::create_tray(app)?;

            // Register global shortcut (Win+V)
            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut("super+v", move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if let Some(win) = app_handle.get_webview_window("main") {
                        window::toggle_panel(&win);
                    }
                }
            })?;

            // Enable autostart on first run
            let autolaunch = app.autolaunch();
            if !autolaunch.is_enabled().unwrap_or(false) {
                let _ = autolaunch.enable();
            }

            // Start clipboard watcher
            let watcher_handle = app.handle().clone();
            let watcher_db = db_state.clone();
            clipboard::watcher::start_watcher(watcher_handle, watcher_db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::history::get_history,
            commands::history::search_history,
            commands::items::pin_item,
            commands::items::delete_item,
            commands::items::clear_history,
            commands::items::copy_item,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::window::hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ClipVault");
}
