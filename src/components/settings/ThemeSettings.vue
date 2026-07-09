<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useThemeStore } from '../../stores/theme'

const store = useThemeStore()
const showCreateDialog = ref(false)
const newThemeName = ref('')
const newThemeConfig = ref<Record<string, string>>({
  '--bg-primary': '#1a1a2e',
  '--bg-secondary': '#16213e',
  '--text-primary': '#e6e6e6',
  '--accent': '#4a9eff',
})

onMounted(() => {
  store.fetchThemes()
})

async function createTheme() {
  if (!newThemeName.value) return
  await store.createCustomTheme(newThemeName.value, newThemeConfig.value)
  showCreateDialog.value = false
  newThemeName.value = ''
}

function getThemePreviewColors(theme: any) {
  const config = store.getThemePreview(theme.id)
  return {
    bg: config['--bg-primary'] || '#1a1a2e',
    text: config['--text-primary'] || '#e6e6e6',
    accent: config['--accent'] || '#4a9eff',
  }
}
</script>

<template>
  <div class="theme-settings">
    <div class="settings-header">
      <h3>🎨 主题设置</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 自定义主题</button>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>
    
    <div v-else class="theme-grid">
      <div 
        v-for="theme in store.themes" 
        :key="theme.id" 
        class="theme-card"
        :class="{ active: theme.isActive }"
        @click="store.setActiveTheme(theme.id)"
      >
        <div class="theme-preview" :style="{ 
          background: getThemePreviewColors(theme).bg,
          color: getThemePreviewColors(theme).text
        }">
          <div class="preview-accent" :style="{ background: getThemePreviewColors(theme).accent }"></div>
          <div class="preview-text">Aa</div>
        </div>
        <div class="theme-name">{{ theme.name }}</div>
        <div class="theme-type">{{ theme.type }}</div>
        <button 
          v-if="theme.type === 'custom'"
          class="btn-delete"
          @click.stop="store.deleteTheme(theme.id)"
        >
          删除
        </button>
      </div>
    </div>

    <!-- 创建自定义主题对话框 -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click="showCreateDialog = false">
      <div class="dialog" @click.stop>
        <h3>创建自定义主题</h3>
        <input v-model="newThemeName" placeholder="主题名称" class="input" />
        
        <div class="color-inputs">
          <div class="color-input">
            <label>背景色</label>
            <input type="color" v-model="newThemeConfig['--bg-primary']" />
          </div>
          <div class="color-input">
            <label>文字色</label>
            <input type="color" v-model="newThemeConfig['--text-primary']" />
          </div>
          <div class="color-input">
            <label>强调色</label>
            <input type="color" v-model="newThemeConfig['--accent']" />
          </div>
        </div>

        <div class="dialog-actions">
          <button class="btn-sm" @click="showCreateDialog = false">取消</button>
          <button class="btn-sm primary" @click="createTheme">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.theme-settings {
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

.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
}

.theme-card {
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.theme-card:hover {
  border-color: var(--accent);
}

.theme-card.active {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent);
}

.theme-preview {
  width: 100%;
  height: 60px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
  position: relative;
  overflow: hidden;
}

.preview-accent {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
}

.preview-text {
  font-size: 18px;
  font-weight: bold;
}

.theme-name {
  font-size: 13px;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.theme-type {
  font-size: 11px;
  color: var(--text-secondary);
}

.btn-delete {
  margin-top: 8px;
  padding: 4px 8px;
  border: none;
  border-radius: 4px;
  background: var(--error);
  color: white;
  cursor: pointer;
  font-size: 11px;
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
  max-width: 400px;
  width: 90%;
}

.dialog h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
}

.input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  margin-bottom: 16px;
  box-sizing: border-box;
}

.color-inputs {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.color-input {
  flex: 1;
  text-align: center;
}

.color-input label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.color-input input[type="color"] {
  width: 100%;
  height: 40px;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>
