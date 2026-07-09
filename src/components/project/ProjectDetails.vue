<script setup lang="ts">
import { ref, computed } from "vue";

interface Project {
  id: string;
  name: string;
  description?: string;
  genre?: string;
  narrative_pov?: string;
  total_chapters?: number;
  words_per_chapter?: number;
  story_structure?: string;
  path: string;
  created_at: string;
  updated_at: string;
}

const props = defineProps<{
  project: Project | null;
}>();

const emit = defineEmits<{
  close: [];
}>();

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleString("zh-CN");
};
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-header">
        <h3>项目详情</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="dialog-body" v-if="project">
        <div class="detail-section">
          <div class="detail-row">
            <span class="detail-label">项目名称</span>
            <span class="detail-value">{{ project.name }}</span>
          </div>
          <div class="detail-row" v-if="project.description">
            <span class="detail-label">项目描述</span>
            <span class="detail-value">{{ project.description }}</span>
          </div>
        </div>

        <div class="detail-section">
          <div class="detail-row">
            <span class="detail-label">题材</span>
            <span class="detail-value">{{ project.genre || '未设置' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">叙事视角</span>
            <span class="detail-value">{{ project.narrative_pov || '未设置' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">总章数</span>
            <span class="detail-value">{{ project.total_chapters || '未设置' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">单章字数</span>
            <span class="detail-value">{{ project.words_per_chapter || '未设置' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">故事结构</span>
            <span class="detail-value">{{ project.story_structure || '未设置' }}</span>
          </div>
        </div>

        <div class="detail-section">
          <div class="detail-row">
            <span class="detail-label">存储路径</span>
            <span class="detail-value path">{{ project.path }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">创建时间</span>
            <span class="detail-value">{{ formatDate(project.created_at) }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">更新时间</span>
            <span class="detail-value">{{ formatDate(project.updated_at) }}</span>
          </div>
        </div>
      </div>

      <div class="dialog-footer">
        <button class="btn-secondary" @click="emit('close')">关闭</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  width: 500px;
  max-height: 80vh;
  background: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border);
}

.dialog-header h3 {
  font-size: 18px;
  font-weight: 500;
}

.close-btn {
  width: 28px;
  height: 28px;
  background: none;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 18px;
}

.close-btn:hover {
  background: var(--bg-hover);
}

.dialog-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  max-height: calc(80vh - 120px);
}

.detail-section {
  margin-bottom: var(--spacing-lg);
  padding-bottom: var(--spacing-lg);
  border-bottom: 1px solid var(--border);
}

.detail-section:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.detail-row {
  display: flex;
  padding: var(--spacing-sm) 0;
}

.detail-label {
  width: 100px;
  flex-shrink: 0;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
}

.detail-value {
  flex: 1;
  color: var(--text-primary);
}

.detail-value.path {
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  word-break: break-all;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border);
}

.btn-secondary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--bg-surface);
  color: var(--text-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.btn-secondary:hover {
  background: var(--bg-hover);
}
</style>
