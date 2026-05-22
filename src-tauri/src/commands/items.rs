use crate::clipboard::watcher::SKIP_NEXT;
use crate::db::queries;
use crate::DbState;
use arboard::Clipboard;
use std::sync::atomic::Ordering;
use tauri::State;

#[tauri::command]
pub fn pin_item(db: State<'_, DbState>, id: i64) -> Result<bool, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    queries::pin_item(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_item(db: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    queries::delete_item(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_history(db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    queries::clear_history(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn copy_item(db: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let (content_type, content_text, content_blob) =
        queries::get_item_raw_blob(&conn, id).map_err(|e| e.to_string())?;
    drop(conn); // Release lock before clipboard write

    // Set skip flag so watcher doesn't recapture this
    SKIP_NEXT.store(true, Ordering::SeqCst);

    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;

    match content_type.as_str() {
        "image" => {
            if let Some(blob) = content_blob {
                // Decode PNG back to RGBA for arboard
                let img = image::load_from_memory(&blob).map_err(|e| e.to_string())?;
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let img_data = arboard::ImageData {
                    width: w as usize,
                    height: h as usize,
                    bytes: std::borrow::Cow::Owned(rgba.into_raw()),
                };
                clipboard
                    .set_image(img_data)
                    .map_err(|e| e.to_string())?;
            }
        }
        _ => {
            let text = content_text.unwrap_or_default();
            clipboard.set_text(text).map_err(|e| e.to_string())?;
        }
    }

    // Small delay then simulate Ctrl+V to paste into the previously focused app
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(100));
        simulate_paste();
    });

    Ok(())
}

/// Simulate Ctrl+V keypress using Windows SendInput API
fn simulate_paste() {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
        VIRTUAL_KEY,
    };

    const VK_CONTROL: VIRTUAL_KEY = VIRTUAL_KEY(0x11);
    const VK_V: VIRTUAL_KEY = VIRTUAL_KEY(0x56);

    let mut inputs: [INPUT; 4] = [
        // Ctrl down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // V down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // V up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Ctrl up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
    }
}
