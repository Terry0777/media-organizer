import { useEffect } from 'react'
import { useTimelineStore } from '../stores/timelineStore'
import { MediaGrid } from '../components/MediaGrid'
import { TimelineHeader } from '../components/TimelineHeader'
import { SelectionToolbar } from '../components/SelectionToolbar'
import './TimelineView.css'

export function TimelineView() {
  const {
    mediaFiles,
    stats,
    isLoading,
    error,
    isSelectionMode,
    selectedMediaIds,
    loadMediaFiles,
    loadStats,
    scanDirectory,
    clearError,
    toggleSelectionMode,
    selectMedia,
    deselectMedia,
  } = useTimelineStore()

  useEffect(() => {
    // Initial load
    loadMediaFiles()
    loadStats()
  }, [])

  const handleScan = async () => {
    // In a real app, this would open a file picker dialog
    const path = prompt('Enter directory path to scan:')
    if (path) {
      await scanDirectory(path)
    }
  }

  const handleSelectMedia = (mediaId: number) => {
    if (selectedMediaIds.includes(mediaId)) {
      deselectMedia(mediaId)
    } else {
      selectMedia(mediaId)
    }
  }

  return (
    <div className="timeline-view">
      <TimelineHeader
        stats={stats}
        onScan={handleScan}
        isSelectionMode={isSelectionMode}
        toggleSelectionMode={toggleSelectionMode}
        selectedCount={selectedMediaIds.length}
      />
      
      <SelectionToolbar />
      
      <main className="timeline-content">
        {error && (
          <div className="error-banner">
            <span>{error}</span>
            <button onClick={clearError}>✕</button>
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
    </div>
  )
}
