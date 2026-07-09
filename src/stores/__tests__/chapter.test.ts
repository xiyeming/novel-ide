import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useChapterStore } from '../chapter'
import { useTauriIPC } from '../../composables/useTauriIPC'

const mockCall = vi.fn()

vi.mock('../../composables/useTauriIPC', () => ({
  useTauriIPC: () => ({
    call: mockCall,
  }),
}))

describe('Chapter Store', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    setActivePinia(createPinia())
  })

  it('initializes with empty chapters', () => {
    const store = useChapterStore()
    expect(store.chapters).toEqual([])
    expect(store.currentChapter).toBeNull()
    expect(store.loading).toBe(false)
  })

  it('sets current chapter', () => {
    const store = useChapterStore()
    const mockChapter = {
      id: '1',
      title: '第一章',
      content: '测试内容',
      project_id: 'proj-1',
      sort_order: 0,
      word_count: 4,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }

    store.currentChapter = mockChapter
    expect(store.currentChapter).toEqual(mockChapter)
  })

  it('fetches chapters', async () => {
    const chapters = [
      {
        id: '1',
        title: '第一章',
        content: '内容1',
        project_id: 'proj-1',
        sort_order: 0,
        word_count: 3,
        created_at: '2024-01-01',
        updated_at: '2024-01-01',
      },
      {
        id: '2',
        title: '第二章',
        content: '内容2',
        project_id: 'proj-1',
        sort_order: 1,
        word_count: 3,
        created_at: '2024-01-02',
        updated_at: '2024-01-02',
      },
    ]

    mockCall.mockResolvedValue(chapters)
    const store = useChapterStore()
    await store.fetchChapters('proj-1')

    expect(store.chapters).toEqual(chapters)
    expect(store.loading).toBe(false)
  })

  it('creates a chapter', async () => {
    const newChapter = {
      id: '3',
      title: '第三章',
      content: '',
      project_id: 'proj-1',
      sort_order: 2,
      word_count: 0,
      created_at: '2024-01-03',
      updated_at: '2024-01-03',
    }

    mockCall.mockResolvedValue(newChapter)
    const store = useChapterStore()
    const result = await store.createChapter('proj-1', '第三章', 2)

    expect(result).toEqual(newChapter)
    expect(store.chapters).toContainEqual(newChapter)
  })

  it('opens a chapter', async () => {
    const chapter = {
      id: '1',
      title: '第一章',
      content: '测试内容',
      project_id: 'proj-1',
      sort_order: 0,
      word_count: 4,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }

    mockCall.mockResolvedValue(chapter)
    const store = useChapterStore()
    await store.openChapter('1')

    expect(store.currentChapter).toEqual(chapter)
  })

  it('updates chapter content', async () => {
    const chapter = {
      id: '1',
      title: '第一章',
      content: '旧内容',
      project_id: 'proj-1',
      sort_order: 0,
      word_count: 3,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }

    mockCall.mockResolvedValueOnce([chapter])
    const store = useChapterStore()
    await store.fetchChapters('proj-1')
    mockCall.mockResolvedValueOnce(chapter)
    await store.openChapter('1')

    const updated = { ...chapter, content: '新内容', word_count: 3 }
    mockCall.mockResolvedValueOnce(updated)
    await store.updateChapterContent('1', '新内容')

    expect(store.chapters[0].content).toBe('新内容')
    expect(store.currentChapter?.content).toBe('新内容')
  })

  it('updates chapter title', async () => {
    const chapter = {
      id: '1',
      title: '旧标题',
      content: '内容',
      project_id: 'proj-1',
      sort_order: 0,
      word_count: 2,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }

    mockCall.mockResolvedValueOnce([chapter])
    const store = useChapterStore()
    await store.fetchChapters('proj-1')
    mockCall.mockResolvedValueOnce(chapter)
    await store.openChapter('1')

    const updated = { ...chapter, title: '新标题' }
    mockCall.mockResolvedValueOnce(updated)
    await store.updateChapterTitle('1', '新标题')

    expect(store.chapters[0].title).toBe('新标题')
    expect(store.currentChapter?.title).toBe('新标题')
  })

  it('deletes a chapter', async () => {
    const chapters = [
      {
        id: '1',
        title: '第一章',
        content: '内容1',
        project_id: 'proj-1',
        sort_order: 0,
        word_count: 3,
        created_at: '2024-01-01',
        updated_at: '2024-01-01',
      },
      {
        id: '2',
        title: '第二章',
        content: '内容2',
        project_id: 'proj-1',
        sort_order: 1,
        word_count: 3,
        created_at: '2024-01-02',
        updated_at: '2024-01-02',
      },
    ]

    mockCall.mockResolvedValue(chapters)
    const store = useChapterStore()
    await store.fetchChapters('proj-1')
    store.currentChapter = store.chapters[0]

    mockCall.mockResolvedValue(undefined)
    await store.deleteChapter('1')

    expect(store.chapters).toHaveLength(1)
    expect(store.chapters[0].id).toBe('2')
    expect(store.currentChapter?.id).toBe('2')
  })

  it('sets loading state during fetch', async () => {
    let resolveCall: (value: unknown) => void
    const pendingPromise = new Promise((resolve) => {
      resolveCall = resolve
    })
    mockCall.mockReturnValueOnce(pendingPromise)

    const store = useChapterStore()
    const fetchPromise = store.fetchChapters('proj-1')

    expect(store.loading).toBe(true)

    resolveCall!([])
    await fetchPromise

    expect(store.loading).toBe(false)
  })
})
