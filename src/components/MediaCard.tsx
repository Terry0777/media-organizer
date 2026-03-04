import type { MediaFile } from '../types'
import './MediaCard.css'

interface MediaCardProps {
  media: MediaFile
}

export function MediaCard({ media }: MediaCardProps) {
  const fileType = media.file_type === 'image' ? '📷' : '🎥'
  const fileSize = formatFileSize(media.file_size)
  
  const handleClick = () => {
    // TODO: Open media viewer
    console.log('Clicked media:', media.id)
  }
  
  return (
    <div className="media-card" onClick={handleClick}>
      <div className="media-thumbnail">
        {media.thumbnail_path ? (
          <img src={media.thumbnail_path} alt={media.file_path} loading="lazy" />
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
