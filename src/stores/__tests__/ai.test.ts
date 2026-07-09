import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAIStore } from '../ai'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}))

vi.mock('../../composables/useTauriIPC', () => ({
  useTauriIPC: () => ({
    call: vi.fn(),
  }),
}))

describe('useAIStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('should initialize with default values', () => {
    const store = useAIStore()
    expect(store.messages).toEqual([])
    expect(store.selectedModel).toBe('deepseek-chat')
    expect(store.streaming).toBe(false)
    expect(store.activeAIFeature).toBeNull()
  })

  it('should set and clear active feature', () => {
    const store = useAIStore()
    store.setActiveFeature('continue')
    expect(store.activeAIFeature).toBe('continue')
    store.clearActiveFeature()
    expect(store.activeAIFeature).toBeNull()
  })

  it('should add messages', () => {
    const store = useAIStore()
    store.addMessage('user', 'Hello')
    expect(store.messages).toHaveLength(1)
    expect(store.messages[0].role).toBe('user')
    expect(store.messages[0].content).toBe('Hello')
  })

  it('should clear messages', () => {
    const store = useAIStore()
    store.addMessage('user', 'Hello')
    store.addMessage('assistant', 'Hi there')
    store.clearMessages()
    expect(store.messages).toEqual([])
  })

  it('should send message and create assistant placeholder', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    const { listen } = await import('@tauri-apps/api/event')
    const mockListen = vi.fn().mockResolvedValue(vi.fn())
    vi.mocked(listen).mockImplementation(mockListen)
    vi.mocked(invoke).mockResolvedValue(undefined)

    const store = useAIStore()
    await store.sendMessage('Hello')

    expect(store.messages).toHaveLength(2)
    expect(store.messages[0].role).toBe('user')
    expect(store.messages[0].content).toBe('Hello')
    expect(store.messages[1].role).toBe('assistant')
    expect(store.messages[1].content).toBe('')
    expect(store.streaming).toBe(true)
  })

  it('should not send empty messages', async () => {
    const store = useAIStore()
    await store.sendMessage('   ')
    expect(store.messages).toHaveLength(0)
  })

  it('should not send messages while streaming', async () => {
    const store = useAIStore()
    store.streaming = true
    await store.sendMessage('Hello')
    expect(store.messages).toHaveLength(0)
  })

  it('should handle streaming errors', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    const { listen } = await import('@tauri-apps/api/event')
    vi.mocked(listen).mockResolvedValue(vi.fn())
    vi.mocked(invoke).mockRejectedValue(new Error('API Error'))

    const store = useAIStore()
    await store.sendMessage('Hello')

    expect(store.messages).toHaveLength(2)
    expect(store.messages[1].content).toContain('Error')
    expect(store.streaming).toBe(false)
  })
})