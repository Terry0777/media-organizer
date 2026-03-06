// Tauri API types
export interface MediaFile {
  id: number;
  file_path: string;
  file_type: 'image' | 'video';
  file_size: number;
  width?: number;
  height?: number;
  duration?: number;
  created_at: number;
  modified_at: number;
  taken_at?: number;
  device?: string;
  gps_lat?: number;
  gps_lon?: number;
  checksum?: string;
  thumbnail_path?: string;
  is_deleted: boolean;
  tags?: Tag[];
}

export interface Tag {
  id: number;
  name: string;
  parent_id?: number;
  color?: string;
  created_at: number;
  media_count?: number;
}

export interface TagTreeNode extends Tag {
  children: TagTreeNode[];
}

export interface Album {
  id: number;
  name: string;
  description?: string;
  cover_media_id?: number;
  created_at: number;
  media_count?: number;
}

export interface SearchFilters {
  file_type?: 'image' | 'video';
  tag_ids?: number[];
  tag_logic?: 'AND' | 'OR';
  album_id?: number;
  date_from?: number;
  date_to?: number;
  size_min?: number;
  size_max?: number;
  folder?: string;
  is_deleted?: boolean;
}

export interface Pagination {
  page: number;
  page_size: number;
}

export interface SearchResult<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export interface ScanStats {
  total_files: number;
  images: number;
  videos: number;
  skipped: number;
  errors: number;
  total_size: number;
  inserted: number;
}

export interface ScanResult {
  message: string;
  stats: ScanStats;
}

export interface DatabaseStats {
  total_files: number;
  total_images: number;
  total_videos: number;
  total_size: number;
  total_tags: number;
  total_albums: number;
}

/**
 * Thumbnail generation result
 */
export interface ThumbnailGenerationResult {
  total: number;
  generated: number;
  errors: number;
  skipped: number;
}
