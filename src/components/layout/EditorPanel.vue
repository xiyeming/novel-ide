<!-- src/components/layout/EditorPanel.vue -->
<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import MonacoEditor from "../editor/MonacoEditor.vue";
import VersionHistory from "../editor/VersionHistory.vue";
import ProofreadPanel from "../editor/ProofreadPanel.vue";
import ExportDialog from "../editor/ExportDialog.vue";
import { useChapterStore } from "../../stores/chapter";
import { useProjectStore } from "../../stores/project";

interface Tab {
  id: string;
  name: string;
  content: string;
  dirty: boolean;
}

const chapterStore = useChapterStore();
const projectStore = useProjectStore();
const tabs = ref<Tab[]>([]);
const activeTabId = ref<string | null>(null);
const editorContent = ref("");
const versionHistoryVisible = ref(false);
const versionHistoryRef = ref<InstanceType<typeof VersionHistory> | null>(null);
const proofreadVisible = ref(false);
const proofreadRef = ref<InstanceType<typeof ProofreadPanel> | null>(null);
const monacoEditorRef = ref<InstanceType<typeof MonacoEditor> | null>(null);
const exportDialogVisible = ref(false);
let autoSaveTimer: ReturnType<typeof setTimeout> | null = null;

onUnmounted(() => {
  if (autoSaveTimer) clearTimeout(autoSaveTimer);
});

const getActiveTab = () => tabs.value.find((t) => t.id === activeTabId.value);

const openTab = (id: string, name: string) => {
  const existing = tabs.value.find((t) => t.id === id);
  if (!existing) {
    tabs.value.push({ id, name, content: "", dirty: false });
  }
  activeTabId.value = id;

  const tab = tabs.value.find((t) => t.id === id);
  if (tab) {
    // If chapter content not loaded yet, fetch it
    if (!tab.content) {
      chapterStore.openChapter(id).then((chapter) => {
        if (chapter && tab) {
          tab.content = chapter.content;
          editorContent.value = chapter.content;
        }
      });
    } else {
      editorContent.value = tab.content;
    }
  }
};

const closeTab = (id: string) => {
  const tab = tabs.value.find((t) => t.id === id);
  if (tab?.dirty && !confirm(`"${tab.name}" 有未保存的更改，确定要关闭吗？`)) {
    return;
  }
  tabs.value = tabs.value.filter((t) => t.id !== id);
  if (activeTabId.value === id) {
    activeTabId.value = tabs.value.length > 0 ? tabs.value[tabs.value.length - 1].id : null;
    if (activeTabId.value) {
      const nextTab = tabs.value.find((t) => t.id === activeTabId.value);
      editorContent.value = nextTab?.content ?? "";
    }
  }
};

const handleContentChange = (value: string) => {
  editorContent.value = value;
  const tab = getActiveTab();
  if (tab) {
    tab.content = value;
    tab.dirty = true;
  }

  // Auto-save with 1s debounce
  if (autoSaveTimer) clearTimeout(autoSaveTimer);
  autoSaveTimer = setTimeout(() => {
    if (activeTabId.value) {
      chapterStore.updateChapterContent(activeTabId.value, value);
      const t = getActiveTab();
      if (t) t.dirty = false;
    }
  }, 1000);
};

const handleSave = () => {
  if (autoSaveTimer) clearTimeout(autoSaveTimer);
  if (activeTabId.value) {
    chapterStore.updateChapterContent(activeTabId.value, editorContent.value);
    const t = getActiveTab();
    if (t) t.dirty = false;
  }
};

const handleNewTab = () => {
  const projectId = projectStore.currentProject?.id;
  if (!projectId) {
    alert("请先打开一个项目");
    return;
  }
  chapterStore.createChapter(projectId, "未命名章节").then((chapter) => {
    openTab(chapter.id, chapter.title);
  });
};

const toggleVersionHistory = () => {
  versionHistoryVisible.value = !versionHistoryVisible.value;
  if (versionHistoryVisible.value) {
    setTimeout(() => versionHistoryRef.value?.fetchVersions(), 50);
  }
};

const toggleProofread = () => {
  proofreadVisible.value = !proofreadVisible.value;
};

const handleProofreadFix = (error: { original: string; suggestion: string; line: number; column: number }) => {
  const lines = editorContent.value.split("\n");
  if (error.line > 0 && error.line <= lines.length) {
    const line = lines[error.line - 1];
    const colIndex = Math.max(0, error.column - 1);
    const segment = line.substring(colIndex);
    const idx = segment.indexOf(error.original);
    if (idx >= 0) {
      const before = line.substring(0, colIndex + idx);
      const after = line.substring(colIndex + idx + error.original.length);
      lines[error.line - 1] = before + error.suggestion + after;
      editorContent.value = lines.join("\n");
      handleContentChange(editorContent.value);
    }
  }
};

