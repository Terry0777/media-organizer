import { useEffect } from 'react'
import type { TagTreeNode } from '../types'
import './TagContextMenu.css'

interface TagContextMenuProps {
  x: number
  y: number
  tag: TagTreeNode
  onClose: () => void
  onCreateChild: () => void
  onDelete: () => void
  onRename: () => void
  onMerge: () => void
}

export function TagContextMenu({
  x,
  y,
  tag,
  onClose,
  onCreateChild,
  onDelete,
  onRename,
  onMerge,
}: TagContextMenuProps) {
  useEffect(() => {
    const handleClick = () => onClose()
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    
    document.addEventListener('click', handleClick)
    document.addEventListener('keydown', handleKeyDown)
    
    return () => {
      document.removeEventListener('click', handleClick)
      document.removeEventListener('keydown', handleKeyDown)
    }
  }, [onClose])

  const style = {
    position: 'fixed' as const,
    left: Math.min(x, window.innerWidth - 200),
    top: Math.min(y, window.innerHeight - 200),
  }

  return (
    <div className="tag-context-menu" style={style}>
      <div className="context-menu-header">
        <span className="tag-name-preview">{tag.name}</span>
        {tag.color && (
          <span className="tag-color-preview" style={{ backgroundColor: tag.color }} />
        )}
      </div>
      
      <div className="context-menu-divider" />
      
      <div className="context-menu-item" onClick={onCreateChild}>
        ➕ Add Child Tag
      </div>
      
      <div className="context-menu-item" onClick={onRename}>
        ✏️ Rename
      </div>
      
      {tag.parent_id !== null && (
        <div className="context-menu-item" onClick={onMerge}>
          🔀 Merge with Parent
        </div>
      )}
      
      <div className="context-menu-divider" />
      
      <div className="context-menu-item danger" onClick={onDelete}>
        🗑️ Delete
        {tag.children.length > 0 && (
          <span className="delete-warning">(includes {tag.children.length} children)</span>
        )}
      </div>
    </div>
  )
}
