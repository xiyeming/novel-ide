<script setup lang="ts">
import { ref, computed } from "vue";
import { useProjectStore } from "../../stores/project";
import { useChapterStore } from "../../stores/chapter";
import { useAIStore } from "../../stores/ai";

const projectStore = useProjectStore();
const chapterStore = useChapterStore();
const aiStore = useAIStore();

const model = computed(() => aiStore.selectedModel);
const contextTokens = ref(92000);
const promptTokens = ref(24000);
const totalTokens = ref(156000);
const branch = ref("main");
const ragEnabled = ref(true);
const mcpServers = ref(3);
const skillsLoaded = ref(12);
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-item">🤖 {{ model }}</span>
      <span class="status-item">📊 Context {{ Math.round(contextTokens / 1000) }}K</span>
      <span class="status-item">💬 Prompt {{ Math.round(promptTokens / 1000) }}K</span>
      <span class="status-item">🔢 Token {{ Math.round(totalTokens / 1000) }}K</span>
    </div>
    <div class="status-center">
      <span class="status-item">📁 {{ projectStore.currentProject?.name || 'Novel' }}</span>
      <span class="status-item">🌿 {{ branch }}</span>
      <span class="status-item">🔍 RAG {{ ragEnabled ? 'ON' : 'OFF' }}</span>
      <span class="status-item">🔌 MCP {{ mcpServers }}</span>
      <span class="status-item">🧩 Skills {{ skillsLoaded }}</span>
    </div>
    <div class="status-right">
      <span class="status-item">Ln 1, Col 1</span>
      <span class="status-item">UTF-8</span>
      <span class="status-item">CRLF</span>
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
