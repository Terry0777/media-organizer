mod db;
mod models;

use db::{Database, MediaRepository, TagRepository, MediaTagRepository, AlbumRepository, AlbumMediaRepository};
use models::*;
use tauri::Manager;
use std::sync::Arc;
use log::info;

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
    app_state
        .db
        .init()
        .map(|_| true)
        .map_err(|e| e.to_string())
}

/// Get database statistics
#[tauri::command]
fn get_database_stats(app_state: tauri::State<AppState>) -> Result<DatabaseStats, String> {
    let conn = app_state.db.get_conn().map_err(|e| e.to_string())?;
    db::repository::get_stats(&conn).map_err(|e| e.to_string())
}

/// Create a new media file record
#[tauri::command]
fn create_media_file(
    app_state: tauri::State<AppState>,
    media: MediaFile,
) -> Result<i64, String> {
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
            
            app.manage(AppState {
                db: Arc::new(db),
            });
            
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
            add_tag_to_media,
            remove_tag_from_media,
            get_tags_for_media,
            create_album,
            get_albums,
            add_media_to_album,
            get_media_for_album,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
