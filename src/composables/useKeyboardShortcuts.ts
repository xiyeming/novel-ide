import { onMounted, onUnmounted } from 'vue'
import { useShortcutStore } from '../stores/shortcuts'

type ShortcutHandler = () => void

export function useKeyboardShortcuts(handlers: Record<string, ShortcutHandler>) {
  const store = useShortcutStore()

  function parseKeyBinding(binding: string): { key: string; ctrl: boolean; alt: boolean; shift: boolean; meta: boolean } {
    const parts = binding.split('+')
    const key = parts[parts.length - 1]
    const ctrl = parts.includes('Ctrl') || parts.includes('CmdOrCtrl')
    const alt = parts.includes('Alt')
    const shift = parts.includes('Shift')
    const meta = parts.includes('Cmd') || parts.includes('CmdOrCtrl')

    return { key, ctrl, alt, shift, meta }
  }

  function handleKeyDown(event: KeyboardEvent) {
    for (const [action, handler] of Object.entries(handlers)) {
      const binding = store.getShortcutForAction(action)
      if (!binding) continue

      const parsed = parseKeyBinding(binding)

      const keyMatch = event.key.toLowerCase() === parsed.key.toLowerCase() ||
                       event.code.toLowerCase() === `key${parsed.key.toLowerCase()}`
      const ctrlMatch = event.ctrlKey === parsed.ctrl
      const altMatch = event.altKey === parsed.alt
      const shiftMatch = event.shiftKey === parsed.shift
      const metaMatch = event.metaKey === parsed.meta

      if (keyMatch && ctrlMatch && altMatch && shiftMatch && metaMatch) {
        event.preventDefault()
        handler()
        return
      }
    }
  }

  onMounted(() => {
    document.addEventListener('keydown', handleKeyDown)
  })

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeyDown)
  })
}
