<script setup lang="ts">
import { ref } from "vue";
import { useTauriIPC } from "../../composables/useTauriIPC";
import { useProjectStore } from "../../stores/project";

interface ExportResult {
  file_path: string;
  file_size: number;
}

const props = defineProps<{
  chapterId?: string | null;
  chapterTitle?: string;
}>();

const emit = defineEmits<{
  close: [];
}>();

const { call } = useTauriIPC();
const projectStore = useProjectStore();

const format = ref("txt");
const outputPath = ref("");
const loading = ref(false);
const results = ref<ExportResult[]>([]);
const status = ref("");

const selectDirectory = async () => {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const dir = await open({
    directory: true,
    multiple: false,
    title: "选择导出目录",
  });
  if (dir) {
    outputPath.value = dir as string;
  }
};

const exportSingle = async () => {
  if (!props.chapterId || !outputPath.value) return;
  loading.value = true;
  status.value = "正在导出...";
  results.value = [];
  try {
    const result = await call<ExportResult>("export_chapter", {
      chapterId: props.chapterId,
      format: format.value,
      outputPath: outputPath.value,
    });
    results.value = [result];
    status.value = "导出完成";
  } catch (e) {
    status.value = "导出失败: " + String(e);
  } finally {
    loading.value = false;
  }
};

const exportAll = async () => {
  const projectId = projectStore.currentProject?.id;
  if (!projectId || !outputPath.value) return;
  loading.value = true;
  status.value = "正在导出全部章节...";
  results.value = [];
  try {
    const res = await call<ExportResult[]>("export_all_chapters", {
      projectId,
      format: format.value,
      outputPath: outputPath.value,
    });
    results.value = res;
    status.value = `导出完成，共 ${res.length} 个文件`;
  } catch (e) {
    status.value = "导出失败: " + String(e);
  } finally {
    loading.value = false;
  }
};

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + " B";
  return (bytes / 1024).toFixed(1) + " KB";
};
</script>

<template>
  <div class="export-dialog-overlay" @click.self="emit('close')">
    <div class="export-dialog">
      <div class="dialog-header">
        <span class="dialog-title">导出章节</span>
        <button class="dialog-close" @click="emit('close')">×</button>
      </div>

      <div class="dialog-body">
        <div class="form-group">
          <label>导出格式</label>
          <div class="format-selector">
            <button
              :class="['format-btn', { active: format === 'txt' }]"
              @click="format = 'txt'"
            >
              TXT 纯文本
            </button>
            <button
              :class="['format-btn', { active: format === 'md' }]"
              @click="format = 'md'"
            >
              Markdown
            </button>
          </div>
        </div>

        <div class="form-group">
          <label>输出目录</label>
          <div class="path-row">
            <input
              type="text"
              class="path-input"
              :value="outputPath"
              placeholder="请选择导出目录"
              readonly
            />
            <button class="browse-btn" @click="selectDirectory">浏览</button>
          </div>
        </div>

        <div class="export-info" v-if="chapterTitle">
          <span>当前章节: {{ chapterTitle }}</span>
        </div>

        <div class="export-actions">
          <button
            class="export-btn"
            :disabled="loading || !outputPath"
            @click="exportSingle"
            v-if="chapterId"
          >
            导出当前章节
          </button>
          <button
            class="export-btn"
            :disabled="loading || !outputPath"
            @click="exportAll"
          >
            导出全部章节
          </button>
        </div>

        <div v-if="status" class="export-status">{{ status }}</div>

        <div v-if="results.length > 0" class="export-results">
          <div class="results-title">导出文件:</div>
          <div v-for="(r, i) in results" :key="i" class="result-item">
            <span class="result-path">{{ r.file_path }}</span>
            <span class="result-size">{{ formatSize(r.file_size) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.export-dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 8px;
  width: 480px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border);
}

.dialog-title {
  font-size: var(--font-size-md);
  font-weight: 600;
  color: var(--text-primary);
}

.dialog-close {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
}

.dialog-close:hover {
  color: var(--text-primary);
}

.dialog-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-group label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-sm);
}

.format-selector {
  display: flex;
  gap: var(--spacing-sm);
}

.format-btn {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.format-btn:hover {
  border-color: var(--accent);
}

.format-btn.active {
  background: var(--accent);
  color: var(--bg-primary);
  border-color: var(--accent);
}

.path-row {
  display: flex;
  gap: var(--spacing-sm);
}

.path-input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.browse-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.browse-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.export-info {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border-radius: 6px;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-lg);
}

.export-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.export-btn {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: opacity 0.15s;
}

.export-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.export-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.export-status {
  margin-top: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border-radius: 6px;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.export-results {
  margin-top: var(--spacing-md);
}

.results-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.result-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--bg-surface);
  border-radius: 4px;
  margin-bottom: 2px;
}

.result-path {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-right: var(--spacing-sm);
}

.result-size {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  font-family: var(--font-mono);
  white-space: nowrap;
}
</style>
