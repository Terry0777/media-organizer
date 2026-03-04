import { useState } from 'react'
import type { TagTreeNode } from '../types'
import './TagTreeNodeItem.css'

interface TagTreeNodeItemProps {
  node: TagTreeNode
  selectedTagId?: number
  onSelectTag?: (tagId: number) => void
  onRightClick?: (tag: TagTreeNode) => void
  depth: number
}

export function TagTreeNodeItem({
  node,
  selectedTagId,
  onSelectTag,
  onRightClick,
  depth,
}: TagTreeNodeItemProps) {
  const [isExpanded, setIsExpanded] = useState(true)
  const hasChildren = node.children.length > 0

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (hasChildren) {
      setIsExpanded(!isExpanded)
    }
    if (onSelectTag) {
      onSelectTag(node.id)
    }
  }

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    if (onRightClick) {
      onRightClick(node)
    }
  }

  return (
    <li className="tag-tree-node" data-depth={depth}>
      <div
        className={`tag-tree-item ${selectedTagId === node.id ? 'selected' : ''}`}
        style={{ paddingLeft: `${depth * 16 + 8}px` }}
        onClick={handleClick}
        onContextMenu={handleContextMenu}
      >
        {hasChildren && (
          <span className={`expand-icon ${isExpanded ? 'expanded' : ''}`}>
            ▶
          </span>
        )}
        {!hasChildren && <span className="expand-icon-placeholder" />}
        
        {node.color && (
          <span className="tag-color-dot" style={{ backgroundColor: node.color }} />
        )}
        
        <span className="tag-name">{node.name}</span>
        
        {node.media_count !== undefined && node.media_count > 0 && (
          <span className="tag-count">{node.media_count}</span>
        )}
      </div>
      
      {hasChildren && isExpanded && (
        <ul className="tag-tree-children">
          {node.children.map((child) => (
            <TagTreeNodeItem
              key={child.id}
              node={child}
              selectedTagId={selectedTagId}
              onSelectTag={onSelectTag}
              onRightClick={onRightClick}
              depth={depth + 1}
            />
          ))}
        </ul>
      )}
    </li>
  )
}
