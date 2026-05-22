use crate::db::queries;
use crate::DbState;
use tauri::State;

#[tauri::command]
pub fn get_history(
    db: State<'_, DbState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<queries::ClipboardItem>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    queries::get_history(&conn, limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_history(
    db: State<'_, DbState>,
    query: String,
) -> Result<Vec<queries::ClipboardItem>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    queries::search_history(&conn, &query).map_err(|e| e.to_string())
}
