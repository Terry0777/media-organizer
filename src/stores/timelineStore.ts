import { create } from 'zustand'
import type { MediaFile, Tag, Album, SearchFilters, DatabaseStats } from '../types'
import * as api from '../api'

interface TimelineState {
  // Data
  mediaFiles: MediaFile[]
  tags: Tag[]
  albums: Album[]
  stats: DatabaseStats | null
  
  // UI State
  isLoading: boolean
  error: string | null
  currentFilters: SearchFilters
  currentPage: number
  totalPages: number
  
  // Batch Selection State
  selectedMediaIds: number[]
  isSelectionMode: boolean
  isBatchTaggingModalOpen: boolean
  
  // Actions
  loadMediaFiles: (filters?: SearchFilters, page?: number) => Promise<void>
  loadTags: () => Promise<void>
  loadAlbums: () => Promise<void>
  loadStats: () => Promise<void>
  scanDirectory: (path: string) => Promise<void>
  setFilters: (filters: SearchFilters) => void
  setPage: (page: number) => void
  clearError: () => void
  
  // Batch Actions
  toggleSelectionMode: () => void
  selectMedia: (mediaId: number) => void
  deselectMedia: (mediaId: number) => void
  selectAllMedia: () => void
  deselectAllMedia: () => void
  openBatchTaggingModal: () => void
  closeBatchTaggingModal: () => void
  batchAddTags: (tagIds: number[]) => Promise<void>
  batchRemoveTags: (tagIds: number[]) => Promise<void>
}

import * as api from './api'

export const useTimelineStore = create<TimelineState>((set, get) => ({
  // Initial state
  mediaFiles: [],
  tags: [],
  albums: [],
  stats: null,
  isLoading: false,
  error: null,
  currentFilters: {},
  currentPage: 1,
  totalPages: 0,
  
  // Batch selection state
  selectedMediaIds: [],
  isSelectionMode: false,
  isBatchTaggingModalOpen: false,
  
  // Load media files with filters and pagination
  loadMediaFiles: async (filters, page = 1) => {
    set({ isLoading: true, error: null, currentPage: page })
    
    try {
      const currentFilters = filters || get().currentFilters
      const result = await api.getMediaFiles(currentFilters, { page, page_size: 50 })
      
      set({
        mediaFiles: result.items,
        totalPages: result.total_pages,
        currentFilters: currentFilters,
        isLoading: false,
      })
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : 'Failed to load media files',
        isLoading: false,
      })
    }
  },
  
  // Load all tags
  loadTags: async () => {
    try {
      const tags = await api.getTags()
      set({ tags })
    } catch (err) {
      console.error('Failed to load tags:', err)
    }
  },
  
  // Load all albums
  loadAlbums: async () => {
    try {
      const albums = await api.getAlbums()
      set({ albums })
    } catch (err) {
      console.error('Failed to load albums:', err)
    }
  },
  
  // Load database statistics
  loadStats: async () => {
    try {
      const stats = await api.getDatabaseStats()
      set({ stats })
    } catch (err) {
      console.error('Failed to load stats:', err)
    }
  },
  
  // Scan directory for new media files
  scanDirectory: async (path: string) => {
    set({ isLoading: true, error: null })
    
    try {
      const result = await api.scanDirectory(path)
      console.log('Scan completed:', result)
      
      // Reload media files after scan
      await get().loadMediaFiles()
      await get().loadStats()
      
      set({ isLoading: false })
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : 'Failed to scan directory',
        isLoading: false,
      })
    }
  },
  
  // Set search filters
  setFilters: (filters) => {
    set({ currentFilters: filters })
    get().loadMediaFiles(filters, 1)
  },
  
  // Set current page
  setPage: (page) => {
    get().loadMediaFiles(undefined, page)
  },
  
  // Clear error
  clearError: () => {
    set({ error: null })
  },
  
  // Toggle selection mode
  toggleSelectionMode: () => {
    const isSelectionMode = !get().isSelectionMode
    set({
      isSelectionMode,
      selectedMediaIds: isSelectionMode ? [] : get().selectedMediaIds,
    })
  },
  
  // Select a media file
  selectMedia: (mediaId: number) => {
    const selectedMediaIds = get().selectedMediaIds
    if (!selectedMediaIds.includes(mediaId)) {
      set({ selectedMediaIds: [...selectedMediaIds, mediaId] })
    }
  },
  
  // Deselect a media file
  deselectMedia: (mediaId: number) => {
    const selectedMediaIds = get().selectedMediaIds.filter(id => id !== mediaId)
    set({ selectedMediaIds })
  },
  
  // Select all visible media files
  selectAllMedia: () => {
    const mediaFiles = get().mediaFiles
    set({ selectedMediaIds: mediaFiles.map(m => m.id!).filter(id => id !== null) as number[] })
  },
  
  // Deselect all media files
  deselectAllMedia: () => {
    set({ selectedMediaIds: [] })
  },
  
  // Open batch tagging modal
  openBatchTaggingModal: () => {
    set({ isBatchTaggingModalOpen: true })
  },
  
  // Close batch tagging modal
  closeBatchTaggingModal: () => {
    set({ isBatchTaggingModalOpen: false })
  },
  
  // Batch add tags to selected media
  batchAddTags: async (tagIds: number[]) => {
    const selectedMediaIds = get().selectedMediaIds
    
    try {
      // Add each tag to each selected media file
      for (const mediaId of selectedMediaIds) {
        for (const tagId of tagIds) {
          await api.addTagToMedia(mediaId, tagId)
        }
      }
      
      // Reload media files to reflect changes
      await get().loadMediaFiles()
      set({ isBatchTaggingModalOpen: false, selectedMediaIds: [] })
    } catch (err) {
      console.error('Failed to batch add tags:', err)
      throw err
    }
  },
  
  // Batch remove tags from selected media
  batchRemoveTags: async (tagIds: number[]) => {
    const selectedMediaIds = get().selectedMediaIds
    
    try {
      // Remove each tag from each selected media file
      for (const mediaId of selectedMediaIds) {
        for (const tagId of tagIds) {
          await api.removeTagFromMedia(mediaId, tagId)
        }
      }
      
      // Reload media files to reflect changes
      await get().loadMediaFiles()
      set({ isBatchTaggingModalOpen: false, selectedMediaIds: [] })
    } catch (err) {
      console.error('Failed to batch remove tags:', err)
      throw err
    }
  },
}))