const handleProofreadFixAll = (errors: Array<{ original: string; suggestion: string; line: number; column: number }>) => {
  let content = editorContent.value;
  for (const error of errors) {
    content = content.replace(error.original, error.suggestion);
  }
  editorContent.value = content;
  handleContentChange(content);
};

const handleJumpToLine = (line: number) => {
  const editorInstance = monacoEditorRef.value?.editor;
  if (editorInstance) {
    editorInstance.revealLineInCenter(line);
    editorInstance.setPosition({ lineNumber: line, column: 1 });
    editorInstance.focus();
  }
};

const handleVersionRestored = (content: string) => {
  editorContent.value = content;
  const tab = getActiveTab();
  if (tab) {
    tab.content = content;
    tab.dirty = false;
  }
};

// Expose openTab for external callers
defineExpose({ openTab });
</script>

<template>
  <div class="editor-panel">
    <div class="editor-tabs" v-if="tabs.length > 0">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        :class="['editor-tab', { active: tab.id === activeTabId }]"
        @click="openTab(tab.id, tab.name)"
      >
        <span class="tab-name">{{ tab.name }}</span>
        <span v-if="tab.dirty" class="tab-dirty">●</span>
        <button class="tab-close" @click.stop="closeTab(tab.id)">×</button>
      </div>
    </div>
    <div v-if="activeTabId" class="editor-toolbar">
      <button
        :class="['toolbar-btn', { active: versionHistoryVisible }]"
        @click="toggleVersionHistory"
        title="版本历史"
      >
        🕐
      </button>
      <button
        :class="['toolbar-btn', { active: proofreadVisible }]"
        @click="toggleProofread"
        title="校对"
      >
        🔍
      </button>
      <button
        class="toolbar-btn"
        @click="exportDialogVisible = true"
        title="导出"
      >
        📦
      </button>
    </div>
    <div class="editor-body">
      <div class="editor-content">
        <div v-if="!activeTabId" class="welcome-screen">
          <div class="welcome-icon">📖</div>
          <h2>Novel IDE</h2>
          <p>专业小说创作 IDE</p>
          <div class="welcome-actions">
            <button class="action-btn" @click="handleNewTab">新建章节</button>
            <button class="action-btn secondary">打开项目</button>
          </div>
        </div>
        <MonacoEditor
          v-else
          ref="monacoEditorRef"
          :modelValue="editorContent"
          @update:modelValue="handleContentChange"
          @save="handleSave"
        />
      </div>
      <VersionHistory
        v-if="versionHistoryVisible && activeTabId"
        ref="versionHistoryRef"
        :chapterId="activeTabId"
        :currentContent="editorContent"
        @restored="handleVersionRestored"
      />
      <ProofreadPanel
        v-if="proofreadVisible && activeTabId"
        ref="proofreadRef"
        :chapterId="activeTabId"
        :content="editorContent"
        @fix="handleProofreadFix"
        @fixAll="handleProofreadFixAll"
        @jumpToLine="handleJumpToLine"
      />
    </div>
    <ExportDialog
      v-if="exportDialogVisible"
      :chapterId="activeTabId"
      :chapterTitle="getActiveTab()?.name"
      @close="exportDialogVisible = false"
    />
  </div>
</template>

<style scoped>
.editor-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.editor-tabs {
  display: flex;
  height: 36px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  overflow-x: auto;
}

.editor-tab {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 0 var(--spacing-md);
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  transition: all 0.15s;
}

.editor-tab:hover {
  background: var(--bg-surface);
}

.editor-tab.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-bottom: 2px solid var(--accent);
}

.tab-dirty {
  color: var(--warning);
  font-size: 10px;
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: none;
  border: none;
  border-radius: 3px;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 14px;
}

.tab-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.editor-content {
  flex: 1;
  overflow: hidden;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 2px var(--spacing-sm);
  height: 28px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: none;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.15s;
}

.toolbar-btn:hover {
  background: var(--bg-hover);
}

.toolbar-btn.active {
  background: var(--bg-surface);
  box-shadow: inset 0 0 0 1px var(--accent);
}

.editor-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.welcome-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: var(--spacing-lg);
}

.welcome-icon {
  font-size: 64px;
}

.welcome-screen h2 {
  font-size: 24px;
  font-weight: 500;
  color: var(--text-primary);
}

.welcome-screen p {
  color: var(--text-muted);
}

.welcome-actions {
  display: flex;
  gap: var(--spacing-md);
  margin-top: var(--spacing-lg);
}

.action-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  font-size: var(--font-size-md);
  cursor: pointer;
  transition: opacity 0.15s;
}

.action-btn:hover {
  opacity: 0.9;
}

.action-btn.secondary {
  background: var(--bg-surface);
  color: var(--text-primary);
}
</style>
