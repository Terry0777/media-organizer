//! Database connection and management

use log::{error, info};
use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;

use crate::db::schema;

/// Database wrapper with connection pooling
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Open or create database at given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Opening database at: {:?}", path.as_ref());

        let conn = Connection::open(path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode = WAL;")?;

        // Set busy timeout (5 seconds)
        conn.execute_batch("PRAGMA busy_timeout = 5000;")?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Open in-memory database (for testing)
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Initialize database schema
    pub fn init(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| {
            error!("Failed to lock database connection: {}", e);
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
                Some("Lock poisoned".to_string()),
            )
        })?;

        schema::init_db(&conn)
    }

    /// Get database connection (blocking)
    pub fn get_conn(&self) -> Result<std::sync::MutexGuard<Connection>> {
        self.conn.lock().map_err(|e| {
            error!("Failed to lock database connection: {}", e);
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
                Some("Lock poisoned".to_string()),
            )
        })
    }

    /// Check if database needs migration
    pub fn needs_migration(&self) -> Result<bool> {
        let conn = self.get_conn()?;
        schema::needs_migration(&conn)
    }

    /// Get database file path (for backup purposes)
    pub fn path(&self) -> Option<String> {
        let conn = self.get_conn().ok()?;
        conn.path().map(|p| p.to_string())
    }

    /// Run VACUUM to optimize database
    pub fn vacuum(&self) -> Result<()> {
        info!("Running VACUUM on database");
        let conn = self.get_conn()?;
        conn.execute("VACUUM", [])?;
        Ok(())
    }

    /// Get database statistics
    pub fn stats(&self) -> Result<crate::models::DatabaseStats> {
        use crate::models::DatabaseStats;

        let conn = self.get_conn()?;

        let mut stats = DatabaseStats::default();

        // Total files
        conn.query_row(
            "SELECT COUNT(*) FROM media_files WHERE is_deleted = 0",
            [],
            |row| {
                stats.total_files = row.get(0)?;
                Ok(())
            },
        )?;

        // Total images
        conn.query_row(
            "SELECT COUNT(*) FROM media_files WHERE file_type = 'image' AND is_deleted = 0",
            [],
            |row| {
                stats.total_images = row.get(0)?;
                Ok(())
            },
        )?;

        // Total videos
        conn.query_row(
            "SELECT COUNT(*) FROM media_files WHERE file_type = 'video' AND is_deleted = 0",
            [],
            |row| {
                stats.total_videos = row.get(0)?;
                Ok(())
            },
        )?;

        // Total size
        conn.query_row(
            "SELECT COALESCE(SUM(file_size), 0) FROM media_files WHERE is_deleted = 0",
            [],
            |row| {
                stats.total_size = row.get(0)?;
                Ok(())
            },
        )?;

        // Total tags
        conn.query_row("SELECT COUNT(*) FROM tags", [], |row| {
            stats.total_tags = row.get(0)?;
            Ok(())
        })?;

        // Total albums
        conn.query_row("SELECT COUNT(*) FROM albums", [], |row| {
            stats.total_albums = row.get(0)?;
            Ok(())
        })?;

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory().unwrap();
        let result = db.init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats() {
        let db = Database::open_in_memory().unwrap();
        db.init().unwrap();

        let stats = db.stats().unwrap();
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_tags, 0);
        assert_eq!(stats.total_albums, 0);
    }
}
