import type { TagTreeNode } from '../types'
import { TagTreeNodeItem } from './TagTreeNodeItem'
import './TagTree.css'

interface TagTreeProps {
  tags: TagTreeNode[]
  selectedTagId?: number
  onSelectTag?: (tagId: number) => void
  onRightClick?: (tag: TagTreeNode) => void
}

export function TagTree({ tags, selectedTagId, onSelectTag, onRightClick }: TagTreeProps) {
  return (
    <div className="tag-tree">
      {tags.length === 0 ? (
        <div className="empty-tags">
          <p>No tags yet</p>
          <button className="create-tag-btn">+ Create Tag</button>
        </div>
      ) : (
        <ul className="tag-tree-list">
          {tags.map((tag) => (
            <TagTreeNodeItem
              key={tag.id}
              node={tag}
              selectedTagId={selectedTagId}
              onSelectTag={onSelectTag}
              onRightClick={onRightClick}
              depth={0}
            />
          ))}
        </ul>
      )}
    </div>
  )
}
