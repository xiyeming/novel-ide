<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useShortcutStore } from '../../stores/shortcuts'

const store = useShortcutStore()
const editingId = ref<string | null>(null)
const newKeyBinding = ref('')
const hyprlandConfig = ref('')
const showHyprlandDialog = ref(false)

onMounted(() => {
  store.fetchShortcuts()
})

function startEdit(id: string, currentBinding: string) {
  editingId.value = id
  newKeyBinding.value = currentBinding
}

function captureKey(event: KeyboardEvent) {
  event.preventDefault()
  const parts: string[] = []
  if (event.ctrlKey) parts.push('Ctrl')
  if (event.altKey) parts.push('Alt')
  if (event.shiftKey) parts.push('Shift')
  if (event.metaKey) parts.push('Cmd')
  
  const key = event.key.length === 1 ? event.key.toUpperCase() : event.key
  if (!['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
    parts.push(key)
    newKeyBinding.value = parts.join('+')
  }
}

async function saveEdit() {
  if (!editingId.value || !newKeyBinding.value) return
  await store.updateShortcut(editingId.value, newKeyBinding.value)
  editingId.value = null
  newKeyBinding.value = ''
}

async function generateHyprland() {
  hyprlandConfig.value = await store.generateHyprlandConfig()
  showHyprlandDialog.value = true
}

function copyToClipboard() {
  navigator.clipboard.writeText(hyprlandConfig.value)
}
</script>

<template>
  <div class="shortcut-settings">
    <div class="settings-header">
      <h3>⌨️ 快捷键设置</h3>
      <button class="btn-sm" @click="generateHyprland">导出 Hyprland 配置</button>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>
    
    <div v-else class="shortcut-list">
      <div v-for="shortcut in store.shortcuts" :key="shortcut.id" class="shortcut-item">
        <div class="shortcut-info">
          <div class="shortcut-action">{{ shortcut.action }}</div>
          <div class="shortcut-binding" v-if="editingId !== shortcut.id">
            {{ shortcut.keyBinding }}
          </div>
          <input 
            v-else
            v-model="newKeyBinding"
            @keydown="captureKey"
            class="key-input"
            readonly
            placeholder="按下快捷键..."
          />
        </div>
        <div class="shortcut-actions">
          <button 
            v-if="editingId === shortcut.id"
            class="btn-sm primary"
            @click="saveEdit"
          >
            保存
          </button>
          <button 
            v-else
            class="btn-sm"
            @click="startEdit(shortcut.id, shortcut.keyBinding)"
          >
            编辑
          </button>
          <label class="toggle">
            <input 
              type="checkbox" 
              :checked="shortcut.isEnabled"
              @change="store.toggleShortcut(shortcut.id, ($event.target as HTMLInputElement).checked)"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <div v-if="showHyprlandDialog" class="dialog-overlay" @click="showHyprlandDialog = false">
      <div class="dialog" @click.stop>
        <h3>Hyprland 快捷键配置</h3>
        <p class="dialog-hint">将以下内容添加到 ~/.config/hypr/hyprland.conf</p>
        <textarea :value="hyprlandConfig" readonly class="config-textarea" rows="15" />
        <div class="dialog-actions">
          <button class="btn-sm" @click="copyToClipboard">复制到剪贴板</button>
          <button class="btn-sm" @click="showHyprlandDialog = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.shortcut-settings {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.settings-header h3 {
  margin: 0;
  font-size: 16px;
}

.btn-sm {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.btn-sm:hover {
  background: var(--bg-tertiary);
}

.btn-sm.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border);
}

.shortcut-info {
  flex: 1;
}

.shortcut-action {
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.shortcut-binding {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: monospace;
}

.key-input {
  width: 200px;
  padding: 6px 8px;
  border: 2px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: monospace;
  font-size: 12px;
}

.shortcut-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--bg-tertiary);
  transition: 0.3s;
  border-radius: 22px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: var(--text-secondary);
  transition: 0.3s;
  border-radius: 50%;
}

.toggle input:checked + .toggle-slider {
  background-color: var(--accent);
}

.toggle input:checked + .toggle-slider:before {
  transform: translateX(18px);
  background-color: white;
}

.loading {
  text-align: center;
  color: var(--text-secondary);
  padding: 20px;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 24px;
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.dialog h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
}

.dialog-hint {
  color: var(--text-secondary);
  font-size: 13px;
  margin-bottom: 16px;
}

.config-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: monospace;
  font-size: 12px;
  resize: vertical;
  margin-bottom: 16px;
  box-sizing: border-box;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>
