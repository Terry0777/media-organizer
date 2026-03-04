import type { MediaFile } from '../types'
import { MediaCard } from './MediaCard'
import './MediaGrid.css'

interface MediaGridProps {
  mediaFiles: MediaFile[]
}

export function MediaGrid({ mediaFiles }: MediaGridProps) {
  // Group media files by date
  const groupedByDate = groupMediaByDate(mediaFiles)

  return (
    <div className="media-grid">
      {Object.entries(groupedByDate).map(([date, files]) => (
        <div key={date} className="media-section">
          <div className="section-header">
            <h3>{formatDate(date)}</h3>
            <span className="section-count">{files.length} files</span>
          </div>
          <div className="media-grid-items">
            {files.map((file) => (
              <MediaCard key={file.id} media={file} />
            ))}
          </div>
        </div>
      ))}
    </div>
  )
}

// Helper functions
function groupMediaByDate(files: MediaFile[]): Record<string, MediaFile[]> {
  const groups: Record<string, MediaFile[]> = {}
  
  files.forEach((file) => {
    // Use taken_at if available, otherwise created_at
    const timestamp = file.taken_at || file.created_at
    const date = new Date(timestamp * 1000).toISOString().split('T')[0]
    
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(file)
  })
  
  // Sort dates in descending order
  return Object.fromEntries(
    Object.entries(groups).sort(([a], [b]) => b.localeCompare(a))
  )
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  const today = new Date()
  const yesterday = new Date(today)
  yesterday.setDate(yesterday.getDate() - 1)
  
  if (date.toDateString() === today.toDateString()) {
    return 'Today'
  } else if (date.toDateString() === yesterday.toDateString()) {
    return 'Yesterday'
  }
  
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
