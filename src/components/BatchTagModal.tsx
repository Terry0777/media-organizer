import { useState } from 'react'
import { useTimelineStore } from '../stores/timelineStore'
import { TagTree } from './TagTree'
import * as tagsApi from '../api/tags'
import type { TagTreeNode } from '../types'
import './BatchTagModal.css'

interface BatchTagModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
}

export function BatchTagModal({ isOpen, onClose, onSuccess }: BatchTagModalProps) {
  const { selectedMediaIds, batchAddTags, batchRemoveTags } = useTimelineStore()
  const [tagTree, setTagTree] = useState<TagTreeNode[]>([])
  const [selectedTagIds, setSelectedTagIds] = useState<number[]>([])
  const [mode, setMode] = useState<'add' | 'remove'>('add')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState('')

  // Load tags when modal opens
  useState(() => {
    if (isOpen) {
      tagsApi.getTagsAsTree().then(setTagTree)
    }
  })

  const handleSubmit = async () => {
    if (selectedTagIds.length === 0) {
      setError('Please select at least one tag')
      return
    }

    setIsSubmitting(true)
    setError('')

    try {
      if (mode === 'add') {
        await batchAddTags(selectedTagIds)
      } else {
        await batchRemoveTags(selectedTagIds)
      }
      onSuccess()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Operation failed')
      setIsSubmitting(false)
    }
  }

  const handleToggleTag = (tagId: number) => {
    setSelectedTagIds(prev =>
      prev.includes(tagId)
        ? prev.filter(id => id !== tagId)
        : [...prev, tagId]
    )
  }

  if (!isOpen) return null

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="batch-tag-modal" onClick={(e) => e.stopPropagation()}>
        <header className="modal-header">
          <h3>Batch Tag {mode === 'add' ? 'Add' : 'Remove'}</h3>
          <button className="modal-close" onClick={onClose}>✕</button>
        </header>

        <div className="modal-body">
          <div className="selection-info">
            📁 Selected: <strong>{selectedMediaIds.length}</strong> files
          </div>

          <div className="mode-switcher">
            <button
              className={`mode-btn ${mode === 'add' ? 'active' : ''}`}
              onClick={() => setMode('add')}
            >
              ➕ Add Tags
            </button>
            <button
              className={`mode-btn ${mode === 'remove' ? 'active' : ''}`}
              onClick={() => setMode('remove')}
            >
              ➖ Remove Tags
            </button>
          </div>

          <div className="tag-selection">
            <h4>Select Tags:</h4>
            <TagTree
              tags={tagTree}
              selectedTagId={undefined}
              onSelectTag={handleToggleTag}
              multipleSelect
              selectedTagIds={selectedTagIds}
            />
          </div>

          {selectedTagIds.length > 0 && (
            <div className="selected-tags-summary">
              <strong>{selectedTagIds.length}</strong> tag(s) selected
            </div>
          )}

          {error && <div className="form-error">{error}</div>}
        </div>

        <footer className="modal-footer">
          <button className="btn-secondary" onClick={onClose} disabled={isSubmitting}>
            Cancel
          </button>
          <button
            className="btn-primary"
            onClick={handleSubmit}
            disabled={isSubmitting || selectedTagIds.length === 0}
          >
            {isSubmitting
              ? 'Processing...'
              : `${mode === 'add' ? 'Add' : 'Remove'} Tags to ${selectedMediaIds.length} Files`}
          </button>
        </footer>
      </div>
    </div>
  )
}
