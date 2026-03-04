import { useTimelineStore } from '../stores/timelineStore'
import { BatchTagModal } from './BatchTagModal'
import './SelectionToolbar.css'

export function SelectionToolbar() {
  const {
    selectedMediaIds,
    isSelectionMode,
    toggleSelectionMode,
    selectAllMedia,
    deselectAllMedia,
    openBatchTaggingModal,
    closeBatchTaggingModal,
    isBatchTaggingModalOpen,
  } = useTimelineStore()

  const selectedCount = selectedMediaIds.length

  const handleBatchTag = () => {
    if (selectedCount > 0) {
      openBatchTaggingModal()
    }
  }

  return (
    <>
      {isSelectionMode && (
        <div className="selection-toolbar">
          <div className="toolbar-info">
            <span className="selected-count">
              📁 Selected: <strong>{selectedCount}</strong> files
            </span>
          </div>

          <div className="toolbar-actions">
            <button
              className="toolbar-btn"
              onClick={selectAllMedia}
              title="Select all visible files"
            >
              ✅ Select All
            </button>
            <button
              className="toolbar-btn"
              onClick={deselectAllMedia}
              title="Deselect all files"
            >
              ❌ Deselect All
            </button>
            <div className="toolbar-divider" />
            <button
              className="toolbar-btn primary"
              onClick={handleBatchTag}
              disabled={selectedCount === 0}
            >
              🏷️ Batch Tag
            </button>
            <button
              className="toolbar-btn danger"
              onClick={toggleSelectionMode}
            >
              ✕ Cancel
            </button>
          </div>
        </div>
      )}

      {isBatchTaggingModalOpen && (
        <BatchTagModal
          isOpen={true}
          onClose={closeBatchTaggingModal}
          onSuccess={() => {
            closeBatchTaggingModal()
            // Reload media files
            window.location.reload()
          }}
        />
      )}
    </>
  )
}
