//! Repository pattern for database operations
//! 
//! Provides CRUD operations for media files, tags, and albums

use rusqlite::{Connection, Result, params, OptionalExtension};
use log::{info, warn};
use chrono::Utc;

use crate::models::*;

/// Media file repository
pub struct MediaRepository;

impl MediaRepository {
    /// Insert a new media file
    pub fn insert(conn: &Connection, media: &MediaFile) -> Result<i64> {
        conn.execute(
            "INSERT INTO media_files (
                file_path, file_type, file_size, width, height, duration,
                created_at, modified_at, taken_at, device, gps_lat, gps_lon,
                checksum, thumbnail_path, is_deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                media.file_path,
                media.file_type.as_str(),
                media.file_size,
                media.width,
                media.height,
                media.duration,
                media.created_at,
                media.modified_at,
                media.taken_at,
                media.device,
                media.gps_lat,
                media.gps_lon,
                media.checksum,
                media.thumbnail_path,
                media.is_deleted as i32
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }
    
    /// Get media file by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<MediaFile>> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, file_type, file_size, width, height, duration,
                    created_at, modified_at, taken_at, device, gps_lat, gps_lon,
                    checksum, thumbnail_path, is_deleted
             FROM media_files WHERE id = ?1"
        )?;
        
        let media = stmt.query_row(params![id], |row| {
            Ok(MediaFile {
                id: Some(row.get(0)?),
                file_path: row.get(1)?,
                file_type: FileType::from_str(&row.get::<_, String>(2)?).unwrap_or(FileType::Image),
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                duration: row.get(6)?,
                created_at: row.get(7)?,
                modified_at: row.get(8)?,
                taken_at: row.get(9)?,
                device: row.get(10)?,
                gps_lat: row.get(11)?,
                gps_lon: row.get(12)?,
                checksum: row.get(13)?,
                thumbnail_path: row.get(14)?,
                is_deleted: row.get::<_, i32>(15)? != 0,
                tags: None,
            })
        }).optional()?;
        
