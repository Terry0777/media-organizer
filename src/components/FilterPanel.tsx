import { useState } from 'react'
import { useTimelineStore } from '../stores/timelineStore'
import type { SearchFilters } from '../types'
import './FilterPanel.css'

interface FilterPanelProps {
  isOpen: boolean
  onClose: () => void
}

export function FilterPanel({ isOpen, onClose }: FilterPanelProps) {
  const { tags, setFilters } = useTimelineStore()
  const [selectedTags, setSelectedTags] = useState<number[]>([])
  const [tagLogic, setTagLogic] = useState<'AND' | 'OR'>('OR')
  const [fileType, setFileType] = useState<'all' | 'image' | 'video'>('all')
  const [dateFrom, setDateFrom] = useState('')
  const [dateTo, setDateTo] = useState('')

  const handleApplyFilters = () => {
    const filters: SearchFilters = {}
    
    if (selectedTags.length > 0) {
      filters.tag_ids = selectedTags
      filters.tag_logic = tagLogic
    }
    
    if (fileType !== 'all') {
      filters.file_type = fileType
    }
    
    if (dateFrom) {
      filters.date_from = new Date(dateFrom).getTime() / 1000
    }
    
    if (dateTo) {
      filters.date_to = new Date(dateTo).getTime() / 1000
      // Add one day to include the end date
      filters.date_to += 24 * 60 * 60
    }
    
    setFilters(filters)
    onClose()
  }

  const handleClearFilters = () => {
    setSelectedTags([])
    setTagLogic('OR')
    setFileType('all')
    setDateFrom('')
    setDateTo('')
    setFilters({})
    onClose()
  }

  const handleToggleTag = (tagId: number) => {
    setSelectedTags(prev =>
      prev.includes(tagId)
        ? prev.filter(id => id !== tagId)
        : [...prev, tagId]
    )
  }

  if (!isOpen) return null

  return (
    <div className="filter-panel-overlay" onClick={onClose}>
      <div className="filter-panel" onClick={(e) => e.stopPropagation()}>
        <header className="filter-header">
          <h3>Filter & Search</h3>
          <button className="filter-close" onClick={onClose}>✕</button>
        </header>

        <div className="filter-body">
          {/* Tag Logic */}
          <div className="filter-section">
            <label className="section-label">Tag Logic</label>
            <div className="logic-switcher">
              <button
                className={`logic-btn ${tagLogic === 'OR' ? 'active' : ''}`}
                onClick={() => setTagLogic('OR')}
              >
                OR (Any tag)
              </button>
              <button
                className={`logic-btn ${tagLogic === 'AND' ? 'active' : ''}`}
                onClick={() => setTagLogic('AND')}
              >
                AND (All tags)
              </button>
            </div>
          </div>

          {/* Tag Selection */}
          <div className="filter-section">
            <label className="section-label">Select Tags</label>
            <div className="tag-list">
              {tags.map((tag) => (
                <button
                  key={tag.id}
                  className={`tag-chip ${selectedTags.includes(tag.id) ? 'selected' : ''}`}
                  onClick={() => handleToggleTag(tag.id)}
                >
                  {tag.color && (
                    <span className="tag-dot" style={{ backgroundColor: tag.color }} />
                  )}
                  {tag.name}
                  {selectedTags.includes(tag.id) && <span className="check-mark">✓</span>}
                </button>
              ))}
            </div>
          </div>

          {/* File Type */}
          <div className="filter-section">
            <label className="section-label">File Type</label>
            <div className="type-selector">
              <button
                className={`type-btn ${fileType === 'all' ? 'active' : ''}`}
                onClick={() => setFileType('all')}
              >
                All
              </button>
              <button
                className={`type-btn ${fileType === 'image' ? 'active' : ''}`}
                onClick={() => setFileType('image')}
              >
                📷 Images
              </button>
              <button
                className={`type-btn ${fileType === 'video' ? 'active' : ''}`}
                onClick={() => setFileType('video')}
              >
                🎥 Videos
              </button>
            </div>
          </div>

          {/* Date Range */}
          <div className="filter-section">
            <label className="section-label">Date Range</label>
            <div className="date-inputs">
              <input
                type="date"
                value={dateFrom}
                onChange={(e) => setDateFrom(e.target.value)}
                placeholder="From"
              />
              <span className="date-separator">to</span>
              <input
                type="date"
                value={dateTo}
                onChange={(e) => setDateTo(e.target.value)}
                placeholder="To"
              />
            </div>
          </div>
        </div>

        <footer className="filter-footer">
          <button className="btn-clear" onClick={handleClearFilters}>
            Clear All
          </button>
          <button className="btn-apply" onClick={handleApplyFilters}>
            Apply Filters
          </button>
        </footer>
      </div>
    </div>
  )
}
