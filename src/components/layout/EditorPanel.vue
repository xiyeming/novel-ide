<!-- src/components/layout/EditorPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const tabs = ref<{ id: string; name: string; active: boolean }[]>([]);
const activeTabId = ref<string | null>(null);

const openTab = (id: string, name: string) => {
  const exists = tabs.value.find((t) => t.id === id);
  if (!exists) {
    tabs.value.push({ id, name, active: true });
  }
  tabs.value.forEach((t) => (t.active = t.id === id));
  activeTabId.value = id;
};

const closeTab = (id: string) => {
  tabs.value = tabs.value.filter((t) => t.id !== id);
  if (activeTabId.value === id) {
    activeTabId.value = tabs.value.length > 0 ? tabs.value[tabs.value.length - 1].id : null;
  }
};
</script>

<template>
  <div class="editor-panel">
    <div class="editor-tabs" v-if="tabs.length > 0">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        :class="['editor-tab', { active: tab.active }]"
        @click="openTab(tab.id, tab.name)"
      >
        <span class="tab-name">{{ tab.name }}</span>
        <button class="tab-close" @click.stop="closeTab(tab.id)">×</button>
      </div>
    </div>
    <div class="editor-content">
      <div v-if="!activeTabId" class="welcome-screen">
        <div class="welcome-icon">📖</div>
        <h2>Novel IDE</h2>
        <p>专业小说创作 IDE</p>
        <div class="welcome-actions">
          <button class="action-btn">新建项目</button>
          <button class="action-btn secondary">打开项目</button>
        </div>
      </div>
      <div v-else class="editor-placeholder">
        <div class="monaco-container" :id="`editor-${activeTabId}`"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.editor-tabs {
  display: flex;
  height: 36px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  overflow-x: auto;
}

.editor-tab {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 0 var(--spacing-md);
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  transition: all 0.15s;
}

.editor-tab:hover {
  background: var(--bg-surface);
}

.editor-tab.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-bottom: 2px solid var(--accent);
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: none;
  border: none;
  border-radius: 3px;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 14px;
}

.tab-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.editor-content {
  flex: 1;
  overflow: hidden;
}

.welcome-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: var(--spacing-lg);
}

.welcome-icon {
  font-size: 64px;
}

.welcome-screen h2 {
  font-size: 24px;
  font-weight: 500;
  color: var(--text-primary);
}

.welcome-screen p {
  color: var(--text-muted);
}

.welcome-actions {
  display: flex;
  gap: var(--spacing-md);
  margin-top: var(--spacing-lg);
}

.action-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  font-size: var(--font-size-md);
  cursor: pointer;
  transition: opacity 0.15s;
}

.action-btn:hover {
  opacity: 0.9;
}

.action-btn.secondary {
  background: var(--bg-surface);
  color: var(--text-primary);
}

.monaco-container {
  width: 100%;
  height: 100%;
}
</style>