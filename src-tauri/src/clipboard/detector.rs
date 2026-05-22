use image::ImageReader;
use std::io::Cursor;

pub enum ClipboardContent {
    Text(String),
    Image {
        raw_png: Vec<u8>,
        width: u32,
        height: u32,
    },
    Url(String),
}

pub fn classify_text(text: &str) -> ClipboardContent {
    let trimmed = text.trim();
    if is_url(trimmed) {
        ClipboardContent::Url(trimmed.to_string())
    } else {
        ClipboardContent::Text(text.to_string())
    }
}

fn is_url(text: &str) -> bool {
    (text.starts_with("http://") || text.starts_with("https://"))
        && !text.contains('\n')
        && text.len() < 2048
}

pub fn generate_thumbnail(png_data: &[u8], max_size: u32) -> Option<Vec<u8>> {
    let reader = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()
        .ok()?;
    let img = reader.decode().ok()?;
    let thumbnail = img.thumbnail(max_size, max_size);
    let mut buf = Vec::new();
    thumbnail
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .ok()?;
    Some(buf)
}

pub fn make_preview(text: &str, max_len: usize) -> String {
    let preview: String = text
        .chars()
        .take(max_len)
        .map(|c| if c.is_control() && c != ' ' { ' ' } else { c })
        .collect();
    if text.len() > max_len {
        format!("{}...", preview.trim_end())
    } else {
        preview.trim_end().to_string()
    }
}

#[cfg(windows)]
pub fn get_foreground_app() -> Option<String> {
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }

        // Try to get the window title as a simpler fallback
        use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;
        let mut title = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut title);
        if len > 0 {
            let title_str = String::from_utf16_lossy(&title[..len as usize]);
            // Try to extract app name from window title
            // Common patterns: "Document - AppName", "AppName"
            if let Some(app) = title_str.rsplit(" - ").next() {
                if !app.is_empty() {
                    return Some(app.to_string());
                }
            }
            return Some(title_str);
        }

        None
    }
}

#[cfg(not(windows))]
pub fn get_foreground_app() -> Option<String> {
    None
}