        Ok(media)
    }
    
    /// Get media files with filters and pagination
    pub fn list(
        conn: &Connection,
        filters: &SearchFilters,
        pagination: &Pagination,
    ) -> Result<SearchResult<MediaFile>> {
        let mut sql = String::from(
            "SELECT id, file_path, file_type, file_size, width, height, duration,
                    created_at, modified_at, taken_at, device, gps_lat, gps_lon,
                    checksum, thumbnail_path, is_deleted
             FROM media_files WHERE 1=1"
        );
        
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        // Apply filters
        if let Some(ref file_type) = filters.file_type {
            sql.push_str(" AND file_type = ?");
            params.push(Box::new(file_type.as_str()));
        }
        
        if let Some(is_deleted) = filters.is_deleted {
            sql.push_str(" AND is_deleted = ?");
            params.push(Box::new(is_deleted as i32));
        } else {
            sql.push_str(" AND is_deleted = 0");
        }
        
        if let Some(date_from) = filters.date_from {
            sql.push_str(" AND taken_at >= ?");
            params.push(Box::new(date_from));
        }
        
        if let Some(date_to) = filters.date_to {
            sql.push_str(" AND taken_at <= ?");
            params.push(Box::new(date_to));
        }
        
        if let Some(size_min) = filters.size_min {
            sql.push_str(" AND file_size >= ?");
            params.push(Box::new(size_min));
        }
        
        if let Some(size_max) = filters.size_max {
            sql.push_str(" AND file_size <= ?");
            params.push(Box::new(size_max));
        }
        
        // Get total count
        let count_sql = format!("SELECT COUNT(*) FROM ({})", sql);
        let total: i64 = conn.query_row(&count_sql, params.as_slice(), |row| row.get(0))?;
        
        // Add ordering and pagination
        sql.push_str(" ORDER BY taken_at DESC, created_at DESC LIMIT ? OFFSET ?");
        let offset = (pagination.page - 1) * pagination.page_size;
        params.push(Box::new(pagination.page_size as i64));
        params.push(Box::new(offset as i64));
        
        // Execute query
        let mut stmt = conn.prepare(&sql)?;
        let media_iter = stmt.query_map(params.as_slice(), |row| {
            Ok(MediaFile {
                id: Some(row.get(0)?),
                file_path: row.get(1)?,
                file_type: FileType::from_str(&row.get::<_, String>(2)?).unwrap_or(FileType::Image),
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                duration: row.get(6)?,
                created_at: row.get(7)?,
                modified_at: row.get(8)?,
                taken_at: row.get(9)?,
                device: row.get(10)?,
                gps_lat: row.get(11)?,
                gps_lon: row.get(12)?,
                checksum: row.get(13)?,
                thumbnail_path: row.get(14)?,
                is_deleted: row.get::<_, i32>(15)? != 0,
                tags: None,
            })
        })?;
        
        let mut items = Vec::new();
        for media in media_iter {
            items.push(media?);
        }
        
        Ok(SearchResult::new(
            items,
            total,
            pagination.page,
            pagination.page_size,
        ))
    }
    
    /// Update media file
    pub fn update(conn: &Connection, media: &MediaFile) -> Result<usize> {
        let rows = conn.execute(
            "UPDATE media_files SET
                file_size = ?1, width = ?2, height = ?3, duration = ?4,
                modified_at = ?5, taken_at = ?6, device = ?7,
                gps_lat = ?8, gps_lon = ?9, checksum = ?10,
                thumbnail_path = ?11
             WHERE id = ?12",
            params![
                media.file_size,
                media.width,
                media.height,
                media.duration,
                media.modified_at,
                media.taken_at,
                media.device,
                media.gps_lat,
                media.gps_lon,
                media.checksum,
                media.thumbnail_path,
                media.id
            ],
        )?;
        
        Ok(rows)
    }
    
    /// Delete media file (soft delete)
    pub fn delete(conn: &Connection, id: i64) -> Result<usize> {
        let rows = conn.execute(
            "UPDATE media_files SET is_deleted = 1, modified_at = ?1 WHERE id = ?2",
            params![Utc::now().timestamp(), id],
        )?;
        
        Ok(rows)
    }
    
    /// Permanently delete media file
    pub fn permanent_delete(conn: &Connection, id: i64) -> Result<usize> {
        let rows = conn.execute("DELETE FROM media_files WHERE id = ?1", params![id])?;
        Ok(rows)
    }
    
    /// Check if file exists by path
    pub fn exists_by_path(conn: &Connection, path: &str) -> Result<bool> {
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM media_files WHERE file_path = ?1)",
            params![path],
            |row| row.get(0),
        )?;
        
        Ok(exists)
    }
    
    /// Get file by checksum
    pub fn get_by_checksum(conn: &Connection, checksum: &str) -> Result<Option<MediaFile>> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, file_type, file_size, width, height, duration,
                    created_at, modified_at, taken_at, device, gps_lat, gps_lon,
                    checksum, thumbnail_path, is_deleted
             FROM media_files WHERE checksum = ?1"
        )?;
        
        let media = stmt.query_row(params![checksum], |row| {
            Ok(MediaFile {
                id: Some(row.get(0)?),
                file_path: row.get(1)?,
                file_type: FileType::from_str(&row.get::<_, String>(2)?).unwrap_or(FileType::Image),
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                duration: row.get(6)?,
                created_at: row.get(7)?,
                modified_at: row.get(8)?,
                taken_at: row.get(9)?,
                device: row.get(10)?,
                gps_lat: row.get(11)?,
                gps_lon: row.get(12)?,
                checksum: row.get(13)?,
                thumbnail_path: row.get(14)?,
                is_deleted: row.get::<_, i32>(15)? != 0,
                tags: None,
            })
        }).optional()?;
        
        Ok(media)
    }
}

/// Tag repository
pub struct TagRepository;

impl TagRepository {
    /// Insert a new tag
    pub fn insert(conn: &Connection, tag: &Tag) -> Result<i64> {
        let now = Utc::now().timestamp();
        conn.execute(
            "INSERT INTO tags (name, parent_id, color, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![tag.name, tag.parent_id, tag.color, now],
        )?;
        
        Ok(conn.last_insert_rowid())
    }
    
