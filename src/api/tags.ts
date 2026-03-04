import { invoke } from '@tauri-apps/api/core'
import type { Tag, TagTreeNode } from '../types'

/**
 * Get all tags as a flat list
 */
export async function getTags(): Promise<Tag[]> {
  return invoke('get_tags')
}

/**
 * Get tags as a tree structure (with hierarchy)
 */
export async function getTagsAsTree(): Promise<TagTreeNode[]> {
  const tags = await getTags()
  return buildTagTree(tags)
}

/**
 * Create a new tag
 */
export async function createTag(
  name: string,
  parentId?: number,
  color?: string
): Promise<number> {
  return invoke('create_tag', { name, parent_id: parentId, color })
}

/**
 * Update a tag
 */
export async function updateTag(
  id: number,
  name?: string,
  parentId?: number | null,
  color?: string
): Promise<void> {
  return invoke('update_tag', { id, name, parent_id: parentId, color })
}

/**
 * Delete a tag
 */
export async function deleteTag(id: number, deleteChildren = false): Promise<void> {
  return invoke('delete_tag', { id, delete_children: deleteChildren })
}

/**
 * Add tag to media
 */
export async function addTagToMedia(mediaId: number, tagId: number): Promise<void> {
  return invoke('add_tag_to_media', { media_id: mediaId, tag_id: tagId })
}

/**
 * Remove tag from media
 */
export async function removeTagFromMedia(mediaId: number, tagId: number): Promise<number> {
  return invoke('remove_tag_from_media', { media_id: mediaId, tag_id: tagId })
}

/**
 * Get tags for a specific media file
 */
export async function getTagsForMedia(mediaId: number): Promise<Tag[]> {
  return invoke('get_tags_for_media', { media_id: mediaId })
}

/**
 * Get media files with a specific tag
 */
export async function getMediaByTag(tagId: number, page = 1, pageSize = 50) {
  return invoke('get_media_files', {
    filters: { tag_ids: [tagId] },
    pagination: { page, page_size: pageSize },
  })
}

/**
 * Get tag usage statistics
 */
export async function getTagUsage(): Promise<Array<Tag & { usage_count: number }>> {
  return invoke('get_tag_usage')
}

/**
 * Merge two tags (move all media from source to target)
 */
export async function mergeTags(sourceTagId: number, targetTagId: number): Promise<void> {
  return invoke('merge_tags', { source_tag_id: sourceTagId, target_tag_id: targetTagId })
}

/**
 * Get suggested tags (based on existing tags and media metadata)
 */
export async function getSuggestedTags(mediaPath: string): Promise<Tag[]> {
  return invoke('get_suggested_tags', { media_path: mediaPath })
}

// Helper function to build tag tree from flat list
function buildTagTree(tags: Tag[]): TagTreeNode[] {
  const tagMap = new Map<number, TagTreeNode>()
  const rootTags: TagTreeNode[] = []

  // Create tag nodes
  tags.forEach((tag) => {
    tagMap.set(tag.id, {
      ...tag,
      children: [],
      media_count: tag.media_count || 0,
    })
  })

  // Build hierarchy
  tags.forEach((tag) => {
    const node = tagMap.get(tag.id)!
    if (tag.parent_id == null) {
      rootTags.push(node)
    } else {
      const parent = tagMap.get(tag.parent_id)
      if (parent) {
        parent.children.push(node)
      } else {
        // Parent not found, treat as root
        rootTags.push(node)
      }
    }
  })

  // Sort by name
  rootTags.sort((a, b) => a.name.localeCompare(b.name))
  
  // Sort children recursively
  const sortChildren = (node: TagTreeNode) => {
    node.children.sort((a, b) => a.name.localeCompare(b.name))
    node.children.forEach(sortChildren)
  }
  rootTags.forEach(sortChildren)

  return rootTags
}
