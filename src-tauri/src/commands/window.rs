use tauri::WebviewWindow;

#[tauri::command]
pub fn hide_window(window: WebviewWindow) {
    let _ = window.hide();
}
