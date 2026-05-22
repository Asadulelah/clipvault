use crate::db::queries;
use crate::DbState;
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_plugin_autostart::ManagerExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPayload {
    pub theme: String,
    pub history_limit: i64,
    pub autostart: bool,
    pub hotkey: String,
    pub show_source_app: bool,
}

#[tauri::command]
pub fn get_settings(db: State<'_, DbState>) -> Result<SettingsPayload, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let map = queries::get_settings(&conn).map_err(|e| e.to_string())?;

    Ok(SettingsPayload {
        theme: map.get("theme").cloned().unwrap_or("system".into()),
        history_limit: map
            .get("history_limit")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        autostart: map
            .get("autostart")
            .map(|v| v == "true")
            .unwrap_or(true),
        hotkey: map.get("hotkey").cloned().unwrap_or("Super+V".into()),
        show_source_app: map
            .get("show_source_app")
            .map(|v| v == "true")
            .unwrap_or(true),
    })
}

#[tauri::command]
pub fn save_settings(
    app: tauri::AppHandle,
    db: State<'_, DbState>,
    settings: SettingsPayload,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    queries::save_setting(&conn, "theme", &settings.theme).map_err(|e| e.to_string())?;
    queries::save_setting(&conn, "history_limit", &settings.history_limit.to_string())
        .map_err(|e| e.to_string())?;
    queries::save_setting(&conn, "autostart", &settings.autostart.to_string())
        .map_err(|e| e.to_string())?;
    queries::save_setting(&conn, "hotkey", &settings.hotkey).map_err(|e| e.to_string())?;
    queries::save_setting(&conn, "show_source_app", &settings.show_source_app.to_string())
        .map_err(|e| e.to_string())?;

    // Toggle Windows auto-start
    let autolaunch = app.autolaunch();
    if settings.autostart {
        let _ = autolaunch.enable();
    } else {
        let _ = autolaunch.disable();
    }

    Ok(())
}
