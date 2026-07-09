<script setup lang="ts">
defineProps<{
  activeView: string;
  aiStudioVisible?: boolean;
}>();

const emit = defineEmits<{
  select: [view: string];
  toggleAI: [];
  toggleSidebar: [];
}>();

const views = [
  { id: 'explorer', icon: '📁', label: '资源管理器' },
  { id: 'search', icon: '🔍', label: '搜索' },
];
</script>

<template>
  <div class="activity-bar">
    <div class="activity-top">
      <button
        v-for="view in views"
        :key="view.id"
        :class="['activity-item', { active: activeView === view.id }]"
        @click="emit('select', view.id)"
        :title="view.label"
      >
        <span class="activity-icon">{{ view.icon }}</span>
      </button>
    </div>
    <div class="activity-bottom">
      <button
        :class="['activity-item', { active: aiStudioVisible }]"
        @click="emit('toggleAI')"
        title="AI 工作室"
      >
        <span class="activity-icon">🤖</span>
      </button>
      <button
        class="activity-item"
        @click="emit('toggleSidebar')"
        title="切换侧边栏"
      >
        <span class="activity-icon">📋</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.activity-bar {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 56px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border-default);
}

.activity-top,
.activity-bottom {
  display: flex;
  flex-direction: column;
}

.activity-item {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 56px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.activity-item:hover {
  background: var(--bg-hover);
}

.activity-item.active {
  border-left-color: var(--blue-500);
  background: var(--bg-panel);
}

.activity-icon {
  font-size: 20px;
}
</style>
