use rusqlite::Connection;
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").expect("APPDATA not set");
    let dir = PathBuf::from(appdata).join("ClipVault");
    std::fs::create_dir_all(&dir).expect("Failed to create ClipVault data directory");
    dir.join("clipvault.db")
}

pub fn init_db() -> Connection {
    let path = get_db_path();
    let conn = Connection::open(&path).expect("Failed to open database");

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("Failed to set pragmas");

    conn
}
