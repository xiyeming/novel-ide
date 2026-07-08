<!-- src/components/layout/Sidebar.vue -->
<script setup lang="ts">
import { ref, computed } from "vue";
import { useChapterStore } from "../../stores/chapter";
import { useProjectStore } from "../../stores/project";

const emit = defineEmits<{
  openChapter: [chapterId: string];
}>();

const activeTab = ref<"files" | "search" | "settings">("files");
const chapterStore = useChapterStore();
const projectStore = useProjectStore();
const hasProject = computed(() => !!projectStore.currentProject);

const tabs = [
  { id: "files" as const, icon: "📁", label: "文件" },
  { id: "search" as const, icon: "🔍", label: "搜索" },
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
        <div class="empty-state">输入关键词搜索</div>
      </div>
      <div v-else-if="activeTab === 'settings'" class="tab-panel">
        <div class="panel-header-sm">设置</div>
        <div class="empty-state">设置面板</div>
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
