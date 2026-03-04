//! Database schema and migrations
//! 
//! Defines the SQLite database structure for Media Organizer

use rusqlite::{Connection, Result};
use log::info;

/// Database schema version
const SCHEMA_VERSION: i32 = 1;

/// Initialize database with all tables
pub fn init_db(conn: &Connection) -> Result<()> {
    info!("Initializing database (version {})", SCHEMA_VERSION);
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // Create tables
    create_media_files_table(conn)?;
    create_tags_table(conn)?;
    create_media_tags_table(conn)?;
    create_albums_table(conn)?;
    create_album_media_table(conn)?;
    create_indexes(conn)?;
    
    // Store schema version
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        [],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
        [SCHEMA_VERSION],
    )?;
    
    info!("Database initialized successfully");
    Ok(())
}

/// Create media_files table
fn create_media_files_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path TEXT UNIQUE NOT NULL,
            file_type TEXT NOT NULL CHECK(file_type IN ('image', 'video')),
            file_size INTEGER NOT NULL,
            width INTEGER,
            height INTEGER,
            duration INTEGER,
            created_at INTEGER NOT NULL,
            modified_at INTEGER NOT NULL,
            taken_at INTEGER,
            device TEXT,
            gps_lat REAL,
            gps_lon REAL,
            checksum TEXT,
            thumbnail_path TEXT,
            is_deleted INTEGER DEFAULT 0
        )",
        [],
    )?;
    info!("Created table: media_files");
    Ok(())
}

/// Create tags table
fn create_tags_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            parent_id INTEGER REFERENCES tags(id),
            color TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;
    info!("Created table: tags");
    Ok(())
}

/// Create media_tags junction table
fn create_media_tags_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_tags (
            media_id INTEGER NOT NULL REFERENCES media_files(id) ON DELETE CASCADE,
            tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            PRIMARY KEY (media_id, tag_id)
        )",
        [],
    )?;
    info!("Created table: media_tags");
    Ok(())
}

/// Create albums table
fn create_albums_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            cover_media_id INTEGER REFERENCES media_files(id),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;
    info!("Created table: albums");
    Ok(())
}

/// Create album_media junction table
fn create_album_media_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS album_media (
            album_id INTEGER NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
            media_id INTEGER NOT NULL REFERENCES media_files(id) ON DELETE CASCADE,
            position INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (album_id, media_id)
        )",
        [],
    )?;
    info!("Created table: album_media");
    Ok(())
}

/// Create indexes for performance
fn create_indexes(conn: &Connection) -> Result<()> {
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_media_type ON media_files(file_type)",
        "CREATE INDEX IF NOT EXISTS idx_media_taken_at ON media_files(taken_at)",
        "CREATE INDEX IF NOT EXISTS idx_media_checksum ON media_files(checksum)",
        "CREATE INDEX IF NOT EXISTS idx_media_created_at ON media_files(created_at)",
        "CREATE INDEX IF NOT EXISTS idx_media_is_deleted ON media_files(is_deleted)",
        "CREATE INDEX IF NOT EXISTS idx_tags_parent ON tags(parent_id)",
        "CREATE INDEX IF NOT EXISTS idx_media_tags_media ON media_tags(media_id)",
        "CREATE INDEX IF NOT EXISTS idx_media_tags_tag ON media_tags(tag_id)",
        "CREATE INDEX IF NOT EXISTS idx_album_media_album ON album_media(album_id)",
        "CREATE INDEX IF NOT EXISTS idx_album_media_media ON album_media(media_id)",
    ];
    
    for sql in indexes {
        conn.execute(sql, [])?;
    }
    info!("Created indexes");
    Ok(())
}

/// Get current schema version
pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    conn.query_row(
        "SELECT version FROM schema_version LIMIT 1",
        [],
        |row| row.get(0),
    )
    .or(Ok(0))
}

/// Check if database needs migration
pub fn needs_migration(conn: &Connection) -> Result<bool> {
    let version = get_schema_version(conn)?;
    Ok(version < SCHEMA_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    #[test]
    fn test_init_db() {
        let conn = Connection::open_in_memory().unwrap();
        let result = init_db(&conn);
        assert!(result.is_ok());
        
        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }
}
