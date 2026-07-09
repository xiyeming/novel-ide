import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import WorkspaceSwitcher from '../WorkspaceSwitcher.vue'
import { useWorkspaceStore } from '../../../stores/workspace'

vi.mock('../../../stores/workspace', () => ({
  useWorkspaceStore: vi.fn(),
}))

describe('WorkspaceSwitcher', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('should render preset buttons', () => {
    const mockStore = {
      presets: [
        { id: 'writing', name: '写作', icon: '✍️' },
        { id: 'review', name: '审稿', icon: '📝' },
      ],
      activePresetId: 'writing',
      setActivePreset: vi.fn(),
    }

    vi.mocked(useWorkspaceStore).mockReturnValue(mockStore as any)

    const wrapper = mount(WorkspaceSwitcher)
    
    expect(wrapper.findAll('.preset-btn')).toHaveLength(2)
    expect(wrapper.text()).toContain('写作')
    expect(wrapper.text()).toContain('审稿')
  })

  it('should highlight active preset', () => {
    const mockStore = {
      presets: [
        { id: 'writing', name: '写作', icon: '✍️' },
        { id: 'review', name: '审稿', icon: '📝' },
      ],
      activePresetId: 'writing',
      setActivePreset: vi.fn(),
    }

    vi.mocked(useWorkspaceStore).mockReturnValue(mockStore as any)

    const wrapper = mount(WorkspaceSwitcher)
    
    const buttons = wrapper.findAll('.preset-btn')
    expect(buttons[0].classes()).toContain('active')
    expect(buttons[1].classes()).not.toContain('active')
  })

  it('should call setActivePreset when clicking a button', async () => {
    const mockStore = {
      presets: [
        { id: 'writing', name: '写作', icon: '✍️' },
        { id: 'review', name: '审稿', icon: '📝' },
      ],
      activePresetId: 'writing',
      setActivePreset: vi.fn(),
    }

    vi.mocked(useWorkspaceStore).mockReturnValue(mockStore as any)

    const wrapper = mount(WorkspaceSwitcher)
    
    await wrapper.findAll('.preset-btn')[1].trigger('click')
    
    expect(mockStore.setActivePreset).toHaveBeenCalledWith('review')
  })
})