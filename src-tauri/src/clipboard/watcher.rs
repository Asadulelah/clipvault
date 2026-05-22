use crate::clipboard::{detector, hasher};
use crate::db::queries;
use crate::DbState;
use arboard::Clipboard;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::Emitter;

/// Global flag to skip self-writes (when copy_item writes to clipboard)
pub static SKIP_NEXT: AtomicBool = AtomicBool::new(false);

pub fn start_watcher(app_handle: tauri::AppHandle, db: DbState) {
    std::thread::spawn(move || {
        let mut clipboard = match Clipboard::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to initialize clipboard: {}", e);
                return;
            }
        };

        let mut last_hash = String::new();

        loop {
            std::thread::sleep(Duration::from_millis(500));

            // Check if we should skip (self-write from copy_item)
            if SKIP_NEXT.load(Ordering::SeqCst) {
                SKIP_NEXT.store(false, Ordering::SeqCst);
                // Read current clipboard to update last_hash so we don't re-capture
                if let Ok(text) = clipboard.get_text() {
                    last_hash = hasher::hash_text(&text);
                }
                continue;
            }

            // Try to read image first, then text
            let result = try_read_clipboard(&mut clipboard, &last_hash);

            let capture = match result {
                Some(c) => c,
                None => continue,
            };

            last_hash = capture.hash.clone();

            // Get source app
            let source_app = detector::get_foreground_app();

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;

            // Check DB for existing hash
            let conn = match db.lock() {
                Ok(c) => c,
                Err(_) => continue,
            };

            if let Ok(exists) = queries::hash_exists(&conn, &capture.hash) {
                if exists {
                    // Update timestamp and move to top
                    if let Ok(id) = queries::update_timestamp(&conn, &capture.hash, now) {
                        if let Ok(item) = queries::get_item_by_id(&conn, id) {
                            let _ = app_handle.emit("clipboard-updated", &item);
                        }
                    }
                    continue;
                }
            }

            let insert = queries::InsertItem {
                content_type: capture.content_type,
                content_text: capture.content_text,
                content_blob: capture.content_blob,
                thumbnail_blob: capture.thumbnail_blob,
                preview_text: capture.preview_text,
                source_app,
                created_at: now,
                hash: capture.hash,
            };

            if let Ok(id) = queries::insert_item(&conn, &insert) {
                // Enforce history limit
                if let Ok(settings) = queries::get_settings(&conn) {
                    if let Some(limit_str) = settings.get("history_limit") {
                        if let Ok(limit) = limit_str.parse::<i64>() {
                            let _ = queries::enforce_history_limit(&conn, limit);
                        }
                    }
                }

                if let Ok(item) = queries::get_item_by_id(&conn, id) {
                    let _ = app_handle.emit("clipboard-updated", &item);
                }
            }
        }
    });
}

struct CapturedClipboard {
    content_type: String,
    content_text: Option<String>,
    content_blob: Option<Vec<u8>>,
    thumbnail_blob: Option<Vec<u8>>,
    preview_text: Option<String>,
    hash: String,
}

fn try_read_clipboard(clipboard: &mut Clipboard, last_hash: &str) -> Option<CapturedClipboard> {
    // Try image first
    if let Ok(img) = clipboard.get_image() {
        let raw_rgba = img.bytes.to_vec();
        let hash = hasher::hash_bytes(&raw_rgba);
        if hash == last_hash {
            return None;
        }

        // Convert RGBA to PNG
        let png_data = rgba_to_png(&raw_rgba, img.width, img.height)?;
        let thumbnail = detector::generate_thumbnail(&png_data, 200);

        return Some(CapturedClipboard {
            content_type: "image".to_string(),
            content_text: None,
            content_blob: Some(png_data),
            thumbnail_blob: thumbnail,
            preview_text: Some(format!("Image {}x{}", img.width, img.height)),
            hash,
        });
    }

    // Try text
    if let Ok(text) = clipboard.get_text() {
        if text.trim().is_empty() {
            return None;
        }

        let hash = hasher::hash_text(&text);
        if hash == last_hash {
            return None;
        }

        let content = detector::classify_text(&text);
        let preview = detector::make_preview(&text, 200);

        match content {
            detector::ClipboardContent::Url(url) => Some(CapturedClipboard {
                content_type: "url".to_string(),
                content_text: Some(url),
                content_blob: None,
                thumbnail_blob: None,
                preview_text: Some(preview),
                hash,
            }),
            detector::ClipboardContent::Text(t) => Some(CapturedClipboard {
                content_type: "text".to_string(),
                content_text: Some(t),
                content_blob: None,
                thumbnail_blob: None,
                preview_text: Some(preview),
                hash,
            }),
            detector::ClipboardContent::Image { .. } => None, // handled above
        }
    } else {
        None
    }
}

fn rgba_to_png(rgba: &[u8], width: usize, height: usize) -> Option<Vec<u8>> {
    let img = image::RgbaImage::from_raw(width as u32, height as u32, rgba.to_vec())?;
    let mut buf = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::Png,
    )
    .ok()?;
    Some(buf)
}