    /// Get tag by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Tag>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, color, created_at FROM tags WHERE id = ?1"
        )?;
        
        let tag = stmt.query_row(params![id], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                media_count: None,
            })
        }).optional()?;
        
        Ok(tag)
    }
    
    /// Get all tags
    pub fn list(conn: &Connection) -> Result<Vec<Tag>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, color, created_at FROM tags ORDER BY name"
        )?;
        
        let tags = stmt.query_map([], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                media_count: None,
            })
        })?;
        
        let mut result = Vec::new();
        for tag in tags {
            result.push(tag?);
        }
        
        Ok(result)
    }
    
    /// Get tag usage statistics
    pub fn get_usage(conn: &Connection) -> Result<Vec<TagUsage>> {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.parent_id, t.color, COUNT(mt.media_id) as usage_count
             FROM tags t
             LEFT JOIN media_tags mt ON t.id = mt.tag_id
             GROUP BY t.id
             ORDER BY usage_count DESC, t.name"
        )?;
        
        let tags = stmt.query_map([], |row| {
            Ok(TagUsage {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                color: row.get(3)?,
                usage_count: row.get(4)?,
            })
        })?;
        
        let mut result = Vec::new();
        for tag in tags {
            result.push(tag?);
        }
        
        Ok(result)
    }
    
    /// Update tag
    pub fn update(conn: &Connection, tag: &Tag) -> Result<usize> {
        let rows = conn.execute(
            "UPDATE tags SET name = ?1, parent_id = ?2, color = ?3 WHERE id = ?4",
            params![tag.name, tag.parent_id, tag.color, tag.id],
        )?;
        
        Ok(rows)
    }
    
    /// Delete tag
    pub fn delete(conn: &Connection, id: i64) -> Result<usize> {
        let rows = conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(rows)
    }
    
    /// Get tag by name
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Tag>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, color, created_at FROM tags WHERE name = ?1"
        )?;
        
        let tag = stmt.query_row(params![name], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                media_count: None,
            })
        }).optional()?;
        
        Ok(tag)
    }
    
    /// Update tag with optional fields
    pub fn update_fields(
        conn: &Connection,
        id: i64,
        name: Option<&str>,
        parent_id: Option<Option<i64>>,
        color: Option<&str>,
    ) -> Result<usize> {
        let mut updates = Vec::new();
        let mut sql_params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        
        if let Some(n) = name {
            updates.push("name = ?");
            sql_params.push(Box::new(n));
        }
        if let Some(pid) = parent_id {
            updates.push("parent_id = ?");
            if let Some(pid_val) = pid {
                sql_params.push(Box::new(pid_val));
            } else {
                sql_params.push(Box::new(None::<i64>));
            }
        }
        if let Some(c) = color {
            updates.push("color = ?");
            sql_params.push(Box::new(c));
        }
        
        if updates.is_empty() {
            return Ok(0);
        }
        
        sql_params.push(Box::new(id));
        let sql = format!("UPDATE tags SET {} WHERE id = ?", updates.join(", "));
        
        let rows = conn.execute(&sql, rusqlite::params_from_iter(sql_params))?;
        Ok(rows)
    }
    
    /// Check if tag has children
    pub fn has_children(conn: &Connection, id: i64) -> Result<bool> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tags WHERE parent_id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
    
    /// Delete tag and all children recursively
    pub fn delete_with_children(conn: &Connection, id: i64) -> Result<usize> {
        // First delete all children (recursive)
        let children: Vec<i64> = conn
            .prepare("SELECT id FROM tags WHERE parent_id = ?1")?
            .query_map(params![id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        
        let mut deleted = 0;
        for child_id in children {
            deleted += Self::delete_with_children(conn, child_id)?;
        }
        
        // Delete this tag
        deleted += Self::delete(conn, id)?;
        Ok(deleted)
    }
    
    /// Merge two tags (move all media associations from source to target)
    pub fn merge_tags(conn: &Connection, source_id: i64, target_id: i64) -> Result<usize> {
        // Use INSERT OR IGNORE to avoid duplicates
        let sql = "
            INSERT OR IGNORE INTO media_tags (media_id, tag_id, created_at)
            SELECT media_id, ?1, MIN(created_at)
            FROM media_tags
            WHERE tag_id = ?2
            GROUP BY media_id
        ";
        
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.execute(params![target_id, source_id])?;
        
        // Delete old associations
        conn.execute("DELETE FROM media_tags WHERE tag_id = ?1", params![source_id])?;
        
        Ok(rows)
    }
}

/// Media-Tag junction repository
pub struct MediaTagRepository;

impl MediaTagRepository {
    /// Add tag to media
    pub fn add(conn: &Connection, media_id: i64, tag_id: i64) -> Result<()> {
        let now = Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO media_tags (media_id, tag_id, created_at) VALUES (?1, ?2, ?3)",
            params![media_id, tag_id, now],
        )?;
        
        Ok(())
    }
    
    /// Remove tag from media
    pub fn remove(conn: &Connection, media_id: i64, tag_id: i64) -> Result<usize> {
        let rows = conn.execute(
            "DELETE FROM media_tags WHERE media_id = ?1 AND tag_id = ?2",
            params![media_id, tag_id],
        )?;
        
        Ok(rows)
    }
    
    /// Get tags for media
    pub fn get_tags_for_media(conn: &Connection, media_id: i64) -> Result<Vec<Tag>> {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.parent_id, t.color, t.created_at
             FROM tags t
             INNER JOIN media_tags mt ON t.id = mt.tag_id
             WHERE mt.media_id = ?1"
        )?;
        
        let tags = stmt.query_map(params![media_id], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                media_count: None,
            })
        })?;
        
        let mut result = Vec::new();
        for tag in tags {
            result.push(tag?);
        }
        
        Ok(result)
    }
    
    /// Get media for tag
    pub fn get_media_for_tag(
        conn: &Connection,
        tag_id: i64,
        pagination: &Pagination,
    ) -> Result<SearchResult<MediaFile>> {
        // Count total
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM media_tags mt
             INNER JOIN media_files mf ON mt.media_id = mf.id
             WHERE mt.tag_id = ?1 AND mf.is_deleted = 0",
            params![tag_id],
            |row| row.get(0),
        )?;
        
        // Get items
        let mut stmt = conn.prepare(
            "SELECT mf.id, mf.file_path, mf.file_type, mf.file_size, mf.width, mf.height,
                    mf.duration, mf.created_at, mf.modified_at, mf.taken_at, mf.device,
                    mf.gps_lat, mf.gps_lon, mf.checksum, mf.thumbnail_path, mf.is_deleted
             FROM media_tags mt
             INNER JOIN media_files mf ON mt.media_id = mf.id
             WHERE mt.tag_id = ?1 AND mf.is_deleted = 0
             ORDER BY mf.taken_at DESC
             LIMIT ?2 OFFSET ?3"
        )?;
        
        let offset = (pagination.page - 1) * pagination.page_size;
        let media_iter = stmt.query_map(
            params![tag_id, pagination.page_size as i64, offset as i64],
            |row| {
                Ok(MediaFile {
                    id: Some(row.get(0)?),
                    file_path: row.get(1)?,
                    file_type: FileType::from_str(&row.get::<_, String>(2)?).unwrap_or(FileType::Image),
                    file_size: row.get(3)?,
                    width: row.get(4)?,
                    height: row.get(5)?,
                    duration: row.get(6)?,
                    created_at: row.get(7)?,
                    modified_at: row.get(8)?,
                    taken_at: row.get(9)?,
                    device: row.get(10)?,
                    gps_lat: row.get(11)?,
                    gps_lon: row.get(12)?,
                    checksum: row.get(13)?,
                    thumbnail_path: row.get(14)?,
                    is_deleted: row.get::<_, i32>(15)? != 0,
                    tags: None,
                })
            },
        )?;
        
        let mut items = Vec::new();
        for media in media_iter {
            items.push(media?);
        }
        
        Ok(SearchResult::new(items, total, pagination.page, pagination.page_size))
    }
    
    /// Batch add tags to media
    pub fn batch_add(conn: &Connection, media_id: i64, tag_ids: &[i64]) -> Result<usize> {
        let now = Utc::now().timestamp();
        let mut count = 0;
        
        for tag_id in tag_ids {
            let rows = conn.execute(
                "INSERT OR IGNORE INTO media_tags (media_id, tag_id, created_at) VALUES (?1, ?2, ?3)",
                params![media_id, tag_id, now],
            )?;
            count += rows;
        }
        
        Ok(count as usize)
    }
}

