import type { DatabaseStats } from '../types'
import './TimelineHeader.css'

interface TimelineHeaderProps {
  stats: DatabaseStats | null
  onScan: () => void
}

export function TimelineHeader({ stats, onScan }: TimelineHeaderProps) {
  return (
    <header className="timeline-header">
      <div className="header-content">
        <div className="logo">
          <span className="logo-icon">🦞</span>
          <h1>Media Organizer</h1>
        </div>
        
        <div className="search-bar">
          <input
            type="text"
            placeholder="🔍 Search files, tags..."
            disabled
            title="Search coming soon"
          />
        </div>
        
        <div className="header-actions">
          {stats && (
            <div className="stats-summary">
              <span className="stat-item">
                📁 {formatNumber(stats.total_files)} files
              </span>
              <span className="stat-item">
                📷 {formatNumber(stats.total_images)} images
              </span>
              <span className="stat-item">
                🎥 {formatNumber(stats.total_videos)} videos
              </span>
            </div>
          )}
          
          <button className="scan-btn" onClick={onScan}>
            📂 Scan Directory
          </button>
          
          <button className="settings-btn" title="Settings">
            ⚙️
          </button>
        </div>
      </div>
    </header>
  )
}

function formatNumber(num: number): string {
  if (num >= 1000000) {
    return `${(num / 1000000).toFixed(1)}M`
  } else if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}K`
  }
  return num.toString()
}
