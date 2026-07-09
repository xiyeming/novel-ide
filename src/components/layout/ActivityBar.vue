<!-- src/components/layout/ActivityBar.vue -->
<script setup lang="ts">
type ViewType = 'explorer' | 'search' | 'ai' | 'plugins' | 'settings';

defineProps<{
  activeView: ViewType;
}>();

const emit = defineEmits<{
  select: [view: ViewType];
}>();

const views: { id: ViewType; icon: string; label: string }[] = [
  { id: "explorer", icon: "📁", label: "资源管理器" },
  { id: "search", icon: "🔍", label: "搜索" },
  { id: "ai", icon: "🤖", label: "AI" },
  { id: "plugins", icon: "🧩", label: "插件" },
  { id: "settings", icon: "⚙️", label: "设置" },
];
</script>

<template>
  <div class="activity-bar">
    <button
      v-for="view in views"
      :key="view.id"
      :class="['activity-btn', { active: activeView === view.id }]"
      @click="emit('select', view.id)"
      :title="view.label"
    >
      <span class="activity-icon">{{ view.icon }}</span>
    </button>
  </div>
</template>

<style scoped>
.activity-bar {
  display: flex;
  flex-direction: column;
  width: 56px;
  flex-shrink: 0;
  background: var(--bg-background);
  border-right: 1px solid var(--border-default);
}

.activity-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 56px;
  height: 56px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.activity-btn:hover {
  background: var(--bg-hover);
}

.activity-btn.active {
  border-left-color: var(--blue-500);
  background: var(--bg-panel);
}

.activity-icon {
  font-size: 22px;
}
</style>