/// Album repository
pub struct AlbumRepository;

impl AlbumRepository {
    /// Insert a new album
    pub fn insert(conn: &Connection, album: &Album) -> Result<i64> {
        let now = Utc::now().timestamp();
        conn.execute(
            "INSERT INTO albums (name, description, cover_media_id, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![album.name, album.description, album.cover_media_id, now],
        )?;
        
        Ok(conn.last_insert_rowid())
    }
    
    /// Get album by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Album>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, cover_media_id, created_at FROM albums WHERE id = ?1"
        )?;
        
        let album = stmt.query_row(params![id], |row| {
            Ok(Album {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                cover_media_id: row.get(3)?,
                created_at: row.get(4)?,
                media_count: None,
            })
        }).optional()?;
        
        Ok(album)
    }
    
    /// Get all albums
    pub fn list(conn: &Connection) -> Result<Vec<Album>> {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.name, a.description, a.cover_media_id, a.created_at,
                    COUNT(am.media_id) as media_count
             FROM albums a
             LEFT JOIN album_media am ON a.id = am.album_id
             GROUP BY a.id
             ORDER BY a.created_at DESC"
        )?;
        
        let albums = stmt.query_map([], |row| {
            Ok(Album {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                cover_media_id: row.get(3)?,
                created_at: row.get(4)?,
                media_count: Some(row.get(5)?),
            })
        })?;
        
        let mut result = Vec::new();
        for album in albums {
            result.push(album?);
        }
        
        Ok(result)
    }
    
    /// Update album
    pub fn update(conn: &Connection, album: &Album) -> Result<usize> {
        let rows = conn.execute(
            "UPDATE albums SET name = ?1, description = ?2, cover_media_id = ?3 WHERE id = ?4",
            params![album.name, album.description, album.cover_media_id, album.id],
        )?;
        
        Ok(rows)
    }
    
    /// Delete album
    pub fn delete(conn: &Connection, id: i64) -> Result<usize> {
        let rows = conn.execute("DELETE FROM albums WHERE id = ?1", params![id])?;
        Ok(rows)
    }
}

