<script setup lang="ts">
import { computed } from "vue";
import { useChapterStore } from "../../stores/chapter";

defineProps<{
  type: 'chapter' | 'character' | 'world' | 'prompt' | 'workflow';
}>();

const chapterStore = useChapterStore();

const chapter = computed(() => chapterStore.currentChapter);
const wordCount = computed(() => chapter.value?.content?.length || 0);
</script>

<template>
  <div class="inspector">
    <div class="inspector-header">
      <h3>{{ type }} Inspector</h3>
    </div>
    <div class="inspector-content">
      <!-- Chapter Inspector -->
      <template v-if="type === 'chapter'">
        <div class="inspector-row">
          <span class="inspector-label">字数</span>
          <span class="inspector-value">{{ wordCount }}</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">状态</span>
          <span class="inspector-value">编辑中</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">标签</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">引用角色</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">引用地点</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">章节摘要</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Character Inspector -->
      <template v-if="type === 'character'">
        <div class="inspector-row">
          <span class="inspector-label">位置</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">状态</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">装备</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">目标</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">最近出场</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- World Inspector -->
      <template v-if="type === 'world'">
        <div class="inspector-row">
          <span class="inspector-label">名称</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">描述</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">规则</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">相关角色</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">相关章节</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Prompt Inspector -->
      <template v-if="type === 'prompt'">
        <div class="inspector-row">
          <span class="inspector-label">System</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Workflow</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Context</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">RAG</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Prompt</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Workflow Inspector -->
      <template v-if="type === 'workflow'">
        <div class="inspector-row">
          <span class="inspector-label">当前阶段</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">进度</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">历史</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">配置</span>
          <span class="inspector-value">-</span>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.inspector {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-left: 1px solid var(--border-default);
}

.inspector-header {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
}

.inspector-header h3 {
  margin: 0;
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
}

.inspector-content {
  flex: 1;
  padding: var(--spacing-3);
  overflow-y: auto;
}

.inspector-row {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-2) 0;
  border-bottom: 1px solid var(--border-divider);
}

.inspector-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.inspector-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}
</style>
