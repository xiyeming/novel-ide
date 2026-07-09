<!-- src/components/layout/Sidebar.vue -->
<script setup lang="ts">
import { ref, computed, watch, onUnmounted } from "vue";
import { useChapterStore } from "../../stores/chapter";
import { useProjectStore } from "../../stores/project";
import { useSearchStore } from "../../stores/search";
import ProjectConfig from "../project/ProjectConfig.vue";
import ModelManager from "../settings/ModelManager.vue";
import KnowledgePanel from "../knowledge/KnowledgePanel.vue";
import WorkflowPanel from "../workflow/WorkflowPanel.vue";
import AgentPanel from "../agent/AgentPanel.vue";
import CloudPanel from "../cloud/CloudPanel.vue";
import ShortcutSettings from "../settings/ShortcutSettings.vue";
import ThemeSettings from "../settings/ThemeSettings.vue";
import SettingsPanel from "../settings/SettingsPanel.vue";

type ViewType = 'explorer' | 'search' | 'ai' | 'plugins' | 'settings';
type SidebarTab = "files" | "search" | "knowledge" | "config" | "model" | "workflow" | "agent" | "cloud" | "shortcut" | "theme" | "settings";

const viewToTab: Record<ViewType, SidebarTab> = {
  explorer: "files",
  search: "search",
  ai: "model",
  plugins: "settings",
  settings: "settings",
};

const props = defineProps<{
  view?: string;
}>();

const emit = defineEmits<{
  openChapter: [chapterId: string];
}>();

const activeTab = ref<SidebarTab>("files");

watch(() => props.view, (newView) => {
  if (newView && newView in viewToTab) {
    activeTab.value = viewToTab[newView as ViewType];
  }
}, { immediate: true });
const chapterStore = useChapterStore();
const projectStore = useProjectStore();
const searchStore = useSearchStore();
const hasProject = computed(() => !!projectStore.currentProject);

const searchInput = ref("");
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

const handleSearchInput = () => {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    if (projectStore.currentProject) {
      searchStore.search(projectStore.currentProject.id, searchInput.value);
    }
  }, 300);
};

const sanitizeSnippet = (s: string): string => {
  return s.replace(/<[^>]*>/g, (tag) => (tag === "<b>" || tag === "</b>" || tag === "<b/>" ? tag : ""));
};

const handleClearSearch = () => {
  searchInput.value = "";
  searchStore.clear();
};

onUnmounted(() => {
  if (debounceTimer) clearTimeout(debounceTimer);
});

watch(activeTab, (tab) => {
  if (tab !== "search") {
    handleClearSearch();
  }
});

const tabs = [
  { id: "files" as const, icon: "📁", label: "文件" },
  { id: "search" as const, icon: "🔍", label: "搜索" },
  { id: "knowledge" as const, icon: "📚", label: "知识库" },
  { id: "config" as const, icon: "📝", label: "配置" },
  { id: "model" as const, icon: "🤖", label: "模型" },
  { id: "workflow" as const, icon: "⚙️", label: "工作流" },
  { id: "agent" as const, icon: "🧠", label: "智能体" },
  { id: "cloud" as const, icon: "☁️", label: "云同步" },
  { id: "shortcut" as const, icon: "⌨️", label: "快捷键" },
  { id: "theme" as const, icon: "🎨", label: "主题" },
  { id: "settings" as const, icon: "⚙️", label: "设置" },
];

// Context menu state
const contextMenu = ref<{
  show: boolean;
  x: number;
  y: number;
  chapterId: string;
  chapterTitle: string;
}>({
  show: false,
  x: 0,
  y: 0,
  chapterId: "",
  chapterTitle: "",
});

// Inline editing state
const editingId = ref<string | null>(null);
const editingTitle = ref("");

const handleChapterClick = (chapterId: string) => {
  emit("openChapter", chapterId);
};