/// Album-Media junction repository
pub struct AlbumMediaRepository;

impl AlbumMediaRepository {
    /// Add media to album
    pub fn add(conn: &Connection, album_id: i64, media_id: i64, position: i32) -> Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO album_media (album_id, media_id, position) VALUES (?1, ?2, ?3)",
            params![album_id, media_id, position],
        )?;
        
        Ok(())
    }
    
    /// Remove media from album
    pub fn remove(conn: &Connection, album_id: i64, media_id: i64) -> Result<usize> {
        let rows = conn.execute(
            "DELETE FROM album_media WHERE album_id = ?1 AND media_id = ?2",
            params![album_id, media_id],
        )?;
        
        Ok(rows)
    }
    
    /// Get media for album
    pub fn get_media_for_album(
        conn: &Connection,
        album_id: i64,
    ) -> Result<Vec<MediaFile>> {
        let mut stmt = conn.prepare(
            "SELECT mf.id, mf.file_path, mf.file_type, mf.file_size, mf.width, mf.height,
                    mf.duration, mf.created_at, mf.modified_at, mf.taken_at, mf.device,
                    mf.gps_lat, mf.gps_lon, mf.checksum, mf.thumbnail_path, mf.is_deleted
             FROM album_media am
             INNER JOIN media_files mf ON am.media_id = mf.id
             WHERE am.album_id = ?1 AND mf.is_deleted = 0
             ORDER BY am.position"
        )?;
        
        let media_iter = stmt.query_map(params![album_id], |row| {
            Ok(MediaFile {
                id: Some(row.get(0)?),
                file_path: row.get(1)?,
                file_type: FileType::from_str(&row.get::<_, String>(2)?).unwrap_or(FileType::Image),
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                duration: row.get(6)?,
                created_at: row.get(7)?,
                modified_at: row.get(8)?,
                taken_at: row.get(9)?,
                device: row.get(10)?,
                gps_lat: row.get(11)?,
                gps_lon: row.get(12)?,
                checksum: row.get(13)?,
                thumbnail_path: row.get(14)?,
                is_deleted: row.get::<_, i32>(15)? != 0,
                tags: None,
            })
        })?;
        
        let mut result = Vec::new();
        for media in media_iter {
            result.push(media?);
        }
        
        Ok(result)
    }
}

/// Get database statistics
pub fn get_stats(conn: &Connection) -> Result<DatabaseStats> {
    use crate::models::DatabaseStats;
    
    let mut stats = DatabaseStats::default();
    
    // Total files
    conn.query_row("SELECT COUNT(*) FROM media_files WHERE is_deleted = 0", [], |row| {
        stats.total_files = row.get(0)?;
        Ok(())
    })?;
    
    // Total images
    conn.query_row("SELECT COUNT(*) FROM media_files WHERE file_type = 'image' AND is_deleted = 0", [], |row| {
        stats.total_images = row.get(0)?;
        Ok(())
    })?;
    
    // Total videos
    conn.query_row("SELECT COUNT(*) FROM media_files WHERE file_type = 'video' AND is_deleted = 0", [], |row| {
        stats.total_videos = row.get(0)?;
        Ok(())
    })?;
    
    // Total size
    conn.query_row("SELECT COALESCE(SUM(file_size), 0) FROM media_files WHERE is_deleted = 0", [], |row| {
        stats.total_size = row.get(0)?;
        Ok(())
    })?;
    
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

// Helper function for FileType
impl FileType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "image" => Some(FileType::Image),
            "video" => Some(FileType::Video),
            _ => None,
        }
    }
}
