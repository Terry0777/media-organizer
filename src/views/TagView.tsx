import { useEffect, useState } from 'react'
import { TagTree } from '../components/TagTree'
import { getTagsAsTree, createTag } from '../api/tags'
import type { TagTreeNode } from '../types'
import './TagView.css'

export function TagView() {
  const [tags, setTags] = useState<TagTreeNode[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedTagId, setSelectedTagId] = useState<number | undefined>()
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [newTagName, setNewTagName] = useState('')
  const [newTagColor, setNewTagColor] = useState('#007aff')

  useEffect(() => {
    loadTags()
  }, [])

  const loadTags = async () => {
    try {
      setIsLoading(true)
      const tree = await getTagsAsTree()
      setTags(tree)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load tags')
    } finally {
      setIsLoading(false)
    }
  }

  const handleCreateTag = async () => {
    if (!newTagName.trim()) return
    
    try {
      await createTag(newTagName.trim(), undefined, newTagColor)
      setNewTagName('')
      setNewTagColor('#007aff')
      setShowCreateModal(false)
      await loadTags()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create tag')
    }
  }

  const handleRightClick = (tag: TagTreeNode) => {
    // TODO: Show context menu for tag operations
    console.log('Right click on tag:', tag)
  }

  return (
    <div className="tag-view">
      <header className="tag-view-header">
        <h2>🏷️ Tags</h2>
        <button className="primary-btn" onClick={() => setShowCreateModal(true)}>
          + New Tag
        </button>
      </header>

      <main className="tag-view-content">
        {error && (
          <div className="error-banner">
            <span>{error}</span>
            <button onClick={() => setError(null)}>✕</button>
          </div>
        )}

        {isLoading ? (
          <div className="loading-spinner">
            <div className="spinner" />
            <p>Loading tags...</p>
          </div>
        ) : (
          <TagTree
            tags={tags}
            selectedTagId={selectedTagId}
            onSelectTag={setSelectedTagId}
            onRightClick={handleRightClick}
          />
        )}
      </main>

      {showCreateModal && (
        <div className="modal-overlay" onClick={() => setShowCreateModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h3>Create New Tag</h3>
            <div className="form-group">
              <label>Tag Name</label>
              <input
                type="text"
                value={newTagName}
                onChange={(e) => setNewTagName(e.target.value)}
                placeholder="Enter tag name..."
                autoFocus
              />
            </div>
            <div className="form-group">
              <label>Color</label>
              <div className="color-picker">
                {['#007aff', '#34c759', '#ff9500', '#ff3b30', '#af52de', '#ff2d55'].map((color) => (
                  <button
                    key={color}
                    className={`color-option ${newTagColor === color ? 'selected' : ''}`}
                    style={{ backgroundColor: color }}
                    onClick={() => setNewTagColor(color)}
                  />
                ))}
              </div>
            </div>
            <div className="modal-actions">
              <button className="secondary-btn" onClick={() => setShowCreateModal(false)}>
                Cancel
              </button>
              <button className="primary-btn" onClick={handleCreateTag}>
                Create
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
