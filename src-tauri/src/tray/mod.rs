use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

pub fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open ClipVault", true, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let clear = MenuItem::with_id(app, "clear", "Clear History", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open, &sep1, &clear, &sep2, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("ClipVault")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                if let Some(win) = app.get_webview_window("main") {
                    crate::window::show_panel(&win);
                }
            }
            "clear" => {
                let db = app.state::<crate::DbState>().inner().clone();
                if let Ok(conn) = db.lock() {
                    let _ = crate::db::queries::clear_history(&conn);
                };
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(win) = tray.app_handle().get_webview_window("main") {
                    crate::window::toggle_panel(&win);
                }
            }
        })
        .build(app)?;

    Ok(())
}
