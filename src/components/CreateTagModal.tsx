import { useState } from 'react'
import * as tagsApi from '../api/tags'
import './CreateTagModal.css'

interface CreateTagModalProps {
  parentId?: number
  onClose: () => void
  onSuccess: () => void
}

const PRESET_COLORS = [
  '#ff6b6b', '#feca57', '#48dbfb', '#ff9ff3',
  '#54a0ff', '#5f27cd', '#00d2d3', '#1dd1a1',
  '#c8d6e5', '#8395a7', '#576574', '#222f3e',
]

export function CreateTagModal({ parentId, onClose, onSuccess }: CreateTagModalProps) {
  const [name, setName] = useState('')
  const [color, setColor] = useState<string | undefined>()
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!name.trim()) {
      setError('Tag name is required')
      return
    }

    setIsSubmitting(true)
    setError('')

    try {
      await tagsApi.createTag(name.trim(), parentId, color)
      onSuccess()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create tag')
      setIsSubmitting(false)
    }
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <header className="modal-header">
          <h3>Create New Tag</h3>
          <button className="modal-close" onClick={onClose}>✕</button>
        </header>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="tag-name">Tag Name</label>
            <input
              id="tag-name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter tag name"
              autoFocus
              disabled={isSubmitting}
            />
          </div>

          <div className="form-group">
            <label>Color (Optional)</label>
            <div className="color-picker">
              <button
                type="button"
                className={`color-option ${!color ? 'selected' : ''}`}
                onClick={() => setColor(undefined)}
                title="No color"
              >
                ∅
              </button>
              {PRESET_COLORS.map((c) => (
                <button
                  key={c}
                  type="button"
                  className={`color-option ${color === c ? 'selected' : ''}`}
                  style={{ backgroundColor: c }}
                  onClick={() => setColor(c)}
                  title={c}
                />
              ))}
            </div>
          </div>

          {error && <div className="form-error">{error}</div>}

          <footer className="modal-footer">
            <button type="button" className="btn-secondary" onClick={onClose}>
              Cancel
            </button>
            <button type="submit" className="btn-primary" disabled={isSubmitting}>
              {isSubmitting ? 'Creating...' : 'Create Tag'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  )
}
