//! Thumbnail Generator Service
//!
//! Generates thumbnails for media files asynchronously

use image::{imageops::thumbnail, ImageFormat};
use log::{info, warn, error};
use std::path::{Path, PathBuf};
use std::fs;
use tokio::task::JoinHandle;

use crate::models::MediaFile;

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
    pub fn generate_thumbnail(&self, media: &MediaFile) -> JoinHandle<Result<Option<String>, String>> {
        let media_path = media.file_path.clone();
        let cache_dir = self.config.cache_dir.clone();
        let size = self.config.size;
        let quality = self.config.quality;
        
        tokio::spawn(async move {
            // Skip if not an image
            if media.file_type != "image" {
                return Ok(None);
            }

            // Create cache directory if not exists
            fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;

            // Generate thumbnail filename from media path hash
            let thumbnail_path = cache_dir.join(format!(
                "{}.jpg",
                sha2::Sha256::digest(media_path.as_bytes())
            ));

            // Skip if thumbnail already exists
            if thumbnail_path.exists() {
                info!("Thumbnail already exists: {:?}", thumbnail_path);
                return Ok(Some(thumbnail_path.to_string_lossy().to_string()));
            }

            // Generate thumbnail
            match image::open(&media_path) {
                Ok(img) => {
                    let thumb = thumbnail(&img, size, size);
                    
                    // Save as JPEG
                    thumb.write_with_format(
                        &thumbnail_path,
                        ImageFormat::Jpeg,
                    )
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
