import { invoke } from '@tauri-apps/api/core'
import type {
  MediaFile,
  Tag,
  Album,
  SearchFilters,
  Pagination,
  SearchResult,
  ScanResult,
  DatabaseStats,
} from './types'

/**
 * Initialize database
 */
export async function initDatabase(): Promise<boolean> {
  return invoke('init_database')
}

/**
 * Get database statistics
 */
export async function getDatabaseStats(): Promise<DatabaseStats> {
  return invoke('get_database_stats')
}

/**
 * Scan directory for media files
 */
export async function scanDirectory(
  path: string,
  recursive = true,
  extractMetadata = true
): Promise<ScanResult> {
  return invoke('scan_directory', { path, recursive, extractMetadata })
}

/**
 * Get media files with filters
 */
export async function getMediaFiles(
  filters: SearchFilters = {},
  pagination: Pagination = { page: 1, page_size: 50 }
): Promise<SearchResult<MediaFile>> {
  return invoke('get_media_files', { filters, pagination })
}

/**
 * Get media file by ID
 */
export async function getMediaFileById(id: number): Promise<MediaFile | null> {
  return invoke('get_media_file_by_id', { id })
}

/**
 * Create a new media file record
 */
export async function createMediaFile(media: MediaFile): Promise<number> {
  return invoke('create_media_file', { media })
}

/**
 * Get all tags
 */
export async function getTags(): Promise<Tag[]> {
  return invoke('get_tags')
}

/**
 * Get tag usage statistics
 */
export async function getTagUsage(): Promise<Array<Tag & { usage_count: number }>> {
  return invoke('get_tag_usage')
}

/**
 * Create a new tag
 */
export async function createTag(
  name: string,
  parentId?: number,
  color?: string
): Promise<number> {
  return invoke('create_tag', { name, parent_id: parentId, color })
}

/**
 * Add tag to media
 */
export async function addTagToMedia(mediaId: number, tagId: number): Promise<void> {
  return invoke('add_tag_to_media', { media_id: mediaId, tag_id: tagId })
}

/**
 * Remove tag from media
 */
export async function removeTagFromMedia(
  mediaId: number,
  tagId: number
): Promise<number> {
  return invoke('remove_tag_from_media', { media_id: mediaId, tag_id: tagId })
}

/**
 * Get tags for media
 */
export async function getTagsForMedia(mediaId: number): Promise<Tag[]> {
  return invoke('get_tags_for_media', { media_id: mediaId })
}

/**
 * Create a new album
 */
export async function createAlbum(
  name: string,
  description?: string,
  coverMediaId?: number
): Promise<number> {
  return invoke('create_album', { name, description, cover_media_id: coverMediaId })
}

/**
 * Get all albums
 */
export async function getAlbums(): Promise<Album[]> {
  return invoke('get_albums')
}

/**
 * Add media to album
 */
export async function addMediaToAlbum(
  albumId: number,
  mediaId: number,
  position = 0
): Promise<void> {
  return invoke('add_media_to_album', { album_id: albumId, media_id: mediaId, position })
}

/**
 * Get media for album
 */
export async function getMediaForAlbum(albumId: number): Promise<MediaFile[]> {
  return invoke('get_media_for_album', { album_id: albumId })
}
