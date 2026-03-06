//! Thumbnail Generator Service
//!
//! Generates thumbnails for media files asynchronously

use image::{imageops::thumbnail, ImageFormat};
use log::{error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task::JoinHandle;

// 引入 Digest trait
use sha2::Digest;

use crate::models::{FileType, MediaFile};

/// Thumbnail generator configuration
pub struct ThumbnailConfig {
    pub size: u32,
    pub quality: u8,
    pub cache_dir: PathBuf,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            size: 512,
            quality: 85,
            cache_dir: PathBuf::from("thumbnails"),
        }
    }
}

/// Thumbnail generator service
pub struct ThumbnailGenerator {
    config: ThumbnailConfig,
}

impl ThumbnailGenerator {
    /// Create new thumbnail generator
    pub fn new(config: ThumbnailConfig) -> Self {
        Self { config }
    }

    /// Generate thumbnail for a media file (async)
    pub fn generate_thumbnail(
        &self,
        media: &MediaFile,
    ) -> JoinHandle<Result<Option<String>, String>> {
        // [关键修复] 克隆需要的数据，使其拥有所有权，从而可以安全地移入 async 块
        // 我们不需要克隆整个 media 对象如果它很大，只克隆需要的字段也可以，但克隆整个结构体最简单
        let media_owned = media.clone();

        let cache_dir = self.config.cache_dir.clone();
        let size = self.config.size;
        let _quality = self.config.quality;

        tokio::spawn(async move {
            // 现在使用的是 media_owned (拥有所有权)，而不是 media (引用)
            if media_owned.file_type != FileType::Image {
                return Ok(None);
            }

            let media_path = &media_owned.file_path;

            // Create cache directory if not exists
            fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;

            // Generate thumbnail filename from media path hash
            let hash = sha2::Sha256::digest(media_path.as_bytes());
            let thumbnail_filename = format!("{:x}.jpg", hash);
            let thumbnail_path = cache_dir.join(thumbnail_filename);

            // Skip if thumbnail already exists
            if thumbnail_path.exists() {
                info!("Thumbnail already exists: {:?}", thumbnail_path);
                return Ok(Some(thumbnail_path.to_string_lossy().to_string()));
            }

            // Generate thumbnail
            match image::open(media_path) {
                Ok(img) => {
                    let thumb = thumbnail(&img, size, size);

                    // Save as JPEG
                    thumb
                        .save(&thumbnail_path)
                        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

                    info!("Generated thumbnail: {:?}", thumbnail_path);
                    Ok(Some(thumbnail_path.to_string_lossy().to_string()))
                }
                Err(e) => {
                    warn!("Failed to open image {:?}: {}", media_path, e);
                    Ok(None)
                }
            }
        })
    }

    /// Generate thumbnails for multiple media files (batch)
    pub fn generate_thumbnails_batch(
        &self,
        media_files: &[MediaFile],
    ) -> Vec<JoinHandle<Result<Option<String>, String>>> {
        media_files
            .iter()
            .map(|media| self.generate_thumbnail(media))
            .collect()
    }
}
