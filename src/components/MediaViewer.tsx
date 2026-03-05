import { useState, useEffect } from 'react'
import { convertFileSrc } from '@tauri-apps/api/core'
import type { MediaFile } from '../types'
import './MediaViewer.css'

interface MediaViewerProps {
  media: MediaFile
  onClose: () => void
  onNext?: () => void
  onPrev?: () => void
}

export function MediaViewer({ media, onClose, onNext, onPrev }: MediaViewerProps) {
  const [isLoading, setIsLoading] = useState(true)

  // Handle keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose()
      } else if (e.key === 'ArrowRight' && onNext) {
        onNext()
      } else if (e.key === 'ArrowLeft' && onPrev) {
        onPrev()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [onClose, onNext, onPrev])

  const mediaSrc = convertFileSrc(media.file_path)

  return (
    <div className="media-viewer-overlay" onClick={onClose}>
      <div className="media-viewer" onClick={(e) => e.stopPropagation()}>
        <header className="viewer-header">
          <div className="viewer-title">
            {media.file_type === 'image' ? '📷' : '🎥'} {media.file_path.split('/').pop()}
          </div>
          <button className="viewer-close" onClick={onClose}>✕</button>
        </header>

        <div className="viewer-content">
          {isLoading && (
            <div className="viewer-loading">
              <div className="spinner" />
              <p>Loading...</p>
            </div>
          )}
          
          {media.file_type === 'image' ? (
            <img
              src={mediaSrc}
              alt={media.file_path}
              className={`viewer-media ${isLoading ? 'hidden' : ''}`}
              onLoad={() => setIsLoading(false)}
            />
          ) : (
            <video
              src={mediaSrc}
              className={`viewer-media ${isLoading ? 'hidden' : ''}`}
              controls
              autoPlay
              onLoadedData={() => setIsLoading(false)}
            />
          )}
        </div>

        <footer className="viewer-footer">
          <div className="viewer-info">
            <span>{media.width}×{media.height}</span>
            <span>{formatFileSize(media.file_size)}</span>
            <span>{formatMediaDate(media.taken_at || media.created_at)}</span>
          </div>
          
          <div className="viewer-controls">
            {onPrev && (
              <button className="viewer-btn" onClick={onPrev} title="Previous">
                ◀ Prev
              </button>
            )}
            {onNext && (
              <button className="viewer-btn" onClick={onNext} title="Next">
                Next ▶
              </button>
            )}
          </div>
        </footer>
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
    hour: '2-digit',
    minute: '2-digit',
  })
}
