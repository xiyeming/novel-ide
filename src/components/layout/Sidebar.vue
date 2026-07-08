<!-- src/components/layout/Sidebar.vue -->
<script setup lang="ts">
import { ref } from "vue";

const activeTab = ref<"files" | "search" | "settings">("files");

const tabs = [
  { id: "files" as const, icon: "📁", label: "文件" },
  { id: "search" as const, icon: "🔍", label: "搜索" },
  { id: "settings" as const, icon: "⚙️", label: "设置" },
];
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
        :title="tab.label"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
      </button>
    </div>
    <div class="sidebar-content">
      <div v-if="activeTab === 'files'" class="tab-panel">
        <div class="panel-header-sm">资源管理器</div>
        <div class="empty-state">未打开项目</div>
      </div>
      <div v-else-if="activeTab === 'search'" class="tab-panel">
        <div class="panel-header-sm">全局搜索</div>
        <div class="empty-state">输入关键词搜索</div>
      </div>
      <div v-else-if="activeTab === 'settings'" class="tab-panel">
        <div class="panel-header-sm">设置</div>
        <div class="empty-state">设置面板</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  height: 100%;
  background: var(--bg-secondary);
}

.sidebar-tabs {
  display: flex;
  flex-direction: column;
  width: 48px;
  background: var(--bg-tertiary);
  border-right: 1px solid var(--border);
}

.tab-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 48px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  background: var(--bg-hover);
}

.tab-btn.active {
  border-left-color: var(--accent);
  background: var(--bg-secondary);
}

.tab-icon {
  font-size: 20px;
}

.sidebar-content {
  flex: 1;
  overflow: auto;
}

.tab-panel {
  padding: var(--spacing-md);
}

.panel-header-sm {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--spacing-md);
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
  text-align: center;
  padding: var(--spacing-xl);
}
</style>