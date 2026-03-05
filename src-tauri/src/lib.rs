mod db;
mod models;
mod services;

use db::{
    AlbumMediaRepository, AlbumRepository, Database, MediaRepository, MediaTagRepository,
    TagRepository,
};
use log::{info, warn};
use models::*;
use services::{FileScanner, ScannerConfig};
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

/// Application state shared across Tauri commands
pub struct AppState {
    pub db: Arc<Database>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Initialize database
#[tauri::command]
fn init_database(app_state: tauri::State<AppState>) -> Result<bool, String> {
    info!("Initializing database");
    app_state.db.init().map(|_| true).map_err(|e| e.to_string())
}

/// Get database statistics
#[tauri::command]
fn get_database_stats(app_state: tauri::State<AppState>) -> Result<DatabaseStats, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    db::repository::get_stats(&conn).map_err(|e| e.to_string())
}

/// Create a new media file record
#[tauri::command]
fn create_media_file(app_state: tauri::State<AppState>, media: MediaFile) -> Result<i64, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaRepository::insert(&conn, &media).map_err(|e| e.to_string())
}

/// Get media files with filters
#[tauri::command]
fn get_media_files(
    app_state: tauri::State<AppState>,
    filters: SearchFilters,
    pagination: Pagination,
) -> Result<SearchResult<MediaFile>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaRepository::list(&conn, &filters, &pagination).map_err(|e| e.to_string())
}

/// Get media file by ID
#[tauri::command]
fn get_media_file_by_id(
    app_state: tauri::State<AppState>,
    id: i64,
) -> Result<Option<MediaFile>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaRepository::get_by_id(&conn, id).map_err(|e| e.to_string())
}

/// Create a new tag
#[tauri::command]
fn create_tag(
    app_state: tauri::State<AppState>,
    name: String,
    parent_id: Option<i64>,
    color: Option<String>,
) -> Result<i64, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    let tag = Tag {
        id: None,
        name,
        parent_id,
        color,
        created_at: 0,
        media_count: None,
    };
    TagRepository::insert(&conn, &tag).map_err(|e| e.to_string())
}

/// Get all tags
#[tauri::command]
fn get_tags(app_state: tauri::State<AppState>) -> Result<Vec<Tag>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    TagRepository::list(&conn).map_err(|e| e.to_string())
}

/// Get tag usage statistics
#[tauri::command]
fn get_tag_usage(app_state: tauri::State<AppState>) -> Result<Vec<TagUsage>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    TagRepository::get_usage(&conn).map_err(|e| e.to_string())
}

/// Update a tag
#[tauri::command]
fn update_tag(
    app_state: tauri::State<AppState>,
    id: i64,
    name: Option<String>,
    parent_id: Option<Option<i64>>,
    color: Option<String>,
) -> Result<(), String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    TagRepository::update_fields(&conn, id, name.as_deref(), parent_id, color.as_deref())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Delete a tag
#[tauri::command]
fn delete_tag(
    app_state: tauri::State<AppState>,
    id: i64,
    delete_children: bool,
) -> Result<usize, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;

    if delete_children {
        // Delete all descendants first
        TagRepository::delete_with_children(&conn, id).map_err(|e| e.to_string())
    } else {
        // Check if has children
        let has_children = TagRepository::has_children(&conn, id).map_err(|e| e.to_string())?;
        if has_children {
            return Err("Tag has child tags. Set delete_children=true to delete all.".to_string());
        }
        TagRepository::delete(&conn, id).map_err(|e| e.to_string())
    }
}

/// Merge two tags (move all media from source to target)
#[tauri::command]
fn merge_tags(
    app_state: tauri::State<AppState>,
    source_tag_id: i64,
    target_tag_id: i64,
) -> Result<usize, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    TagRepository::merge_tags(&conn, source_tag_id, target_tag_id).map_err(|e| e.to_string())
}

/// Get suggested tags based on media metadata
#[tauri::command]
fn get_suggested_tags(
    app_state: tauri::State<AppState>,
    media_path: String,
) -> Result<Vec<Tag>, String> {
    // TODO: Implement AI-based tag suggestions
    // For now, return empty list
    let _ = media_path;
    let _ = app_state;
    Ok(Vec::new())
}

/// Add tag to media
#[tauri::command]
fn add_tag_to_media(
    app_state: tauri::State<AppState>,
    media_id: i64,
    tag_id: i64,
) -> Result<(), String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaTagRepository::add(&conn, media_id, tag_id).map_err(|e| e.to_string())
}