const handleNewChapter = async () => {
  if (!projectStore.currentProject) return;
  const chapter = await chapterStore.createChapter(
    projectStore.currentProject.id,
    "未命名章节"
  );
  emit("openChapter", chapter.id);
};

const handleContextMenu = (e: MouseEvent, chapterId: string, chapterTitle: string) => {
  e.preventDefault();
  contextMenu.value = {
    show: true,
    x: e.clientX,
    y: e.clientY,
    chapterId,
    chapterTitle,
  };
};

const closeContextMenu = () => {
  contextMenu.value.show = false;
};

const startRename = () => {
  editingId.value = contextMenu.value.chapterId;
  editingTitle.value = contextMenu.value.chapterTitle;
  closeContextMenu();
};

const finishRename = async () => {
  if (editingId.value && editingTitle.value.trim()) {
    await chapterStore.updateChapterTitle(editingId.value, editingTitle.value.trim());
  }
  editingId.value = null;
  editingTitle.value = "";
};

const cancelRename = () => {
  editingId.value = null;
  editingTitle.value = "";
};

const handleDelete = async () => {
  if (contextMenu.value.chapterId) {
    await chapterStore.deleteChapter(contextMenu.value.chapterId);
  }
  closeContextMenu();
};

const formatWordCount = (count: number) => {
  if (count >= 10000) {
    return `${(count / 10000).toFixed(1)}万`;
  }
  return `${count}`;
};
</script>

<template>
  <div class="sidebar" @click="closeContextMenu">
    <div class="sidebar-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
        :title="tab.label"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
      </button>
    </div>
    <div class="sidebar-content">
      <div v-if="activeTab === 'files'" class="tab-panel">
        <div class="panel-header-sm">资源管理器</div>
        <div v-if="!hasProject" class="empty-state">未打开项目</div>
        <div v-else-if="chapterStore.chapters.length === 0" class="empty-state">
          <p>暂无章节</p>
          <button class="new-chapter-btn" @click="handleNewChapter">
            + 新建章节
          </button>
        </div>
        <div v-else class="chapter-list">
          <div
            v-for="chapter in chapterStore.chapters"
            :key="chapter.id"
            :class="['chapter-item', { active: chapterStore.currentChapter?.id === chapter.id }]"
            @click="handleChapterClick(chapter.id)"
            @contextmenu="handleContextMenu($event, chapter.id, chapter.title)"
          >
            <span class="chapter-icon">📄</span>
            <div class="chapter-info">
              <input
                v-if="editingId === chapter.id"
                v-model="editingTitle"
                class="chapter-title-input"
                @blur="finishRename"
                @keyup.enter="finishRename"
                @keyup.escape="cancelRename"
                @click.stop
                autofocus
              />
              <span v-else class="chapter-title">{{ chapter.title }}</span>
              <span class="chapter-word-count">{{ formatWordCount(chapter.word_count) }}字</span>
            </div>
          </div>
          <button class="new-chapter-btn" @click="handleNewChapter">
            + 新建章节
          </button>
        </div>
      </div>
      <div v-else-if="activeTab === 'search'" class="tab-panel">
        <div class="panel-header-sm">全局搜索</div>
        <div class="search-input-wrapper">
          <input
            v-model="searchInput"
            type="text"
            class="search-input"
            placeholder="输入关键词搜索…"
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
        <div v-if="!hasProject" class="empty-state">未打开项目</div>
        <div v-else-if="searchStore.loading" class="empty-state">
          <span class="search-spinner"></span> 搜索中…
        </div>
        <div v-else-if="searchStore.query && searchStore.results.length === 0" class="empty-state">
          未找到匹配结果
        </div>
        <div v-else-if="searchStore.results.length > 0" class="search-results">
          <div
            v-for="result in searchStore.results"
            :key="result.chapter_id"
            class="search-result-item"
            @click="handleChapterClick(result.chapter_id)"
          >
            <div class="search-result-title">📄 {{ result.chapter_title }}</div>
            <div class="search-result-snippet" v-html="sanitizeSnippet(result.snippet)"></div>
          </div>
        </div>
        <div v-else-if="!searchStore.query" class="empty-state">输入关键词搜索</div>
      </div>
      <div v-else-if="activeTab === 'knowledge'" class="tab-panel">
        <KnowledgePanel />
      </div>
      <div v-else-if="activeTab === 'config'" class="tab-panel">
        <div class="panel-header-sm">项目配置</div>
        <div v-if="!hasProject" class="empty-state">未打开项目</div>
        <ProjectConfig v-else />
      </div>
      <div v-else-if="activeTab === 'model'" class="tab-panel">
        <ModelManager />
      </div>
      <div v-else-if="activeTab === 'workflow'" class="tab-panel">
        <WorkflowPanel />
      </div>
      <div v-else-if="activeTab === 'agent'" class="tab-panel">
        <AgentPanel />
      </div>
      <div v-else-if="activeTab === 'cloud'" class="tab-panel">
        <CloudPanel />
      </div>
      <div v-else-if="activeTab === 'shortcut'" class="tab-panel">
        <ShortcutSettings />
      </div>
      <div v-else-if="activeTab === 'theme'" class="tab-panel">
        <ThemeSettings />
      </div>
      <div v-else-if="activeTab === 'settings'" class="tab-panel">
        <SettingsPanel />
      </div>
    </div>

    <!-- Context Menu -->
    <Teleport to="body">
      <div
        v-if="contextMenu.show"
        class="context-menu"
        :style="{ left: `${contextMenu.x}px`, top: `${contextMenu.y}px` }"
      >
        <button class="context-menu-item" @click="startRename">
          ✏️ 重命名
        </button>
        <button class="context-menu-item danger" @click="handleDelete">
          🗑️ 删除
        </button>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  height: 100%;
  background: var(--bg-secondary);
}

