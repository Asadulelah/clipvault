use rusqlite::Connection;

pub fn run(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS clipboard_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content_type TEXT NOT NULL,
            content_text TEXT,
            content_blob BLOB,
            thumbnail_blob BLOB,
            preview_text TEXT,
            source_app TEXT,
            is_pinned INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            hash TEXT NOT NULL UNIQUE
        );

        CREATE INDEX IF NOT EXISTS idx_clipboard_items_hash ON clipboard_items(hash);
        CREATE INDEX IF NOT EXISTS idx_clipboard_items_created_at ON clipboard_items(created_at);
        CREATE INDEX IF NOT EXISTS idx_clipboard_items_pinned ON clipboard_items(is_pinned);

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );

        INSERT OR IGNORE INTO settings (key, value) VALUES ('theme', 'system');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('history_limit', '0');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('autostart', 'true');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('hotkey', 'Super+V');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('show_source_app', 'true');
        ",
    )
    .expect("Failed to run migrations");
}
