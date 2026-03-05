import { useEffect, useState } from 'react'
import { open } from '@tauri-apps/plugin-dialog'
import { useTimelineStore } from '../stores/timelineStore'
import { MediaGrid } from '../components/MediaGrid'
import { TimelineHeader } from '../components/TimelineHeader'
import { SelectionToolbar } from '../components/SelectionToolbar'
import { FilterPanel } from '../components/FilterPanel'
import './TimelineView.css'

export function TimelineView() {
  const {
    mediaFiles,
    stats,
    isLoading,
    error,
    isSelectionMode,
    selectedMediaIds,
    currentFilters,
    loadMediaFiles,
    loadStats,
    scanDirectory,
    clearError,
    selectMedia,
    deselectMedia,
  } = useTimelineStore()
  
  const [isFilterOpen, setIsFilterOpen] = useState(false)

  useEffect(() => {
    // Initial load
    loadMediaFiles()
    loadStats()
  }, [])

  const handleScan = async () => {
    // Open folder picker dialog
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select a folder to scan for media files',
    })
    
    if (selected && typeof selected === 'string') {
      await scanDirectory(selected)
    }
  }

  const handleSelectMedia = (mediaId: number) => {
    if (selectedMediaIds.includes(mediaId)) {
      deselectMedia(mediaId)
    } else {
      selectMedia(mediaId)
    }
  }

  const hasActiveFilters = Object.keys(currentFilters).length > 0

  return (
    <div className="timeline-view">
      <TimelineHeader
        stats={stats}
        onScan={handleScan}
        isSelectionMode={isSelectionMode}
        selectedCount={selectedMediaIds.length}
        onOpenFilter={() => setIsFilterOpen(true)}
        hasActiveFilters={hasActiveFilters}
      />
      
      <SelectionToolbar />
      
      <main className="timeline-content">
        {error && (
          <div className="error-banner">
            <span>{error}</span>
            <button onClick={clearError}>✕</button>
          </div>
        )}
        
        {hasActiveFilters && (
          <div className="active-filters-banner">
            <span>🔍 Active filters applied</span>
            <button onClick={() => loadMediaFiles()}>Clear</button>
          </div>
        )}
        
        {isLoading ? (
          <div className="loading-spinner">
            <div className="spinner" />
            <p>Loading media files...</p>
          </div>
        ) : mediaFiles.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">📁</div>
            <h2>No media files found</h2>
            <p>Start by scanning a directory to add your photos and videos</p>
            <button className="primary-btn" onClick={handleScan}>
              Scan Directory
            </button>
          </div>
        ) : (
          <MediaGrid
            mediaFiles={mediaFiles}
            isSelectionMode={isSelectionMode}
            selectedMediaIds={selectedMediaIds}
            onSelectMedia={handleSelectMedia}
          />
        )}
      </main>

      <FilterPanel
        isOpen={isFilterOpen}
        onClose={() => setIsFilterOpen(false)}
      />
    </div>
  )
}
