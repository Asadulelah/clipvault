use tauri::{PhysicalPosition, WebviewWindow};

pub fn toggle_panel(window: &WebviewWindow) {
    if window.is_visible().unwrap_or(false) {
        hide_panel(window);
    } else {
        show_panel(window);
    }
}

pub fn show_panel(window: &WebviewWindow) {
    position_panel(window);
    let _ = window.show();
    let _ = window.set_focus();
}

pub fn hide_panel(window: &WebviewWindow) {
    let _ = window.hide();
}

fn position_panel(window: &WebviewWindow) {
    if let Some(monitor) = window.current_monitor().ok().flatten() {
        let screen = monitor.size();
        let scale = monitor.scale_factor();
        let position = monitor.position();

        let panel_width = 360.0;
        let panel_height = 460.0;
        let taskbar_height = 48.0;
        let margin = 16.0;

        let x = (screen.width as f64 / scale) - panel_width - margin + (position.x as f64 / scale);
        let y = (screen.height as f64 / scale) - panel_height - taskbar_height - margin
            + (position.y as f64 / scale);

        let _ = window.set_position(PhysicalPosition::new(
            (x * scale) as i32,
            (y * scale) as i32,
        ));
    }
}