/// Remove tag from media
#[tauri::command]
fn remove_tag_from_media(
    app_state: tauri::State<AppState>,
    media_id: i64,
    tag_id: i64,
) -> Result<usize, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaTagRepository::remove(&conn, media_id, tag_id).map_err(|e| e.to_string())
}

/// Scan directory for media files
#[tauri::command]
async fn scan_directory(
    app_state: tauri::State<'_, AppState>,
    path: String,
    recursive: bool,
    extract_metadata: bool,
) -> Result<ScanResult, String> {
    use std::path::Path;

    info!(
        "Scanning directory: {} (recursive={}, metadata={})",
        path, recursive, extract_metadata
    );

    let config = ScannerConfig {
        recursive,
        extract_metadata,
        generate_thumbnail: false,
        thumbnail_size: 256,
    };

    let scanner = FileScanner::new(config);
    let path = Path::new(&path);

    let (media_files, stats) = scanner.scan_directory(path)?;

    // Insert media files into database
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    let mut inserted_count = 0;

    for media in media_files {
        match MediaRepository::insert(&conn, &media) {
            Ok(_) => inserted_count += 1,
            Err(e) => {
                warn!("Failed to insert media {:?}: {}", media.file_path, e);
            }
        }
    }

    info!("Scan completed: {} files inserted", inserted_count);

    Ok(ScanResult {
        message: format!("Successfully scanned {} files", stats.total_files),
        stats: ScanStats {
            total_files: stats.total_files,
            images: stats.images,
            videos: stats.videos,
            skipped: stats.skipped,
            errors: stats.errors,
            total_size: stats.total_size,
            inserted: inserted_count as u64,
        },
    })
}

/// Get scan progress (placeholder for future async progress reporting)
#[tauri::command]
fn get_scan_progress() -> Result<ScanProgress, String> {
    Ok(ScanProgress {
        is_scanning: false,
        current_file: None,
        processed: 0,
        total: 0,
        percentage: 0.0,
    })
}

/// Get tags for media
#[tauri::command]
fn get_tags_for_media(
    app_state: tauri::State<AppState>,
    media_id: i64,
) -> Result<Vec<Tag>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    MediaTagRepository::get_tags_for_media(&conn, media_id).map_err(|e| e.to_string())
}

/// Create a new album
#[tauri::command]
fn create_album(
    app_state: tauri::State<AppState>,
    name: String,
    description: Option<String>,
    cover_media_id: Option<i64>,
) -> Result<i64, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    let album = Album {
        id: None,
        name,
        description,
        cover_media_id,
        created_at: 0,
        media_count: None,
    };
    AlbumRepository::insert(&conn, &album).map_err(|e| e.to_string())
}

/// Get all albums
#[tauri::command]
fn get_albums(app_state: tauri::State<AppState>) -> Result<Vec<Album>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    AlbumRepository::list(&conn).map_err(|e| e.to_string())
}

/// Add media to album
#[tauri::command]
fn add_media_to_album(
    app_state: tauri::State<AppState>,
    album_id: i64,
    media_id: i64,
    position: i32,
) -> Result<(), String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    AlbumMediaRepository::add(&conn, album_id, media_id, position).map_err(|e| e.to_string())
}

/// Get media for album
#[tauri::command]
fn get_media_for_album(
    app_state: tauri::State<AppState>,
    album_id: i64,
) -> Result<Vec<MediaFile>, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    AlbumMediaRepository::get_media_for_album(&conn, album_id).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            let db_path = app_handle
                .path()
                .app_data_dir()
                .map(|p| p.join("media_organizer.db"))
                .map_err(|e| e.to_string())?;

            info!("Database path: {:?}", db_path);

            let db = Database::open(db_path).map_err(|e| e.to_string())?;
            db.init().map_err(|e| e.to_string())?;

            app.manage(AppState { db: Arc::new(db) });

            info!("Application started successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            get_database_stats,
            create_media_file,
            get_media_files,
            get_media_file_by_id,
            create_tag,
            get_tags,
            get_tag_usage,
            update_tag,
            delete_tag,
            merge_tags,
            get_suggested_tags,
            add_tag_to_media,
            remove_tag_from_media,
            get_tags_for_media,
            create_album,
            get_albums,
            add_media_to_album,
            get_media_for_album,
            scan_directory,
            get_scan_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
