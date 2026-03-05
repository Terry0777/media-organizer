//! File Scanner Service
//!
//! Scans directories for media files and extracts metadata

use chrono::Utc;
use image::ImageReader;
use kamadak_exif;
use log::{info, warn};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::*;

/// Supported image extensions
const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "heic", "raw", "cr2", "nef", "arw",
];

/// Supported video extensions
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm", "m4v"];

/// File scanner configuration
pub struct ScannerConfig {
    pub recursive: bool,
    pub extract_metadata: bool,
    pub generate_thumbnail: bool,
    pub thumbnail_size: u32,
    pub calculate_checksum: bool, // Only calculate when needed (e.g., deduplication)
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            extract_metadata: true,
            generate_thumbnail: false,
            thumbnail_size: 256,
            calculate_checksum: false, // Default to false for performance
        }
    }
}

/// Scan result statistics
#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    pub total_files: u64,
    pub images: u64,
    pub videos: u64,
    pub skipped: u64,
    pub errors: u64,
    pub total_size: u64,
}

/// File scanner service
pub struct FileScanner {
    config: ScannerConfig,
}

impl FileScanner {
    /// Create new file scanner
    pub fn new(config: ScannerConfig) -> Self {
        Self { config }
    }

    /// Scan directory for media files
    pub fn scan_directory(&self, path: &Path) -> Result<(Vec<MediaFile>, ScanStats), String> {
        info!("Scanning directory: {:?}", path);

        if !path.exists() {
            return Err(format!("Directory does not exist: {:?}", path));
        }

        let mut media_files = Vec::new();
        let mut stats = ScanStats::default();

        let walker = WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'));

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        match self.process_file(entry.path()) {
                            Ok(Some(media)) => {
                                stats.total_files += 1;
                                stats.total_size += media.file_size as u64;

                                match media.file_type {
                                    FileType::Image => stats.images += 1,
                                    FileType::Video => stats.videos += 1,
                                }

                                media_files.push(media);
                            }
                            Ok(None) => {
                                stats.skipped += 1;
                            }
                            Err(e) => {
                                stats.errors += 1;
                                warn!("Error processing {:?}: {}", entry.path(), e);
                            }
                        }
                    }
                }
                Err(e) => {
                    stats.errors += 1;
                    warn!("Error reading entry: {}", e);
                }
            }
        }

        info!(
            "Scan completed: {} files ({} images, {} videos), {} skipped, {} errors",
            stats.total_files, stats.images, stats.videos, stats.skipped, stats.errors
        );

        Ok((media_files, stats))
    }

    /// Process single file (optimized for speed)
    fn process_file(&self, path: &Path) -> Result<Option<MediaFile>, String> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .ok_or("No file extension")?;

        // Determine file type
        let file_type = if IMAGE_EXTENSIONS.contains(&extension.as_str()) {
            FileType::Image
        } else if VIDEO_EXTENSIONS.contains(&extension.as_str()) {
            FileType::Video
        } else {
            return Ok(None); // Not a supported media file
        };

        // Get file metadata
        let file_meta =
            fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;

        let file_size = file_meta.len() as i64;
        let created_at = file_meta
            .created()
            .map(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64
            })
            .unwrap_or_else(|_| Utc::now().timestamp() as i64);
        let modified_at = file_meta
            .modified()
            .map(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64
            })
            .unwrap_or_else(|_| Utc::now().timestamp() as i64);

        // Calculate checksum only if requested (for deduplication)
        let checksum = if self.config.calculate_checksum {
            Some(self.calculate_checksum(path)?)
        } else {
            None // Skip for performance
        };

        // Extract media-specific metadata
        let (width, height, duration, taken_at, device, gps) = if file_type == FileType::Image {
            self.extract_image_metadata(path, &file_meta)?
        } else {
            (None, None, None, None, None, None)
        };

        let media = MediaFile {
            id: None,
            file_path: path.to_string_lossy().to_string(),
            file_type,
            file_size,
            width,
            height,
            duration,
            created_at,
            modified_at,
            taken_at,
            device,
            gps_lat: gps.as_ref().map(|g| g.0),
            gps_lon: gps.as_ref().map(|g| g.1),
            checksum,
            thumbnail_path: None,
            is_deleted: false,
            tags: Some(vec![]),
        };

        Ok(Some(media))
    }

    /// Extract image metadata
    fn extract_image_metadata(
        &self,
        path: &Path,
        _file_meta: &fs::Metadata,
    ) -> Result<
        (
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i64>,
            Option<String>,
            Option<(f64, f64)>,
        ),
        String,
    > {
        let mut width = None;
        let mut height = None;
        let mut taken_at = None;
        let mut device = None;
        let mut gps = None;

        // Try to read image dimensions
        if let Ok(img) = ImageReader::open(path) {
            if let Ok(dimensions) = img.into_dimensions() {
                width = Some(dimensions.0 as i32);
                height = Some(dimensions.1 as i32);
            }
        }

        // Try to extract EXIF data
        if self.config.extract_metadata {
            let file = File::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
            let mut buf_reader = BufReader::new(file);

            // FIX: Correct API usage for kamadak-exif 0.5.x
            // read_from_container returns the populated Reader directly
            match kamadak_exif::Reader::new().read_from_container(&mut buf_reader) {
                Ok(exif_reader) => {
                    // Extract date/time
                    if let Some(field) = exif_reader
                        .get_field(kamadak_exif::Tag::DateTime, kamadak_exif::In::PRIMARY)
                    {
                        let date_str: String = field.display_value().to_string();
                        if let Some(clean_date) = date_str.strip_prefix("ASCII, ") {
                            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(
                                clean_date,
                                "%Y:%m:%d %H:%M:%S",
                            ) {
                                taken_at = Some(dt.and_utc().timestamp() as i64);
                            }
                        }
                    }

                    // Extract camera model
                    if let Some(field) =
                        exif_reader.get_field(kamadak_exif::Tag::Model, kamadak_exif::In::PRIMARY)
                    {
                        let device_str: String = field.display_value().to_string();
                        device = Some(device_str);
                    }

                    // Extract GPS
                    if let (Some(_lat), Some(_lon)) = (
                        exif_reader
                            .get_field(kamadak_exif::Tag::GPSLatitude, kamadak_exif::In::PRIMARY),
                        exif_reader
                            .get_field(kamadak_exif::Tag::GPSLongitude, kamadak_exif::In::PRIMARY),
                    ) {
                        // Simplified GPS parsing (full implementation would handle degrees/minutes/seconds)
                        // For now, skip complex GPS parsing
                        warn!("GPS data found but not parsed (complex format)");
                    }
                }
                Err(e) => {
                    warn!("Failed to parse EXIF data: {}", e);
                }
            }
        }

        Ok((width, height, None, taken_at, device, gps))
    }

    /// Calculate file checksum
    fn calculate_checksum(&self, path: &Path) -> Result<String, String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher).map_err(|e| format!("Failed to read file: {}", e))?;

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Generate thumbnail for image
    pub fn generate_thumbnail(&self, path: &Path, output_path: &Path) -> Result<(), String> {
        if !self.config.generate_thumbnail {
            return Ok(());
        }

        use image::ImageFormat;

        // Determine output format from extension
        let format = match output_path.extension().and_then(|e| e.to_str()) {
            Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
            Some("png") => ImageFormat::Png,
            Some("webp") => ImageFormat::WebP,
            _ => ImageFormat::Jpeg, // Default to JPEG
        };

        // Open and resize image
        let img = ImageReader::open(path)
            .map_err(|e| format!("Failed to open image: {}", e))?
            .decode()
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        let thumbnail = img.resize_exact(
            self.config.thumbnail_size,
            self.config.thumbnail_size,
            image::imageops::FilterType::Lanczos3,
        );

        // Save thumbnail
        let mut output_file = File::create(output_path)
            .map_err(|e| format!("Failed to create thumbnail file: {}", e))?;
        thumbnail
            .write_to(&mut output_file, format)
            .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

        info!("Generated thumbnail: {:?}", output_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = FileScanner::new(ScannerConfig::default());
        assert!(true);
    }

    #[test]
    fn test_supported_extensions() {
        assert!(IMAGE_EXTENSIONS.contains(&"jpg"));
        assert!(IMAGE_EXTENSIONS.contains(&"png"));
        assert!(IMAGE_EXTENSIONS.contains(&"heic"));
        assert!(VIDEO_EXTENSIONS.contains(&"mp4"));
        assert!(VIDEO_EXTENSIONS.contains(&"mov"));
    }

    #[test]
    fn test_config_default() {
        let config = ScannerConfig::default();
        assert!(config.recursive);
        assert!(config.extract_metadata);
        assert!(!config.generate_thumbnail);
        assert_eq!(config.thumbnail_size, 256);
    }
}
