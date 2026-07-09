<!-- src/components/knowledge/KnowledgePanel.vue -->
<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useKnowledgeStore } from "../../stores/knowledge";
import { useProjectStore } from "../../stores/project";

const projectStore = useProjectStore();
const knowledgeStore = useKnowledgeStore();

const hasProject = computed(() => !!projectStore.currentProject);
const showImportDialog = ref(false);
const importTitle = ref("");
const importContent = ref("");
const searchInput = ref("");
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

const handleSearchInput = () => {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    if (projectStore.currentProject && searchInput.value.trim()) {
      knowledgeStore.search(projectStore.currentProject.id, searchInput.value);
    } else {
      knowledgeStore.clearSearch();
    }
  }, 300);
};

const handleClearSearch = () => {
  searchInput.value = "";
  knowledgeStore.clearSearch();
};

const handleImport = async () => {
  if (!projectStore.currentProject || !importTitle.value.trim() || !importContent.value.trim()) return;
  await knowledgeStore.importDocument(
    projectStore.currentProject.id,
    importTitle.value.trim(),
    importContent.value.trim()
  );
  importTitle.value = "";
  importContent.value = "";
  showImportDialog.value = false;
};

const handleDelete = async (id: string) => {
  if (!confirm("确定删除该知识文档？")) return;
  await knowledgeStore.deleteDocument(id);
};

const formatDocType = (type: string) => {
  const map: Record<string, string> = {
    text: "文本",
    file: "文件",
    url: "网址",
  };
  return map[type] || type;
};

const formatDate = (dateStr: string) => {
  const d = new Date(dateStr);
  return `${d.getMonth() + 1}/${d.getDate()} ${d.getHours()}:${String(d.getMinutes()).padStart(2, "0")}`;
};

