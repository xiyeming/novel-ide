<!-- src/components/layout/IDELayout.vue -->
<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import TitleBar from "./TitleBar.vue";
import ActivityBar from "./ActivityBar.vue";
import Sidebar from "./Sidebar.vue";
import Breadcrumb from "./Breadcrumb.vue";
import EditorPanel from "./EditorPanel.vue";
import BottomPanel from "./BottomPanel.vue";
import AIStudio from "../ai/AIStudio.vue";
import StatusBar from "./StatusBar.vue";
import { useProjectStore } from "../../stores/project";
import { useChapterStore } from "../../stores/chapter";

const emit = defineEmits<{
  back: [];
}>();

const projectStore = useProjectStore();
const chapterStore = useChapterStore();
const editorPanelRef = ref<InstanceType<typeof EditorPanel> | null>(null);

const sidebarWidth = ref(280);
const aiStudioWidth = ref(420);
const bottomPanelHeight = ref(200);

const activeView = ref<string>('explorer');
const sidebarVisible = ref(true);
const aiStudioVisible = ref(false);
const bottomPanelVisible = ref(false);

const breadcrumbItems = computed(() => {
  const items: Array<{ label: string }> = [];
  if (projectStore.currentProject) {
    items.push({ label: projectStore.currentProject.name });
  }
  if (chapterStore.currentChapter) {
    items.push({ label: chapterStore.currentChapter.title });
  }
  return items;
});

const isDragging = ref(false);
const dragTarget = ref<"sidebar" | "ai" | "bottom" | null>(null);

const onMouseDown = (target: "sidebar" | "ai" | "bottom", e: MouseEvent) => {
  isDragging.value = true;
  dragTarget.value = target;
  e.preventDefault();
};

const onMouseMove = (e: MouseEvent) => {
  if (!isDragging.value) return;

  if (dragTarget.value === "sidebar") {
    sidebarWidth.value = Math.max(200, Math.min(e.clientX, 500));
  } else if (dragTarget.value === "ai") {
    aiStudioWidth.value = Math.max(320, Math.min(window.innerWidth - e.clientX, 600));
  } else if (dragTarget.value === "bottom") {
    bottomPanelHeight.value = Math.max(100, Math.min(window.innerHeight - e.clientY, 400));
  }
};

const onMouseUp = () => {
  isDragging.value = false;
  dragTarget.value = null;
};

const toggleAIStudio = () => {
  aiStudioVisible.value = !aiStudioVisible.value;
};

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value;
};

onMounted(() => {
  window.addEventListener("mousemove", onMouseMove);
  window.addEventListener("mouseup", onMouseUp);
});

onUnmounted(() => {
  window.removeEventListener("mousemove", onMouseMove);
  window.removeEventListener("mouseup", onMouseUp);
});

watch(
  () => projectStore.currentProject,
  async (project) => {
    if (project) {
      await chapterStore.fetchChapters(project.id);
    } else {
      chapterStore.chapters = [];
      chapterStore.currentChapter = null;
    }
  },
  { immediate: true }
);

const handleOpenChapter = (chapterId: string) => {
  const chapter = chapterStore.chapters.find((c) => c.id === chapterId);
  if (chapter && editorPanelRef.value) {
    editorPanelRef.value.openTab(chapter.id, chapter.title);
  }
};

defineExpose({
  toggleAIStudio,
  toggleSidebar,
  aiStudioVisible,
  sidebarVisible,
});
</script>

<template>
  <div class="ide-layout">
    <TitleBar @back="emit('back')" />
    <div class="ide-main">
      <ActivityBar
        :activeView="activeView"
        :aiStudioVisible="aiStudioVisible"
        @select="activeView = $event"
        @toggleAI="toggleAIStudio"
        @toggleSidebar="toggleSidebar"
      />
      <div class="sidebar" :style="{ width: sidebarVisible ? `${sidebarWidth}px` : '0' }">
        <Sidebar :view="activeView" @openChapter="handleOpenChapter" />
      </div>
      <div
        v-if="sidebarVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('sidebar', $event)"
      />
      <div class="editor-area">
        <Breadcrumb :items="breadcrumbItems" />
        <div class="editor-workspace" :style="{ height: bottomPanelVisible ? `calc(100% - ${bottomPanelHeight}px)` : '100%' }">
          <EditorPanel ref="editorPanelRef" />
        </div>
        <div
          v-if="bottomPanelVisible"
          class="resize-handle horizontal"
          @mousedown="onMouseDown('bottom', $event)"
        />
        <BottomPanel v-if="bottomPanelVisible" />
      </div>
      <div
        v-if="aiStudioVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('ai', $event)"
      />
      <div class="ai-studio" :style="{ width: aiStudioVisible ? `${aiStudioWidth}px` : '0' }">
        <AIStudio />
      </div>
    </div>
    <StatusBar />
  </div>
</template>

<style scoped>
.ide-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-background);
}

.ide-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.ai-studio {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.resize-handle {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background var(--duration-fast) var(--ease-out);
}

.resize-handle:hover {
  background: var(--blue-500);
}

.resize-handle.horizontal {
  width: 100%;
  height: 4px;
  cursor: row-resize;
}
</style>
