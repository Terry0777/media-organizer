import { convertFileSrc } from '@tauri-apps/api/core'
import type { MediaFile } from '../types'
import './MediaCard.css'

interface MediaCardProps {
  media: MediaFile
  onClick?: () => void
}

export function MediaCard({ media, onClick }: MediaCardProps) {
  const fileType = media.file_type === 'image' ? '📷' : '🎥'
  const fileSize = formatFileSize(media.file_size)
  
  // Convert thumbnail path to asset URL if available
  const thumbnailSrc = media.thumbnail_path && media.thumbnail_path.trim() !== ''
    ? convertFileSrc(media.thumbnail_path)
    : null
  
  const handleClick = () => {
    if (onClick) {
      onClick()
    }
  }
  
  return (
    <div className="media-card" onClick={handleClick}>
      <div className="media-thumbnail">
        {thumbnailSrc ? (
          <img src={thumbnailSrc} alt={media.file_path} loading="lazy" />
        ) : (
          <div className="thumbnail-placeholder">
            <span className="file-type">{fileType}</span>
          </div>
        )}
        
        {media.file_type === 'video' && (
          <div className="video-indicator">▶</div>
        )}
      </div>
      
      <div className="media-info">
        <div className="media-meta">
          <span className="media-type">{fileType}</span>
          <span className="media-size">{fileSize}</span>
        </div>
        
        {media.width && media.height && (
          <div className="media-dimensions">
            {media.width}×{media.height}
          </div>
        )}
        
        <div className="media-date">
          {formatMediaDate(media.taken_at || media.created_at)}
        </div>
      </div>
    </div>
  )
}

// Helper functions
function formatFileSize(bytes: number): string {
  const mb = bytes / (1024 * 1024)
  if (mb >= 100) {
    return `${Math.round(mb)} MB`
  } else if (mb >= 1) {
    return `${mb.toFixed(1)} MB`
  } else {
    return `${Math.round(bytes / 1024)} KB`
  }
}

function formatMediaDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  })
}