.sidebar-tabs {
  display: flex;
  flex-direction: column;
  width: 48px;
  background: var(--bg-tertiary);
  border-right: 1px solid var(--border);
}

.tab-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 48px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  background: var(--bg-hover);
}

.tab-btn.active {
  border-left-color: var(--accent);
  background: var(--bg-secondary);
}

.tab-icon {
  font-size: 20px;
}

.sidebar-content {
  flex: 1;
  overflow: auto;
}

.tab-panel {
  padding: var(--spacing-md);
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

.chapter-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.chapter-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s;
}

.chapter-item:hover {
  background: var(--bg-hover);
}

.chapter-item.active {
  background: var(--accent);
  color: var(--bg-primary);
}

.chapter-icon {
  font-size: 14px;
}

.chapter-info {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.chapter-title {
  font-size: var(--font-size-sm);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.chapter-title-input {
  font-size: var(--font-size-sm);
  background: var(--bg-primary);
  border: 1px solid var(--accent);
  border-radius: 3px;
  padding: 2px 4px;
  color: var(--text-primary);
  outline: none;
}

.chapter-word-count {
  font-size: 11px;
  color: var(--text-muted);
}

.chapter-item.active .chapter-word-count {
  color: var(--bg-primary);
  opacity: 0.8;
}

.new-chapter-btn {
  width: 100%;
  padding: var(--spacing-sm);
  background: transparent;
  border: 1px dashed var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
  margin-top: var(--spacing-sm);
}

.new-chapter-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--accent);
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

.search-results {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.search-result-item {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s;
}

.search-result-item:hover {
  background: var(--bg-hover);
}

.search-result-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.search-result-snippet {
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.4;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.search-result-snippet :deep(b) {
  color: var(--accent);
  font-weight: 600;
}
</style>

<style>
.context-menu {
  position: fixed;
  z-index: 1000;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: var(--spacing-xs);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  min-width: 120px;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: none;
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  text-align: left;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-item.danger {
  color: var(--error);
}

.context-menu-item.danger:hover {
  background: var(--error);
  color: white;
}
</style>