watch(
  () => projectStore.currentProject,
  (proj) => {
    if (proj) {
      knowledgeStore.fetchDocuments(proj.id);
    } else {
      knowledgeStore.documents = [];
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="knowledge-panel">
    <div class="panel-header-sm">知识库</div>

    <div v-if="!hasProject" class="empty-state">未打开项目</div>

    <template v-else>
      <!-- Action bar -->
      <div class="action-bar">
        <button class="import-btn" @click="showImportDialog = true">+ 导入</button>
      </div>

      <!-- Search -->
      <div class="search-input-wrapper">
        <input
          v-model="searchInput"
          type="text"
          class="search-input"
          placeholder="搜索知识库 + 章节…"
          @input="handleSearchInput"
        />
        <button
          v-if="searchInput"
          class="search-clear-btn"
          @click="handleClearSearch"
          title="清除"
        >
          ✕
        </button>
      </div>

      <!-- Search results -->
      <div v-if="searchInput" class="search-results">
        <div v-if="knowledgeStore.searching" class="empty-state">
          <span class="search-spinner"></span> 搜索中…
        </div>
        <div v-else-if="knowledgeStore.searchResults.length === 0" class="empty-state">
          未找到匹配结果
        </div>
        <div v-else class="result-list">
          <div
            v-for="result in knowledgeStore.searchResults"
            :key="result.doc_id"
            class="result-item"
          >
            <div class="result-header">
              <span class="result-type-badge">{{ result.source_type === "knowledge" ? "知识" : "章节" }}</span>
              <span class="result-title">{{ result.title }}</span>
            </div>
            <div class="result-snippet" v-html="result.snippet"></div>
          </div>
        </div>
      </div>

      <!-- Document list -->
      <div v-else-if="knowledgeStore.loading" class="empty-state">加载中…</div>
      <div v-else-if="knowledgeStore.documents.length === 0" class="empty-state">
        暂无知识文档，点击"导入"添加
      </div>
      <div v-else class="doc-list">
        <div v-for="doc in knowledgeStore.documents" :key="doc.id" class="doc-item">
          <div class="doc-info">
            <div class="doc-title">{{ doc.title }}</div>
            <div class="doc-meta">
              <span class="doc-type">{{ formatDocType(doc.doc_type) }}</span>
              <span class="doc-chunks">{{ doc.chunk_count }} 块</span>
              <span class="doc-date">{{ formatDate(doc.created_at) }}</span>
            </div>
          </div>
          <button class="doc-delete-btn" @click="handleDelete(doc.id)" title="删除">🗑️</button>
        </div>
      </div>
    </template>

    <!-- Import dialog -->
    <Teleport to="body">
      <div v-if="showImportDialog" class="dialog-overlay" @click.self="showImportDialog = false">
        <div class="dialog">
          <div class="dialog-title">导入知识文档</div>
          <div class="dialog-body">
            <label class="field-label">标题</label>
            <input
              v-model="importTitle"
              type="text"
              class="field-input"
              placeholder="文档标题"
            />
            <label class="field-label">内容</label>
            <textarea
              v-model="importContent"
              class="field-textarea"
              placeholder="粘贴文本内容…"
              rows="8"
            ></textarea>
          </div>
          <div class="dialog-footer">
            <button class="dialog-cancel" @click="showImportDialog = false">取消</button>
            <button
              class="dialog-confirm"
              :disabled="!importTitle.trim() || !importContent.trim()"
              @click="handleImport"
            >
              导入
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.knowledge-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
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

.action-bar {
  margin-bottom: var(--spacing-md);
}

.import-btn {
  width: 100%;
  padding: var(--spacing-sm);
  background: transparent;
  border: 1px dashed var(--accent);
  border-radius: 4px;
  color: var(--accent);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.import-btn:hover {
  background: var(--accent);
  color: var(--bg-primary);
}

.search-input-wrapper {
  position: relative;
  margin-bottom: var(--spacing-md);
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-right: 28px;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  transition: border-color 0.15s;
}

.search-input:focus {
  border-color: var(--accent);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-clear-btn {
  position: absolute;
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 12px;
  cursor: pointer;
  padding: 2px 4px;
  line-height: 1;
}

.search-clear-btn:hover {
  color: var(--text-primary);
}

.search-spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid var(--text-muted);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  vertical-align: middle;
  margin-right: 4px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.result-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.result-item {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 4px;
  background: var(--bg-hover);
}

.result-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: 4px;
}

.result-type-badge {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 3px;
  background: var(--accent);
  color: var(--bg-primary);
  font-weight: 500;
}

.result-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.result-snippet {
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.4;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.result-snippet :deep(mark) {
  color: var(--accent);
  font-weight: 600;
}

.doc-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.doc-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 4px;
  transition: background 0.15s;
}

.doc-item:hover {
  background: var(--bg-hover);
}

.doc-info {
  flex: 1;
  min-width: 0;
}

.doc-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.doc-meta {
  display: flex;
  gap: var(--spacing-sm);
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 2px;
}

.doc-type {
  color: var(--accent);
}

.doc-delete-btn {
  background: none;
  border: none;
  font-size: 14px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
  padding: 2px 4px;
}

.doc-item:hover .doc-delete-btn {
  opacity: 1;
}

.doc-delete-btn:hover {
  transform: scale(1.2);
}

/* Dialog styles */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.dialog {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  width: 480px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.dialog-title {
  padding: var(--spacing-md) var(--spacing-lg);
  font-size: var(--font-size-base);
  font-weight: 600;
  border-bottom: 1px solid var(--border);
}

.dialog-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
}

.field-label {
  display: block;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.field-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  margin-bottom: var(--spacing-md);
}

.field-input:focus {
  border-color: var(--accent);
}

.field-textarea {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  resize: vertical;
  font-family: inherit;
}

.field-textarea:focus {
  border-color: var(--accent);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border);
}

.dialog-cancel {
  padding: var(--spacing-sm) var(--spacing-md);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
}

.dialog-cancel:hover {
  background: var(--bg-hover);
}

.dialog-confirm {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--accent);
  border: none;
  border-radius: 4px;
  color: var(--bg-primary);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
}

.dialog-confirm:hover {
  opacity: 0.9;
}

.dialog-confirm:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
