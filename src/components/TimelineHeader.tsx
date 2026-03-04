import type { DatabaseStats } from '../types'
import './TimelineHeader.css'

interface TimelineHeaderProps {
  stats: DatabaseStats | null
  onScan: () => void
  isSelectionMode: boolean
  toggleSelectionMode: () => void
  selectedCount: number
  onOpenFilter: () => void
  hasActiveFilters: boolean
}

export function TimelineHeader({
  stats,
  onScan,
  isSelectionMode,
  toggleSelectionMode,
  selectedCount,
  onOpenFilter,
  hasActiveFilters,
}: TimelineHeaderProps) {
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
          {stats && !isSelectionMode && (
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
          
          {isSelectionMode ? (
            <div className="selection-mode-info">
              <span className="selected-badge">
                📁 {selectedCount} selected
              </span>
            </div>
          ) : (
            <>
              <button
                className={`filter-btn ${hasActiveFilters ? 'active' : ''}`}
                onClick={onOpenFilter}
                title="Filter and search"
              >
                🔍
              </button>
              
              <button className="scan-btn" onClick={onScan}>
                📂 Scan Directory
              </button>
              
              <button className="settings-btn" title="Settings">
                ⚙️
              </button>
            </>
          )}
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
