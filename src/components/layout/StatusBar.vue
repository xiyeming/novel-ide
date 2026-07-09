<script setup lang="ts">
import { computed } from "vue";
import { useProjectStore } from "../../stores/project";
import { useAIStore } from "../../stores/ai";

const projectStore = useProjectStore();
const aiStore = useAIStore();

const model = computed(() => aiStore.selectedModel || "未选择模型");
const projectName = computed(() => projectStore.currentProject?.name || "未打开项目");
const wordsCount = computed(() => {
  const chapter = projectStore.currentChapter;
  return chapter?.content?.length || 0;
});
const charsCount = computed(() => {
  const chapter = projectStore.currentChapter;
  return chapter?.content?.replace(/\s/g, "").length || 0;
});
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-item">🤖 {{ model }}</span>
    </div>
    <div class="status-center">
      <span class="status-item">📁 {{ projectName }}</span>
    </div>
    <div class="status-right">
      <span class="status-item">📝 字数 {{ wordsCount }}</span>
      <span class="status-item">📄 字符 {{ charsCount }}</span>
      <span class="status-item">UTF-8</span>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 24px;
  padding: 0 var(--spacing-3);
  background: var(--bg-sidebar);
  border-top: 1px solid var(--border-default);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.status-left,
.status-center,
.status-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  cursor: pointer;
}

.status-item:hover {
  color: var(--text-primary);
}
</style>
