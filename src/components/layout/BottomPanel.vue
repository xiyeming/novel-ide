<!-- src/components/layout/BottomPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const activeTab = ref<"output" | "terminal" | "problems">("output");

const tabs = [
  { id: "output" as const, label: "输出" },
  { id: "terminal" as const, label: "终端" },
  { id: "problems" as const, label: "问题" },
];
</script>

<template>
  <div class="bottom-panel">
    <div class="panel-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>
    <div class="panel-content">
      <div v-if="activeTab === 'output'" class="tab-body">
        <div class="empty-state">暂无输出</div>
      </div>
      <div v-else-if="activeTab === 'terminal'" class="tab-body">
        <div class="empty-state">终端功能开发中</div>
      </div>
      <div v-else-if="activeTab === 'problems'" class="tab-body">
        <div class="empty-state">暂无问题</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bottom-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border);
}

.panel-tabs {
  display: flex;
  height: 32px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
}

.tab-btn {
  padding: 0 var(--spacing-md);
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  color: var(--text-primary);
  border-bottom-color: var(--accent);
}

.panel-content {
  flex: 1;
  overflow: auto;
}

.tab-body {
  padding: var(--spacing-md);
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
}
</style>