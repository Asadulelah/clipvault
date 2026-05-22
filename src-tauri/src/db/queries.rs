use base64::Engine;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: i64,
    pub content_type: String,
    pub content_text: Option<String>,
    pub content_blob: Option<String>,     // base64 encoded
    pub thumbnail_blob: Option<String>,   // base64 encoded
    pub preview_text: Option<String>,
    pub source_app: Option<String>,
    pub is_pinned: bool,
    pub created_at: i64,
    pub hash: String,
}

pub struct InsertItem {
    pub content_type: String,
    pub content_text: Option<String>,
    pub content_blob: Option<Vec<u8>>,
    pub thumbnail_blob: Option<Vec<u8>>,
    pub preview_text: Option<String>,
    pub source_app: Option<String>,
    pub created_at: i64,
    pub hash: String,
}

fn row_to_item(row: &rusqlite::Row) -> rusqlite::Result<ClipboardItem> {
    let content_blob: Option<Vec<u8>> = row.get(3)?;
    let thumbnail_blob: Option<Vec<u8>> = row.get(4)?;
    let engine = base64::engine::general_purpose::STANDARD;

    Ok(ClipboardItem {
        id: row.get(0)?,
        content_type: row.get(1)?,
        content_text: row.get(2)?,
        content_blob: content_blob.map(|b| engine.encode(&b)),
        thumbnail_blob: thumbnail_blob.map(|b| engine.encode(&b)),
        preview_text: row.get(5)?,
        source_app: row.get(6)?,
        is_pinned: row.get::<_, i32>(7)? != 0,
        created_at: row.get(8)?,
        hash: row.get(9)?,
    })
}

pub fn insert_item(conn: &Connection, item: &InsertItem) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO clipboard_items (content_type, content_text, content_blob, thumbnail_blob, preview_text, source_app, created_at, hash)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            item.content_type,
            item.content_text,
            item.content_blob,
            item.thumbnail_blob,
            item.preview_text,
            item.source_app,
            item.created_at,
            item.hash,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_timestamp(conn: &Connection, hash: &str, timestamp: i64) -> rusqlite::Result<i64> {
    conn.execute(
        "UPDATE clipboard_items SET created_at = ?1 WHERE hash = ?2",
        params![timestamp, hash],
    )?;
    let id: i64 = conn.query_row(
        "SELECT id FROM clipboard_items WHERE hash = ?1",
        params![hash],
        |row| row.get(0),
    )?;
    Ok(id)
}

pub fn hash_exists(conn: &Connection, hash: &str) -> rusqlite::Result<bool> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM clipboard_items WHERE hash = ?1",
        params![hash],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn get_history(conn: &Connection, limit: i64, offset: i64) -> rusqlite::Result<Vec<ClipboardItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, content_type, content_text, content_blob, thumbnail_blob, preview_text, source_app, is_pinned, created_at, hash
         FROM clipboard_items
         ORDER BY is_pinned DESC, created_at DESC
         LIMIT ?1 OFFSET ?2",
    )?;
    let items = stmt
        .query_map(params![limit, offset], |row| row_to_item(row))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(items)
}

pub fn search_history(conn: &Connection, query: &str) -> rusqlite::Result<Vec<ClipboardItem>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, content_type, content_text, content_blob, thumbnail_blob, preview_text, source_app, is_pinned, created_at, hash
         FROM clipboard_items
         WHERE preview_text LIKE ?1 OR content_text LIKE ?1
         ORDER BY is_pinned DESC, created_at DESC
         LIMIT 100",
    )?;
    let items = stmt
        .query_map(params![pattern], |row| row_to_item(row))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(items)
}

pub fn pin_item(conn: &Connection, id: i64) -> rusqlite::Result<bool> {
    conn.execute(
        "UPDATE clipboard_items SET is_pinned = CASE WHEN is_pinned = 0 THEN 1 ELSE 0 END WHERE id = ?1",
        params![id],
    )?;
    let pinned: i32 = conn.query_row(
        "SELECT is_pinned FROM clipboard_items WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    Ok(pinned != 0)
}

pub fn delete_item(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM clipboard_items WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn clear_history(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM clipboard_items WHERE is_pinned = 0", [])?;
    Ok(())
}

pub fn get_item_by_id(conn: &Connection, id: i64) -> rusqlite::Result<ClipboardItem> {
    conn.query_row(
        "SELECT id, content_type, content_text, content_blob, thumbnail_blob, preview_text, source_app, is_pinned, created_at, hash
         FROM clipboard_items WHERE id = ?1",
        params![id],
        |row| row_to_item(row),
    )
}

pub fn get_item_raw_blob(conn: &Connection, id: i64) -> rusqlite::Result<(String, Option<String>, Option<Vec<u8>>)> {
    conn.query_row(
        "SELECT content_type, content_text, content_blob FROM clipboard_items WHERE id = ?1",
        params![id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
}

pub fn get_settings(conn: &Connection) -> rusqlite::Result<HashMap<String, String>> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let settings = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<HashMap<String, String>>>()?;
    Ok(settings)
}

pub fn save_setting(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )?;
    Ok(())
}

pub fn enforce_history_limit(conn: &Connection, limit: i64) -> rusqlite::Result<()> {
    if limit <= 0 {
        return Ok(());
    }
    conn.execute(
        "DELETE FROM clipboard_items WHERE is_pinned = 0 AND id NOT IN (
            SELECT id FROM clipboard_items ORDER BY is_pinned DESC, created_at DESC LIMIT ?1
        )",
        params![limit],
    )?;
    Ok(())
}
