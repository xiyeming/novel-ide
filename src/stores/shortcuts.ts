import { ref } from 'vue'
import { useTauriIPC } from '../composables/useTauriIPC'

export interface Shortcut {
  id: string
  action: string
  keyBinding: string
  platform: string
  isEnabled: boolean
}

export function useShortcutStore() {
  const shortcuts = ref<Shortcut[]>([])
  const loading = ref(false)
  const { call } = useTauriIPC()

  async function fetchShortcuts() {
    loading.value = true
    try {
      shortcuts.value = await call<Shortcut[]>('list_shortcuts')
    } finally {
      loading.value = false
    }
  }

  async function updateShortcut(id: string, keyBinding: string) {
    const result = await call<Shortcut>('update_shortcut', { id, keyBinding })
    const idx = shortcuts.value.findIndex(s => s.id === id)
    if (idx >= 0) shortcuts.value[idx] = result
    return result
  }

  async function toggleShortcut(id: string, enabled: boolean) {
    await call('toggle_shortcut', { id, enabled })
    const idx = shortcuts.value.findIndex(s => s.id === id)
    if (idx >= 0) shortcuts.value[idx].isEnabled = enabled
  }

  async function generateHyprlandConfig() {
    return await call<string>('generate_hyprland_config')
  }

  function getShortcutForAction(action: string) {
    const shortcut = shortcuts.value.find(s => s.action === action && s.isEnabled)
    return shortcut?.keyBinding || null
  }

  return {
    shortcuts,
    loading,
    fetchShortcuts,
    updateShortcut,
    toggleShortcut,
    generateHyprlandConfig,
    getShortcutForAction,
  }
}
