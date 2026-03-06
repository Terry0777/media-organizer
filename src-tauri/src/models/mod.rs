//! Database models
//! 
//! Data structures for media files, tags, and albums

use serde::{Deserialize, Serialize};

/// Media file types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Image,
    Video,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            ".jpg" | ".jpeg" | ".png" | ".gif" | ".webp" | ".heic" | ".raw" | ".tiff" => {
                Some(FileType::Image)
            }
            ".mp4" | ".mov" | ".avi" | ".mkv" | ".webm" | ".flv" | ".wmv" => {
                Some(FileType::Video)
            }
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Image => "image",
            FileType::Video => "video",
        }
    }
}

/// Media file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: Option<i64>,
    pub file_path: String,
    pub file_type: FileType,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub created_at: i64,
    pub modified_at: i64,
    pub taken_at: Option<i64>,
    pub device: Option<String>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
    pub checksum: Option<String>,
    pub thumbnail_path: Option<String>,
    pub is_deleted: bool,
    
    // Relations (not stored in DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// Tag for organizing media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
    pub created_at: i64,
    
    // Relations (not stored in DB)
    #[serde(skip_serializing)]
    pub media_count: Option<i64>,
}

/// Album (collection of media files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub cover_media_id: Option<i64>,
    pub created_at: i64,
    
    // Relations (not stored in DB)
    #[serde(skip_serializing)]
    pub media_count: Option<i64>,
}

/// Junction table: media <-> tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTag {
    pub media_id: i64,
    pub tag_id: i64,
    pub created_at: i64,
}

/// Junction table: album <-> media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumMedia {
    pub album_id: i64,
    pub media_id: i64,
    pub position: i32,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub file_type: Option<FileType>,
    pub tag_ids: Option<Vec<i64>>,
    pub album_id: Option<i64>,
    pub date_from: Option<i64>,
    pub date_to: Option<i64>,
    pub size_min: Option<i64>,
    pub size_max: Option<i64>,
    pub folder: Option<String>,
    pub is_deleted: Option<bool>,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 50,
        }
    }
}

/// Search result with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> SearchResult<T> {
    pub fn new(items: Vec<T>, total: i64, page: usize, page_size: usize) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as usize;
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

/// Tag usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagUsage {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
    pub usage_count: i64,
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseStats {
    pub total_files: i64,
    pub total_images: i64,
    pub total_videos: i64,
    pub total_size: i64,
    pub total_tags: i64,
    pub total_albums: i64,
}

/// Scan result statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanStats {
    pub total_files: u64,
    pub images: u64,
    pub videos: u64,
    pub skipped: u64,
    pub errors: u64,
    pub total_size: u64,
    pub inserted: u64,
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub message: String,
    pub stats: ScanStats,
}

/// Scan progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub is_scanning: bool,
    pub current_file: Option<String>,
    pub processed: u64,
    pub total: u64,
    pub percentage: f64,
}

/// Thumbnail generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailGenerationResult {
    pub total: usize,
    pub generated: usize,
    pub errors: usize,
    pub skipped: usize,
}
