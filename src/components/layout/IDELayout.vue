<!-- src/components/layout/IDELayout.vue -->
<script setup lang="ts">
import { ref } from "vue";
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar.vue";
import EditorPanel from "./EditorPanel.vue";
import AIPanel from "./AIPanel.vue";
import BottomPanel from "./BottomPanel.vue";

const sidebarWidth = ref(260);
const aiPanelWidth = ref(320);
const bottomPanelHeight = ref(200);
const showBottomPanel = ref(true);
const showAIPanel = ref(true);

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
    aiPanelWidth.value = Math.max(250, Math.min(window.innerWidth - e.clientX, 500));
  } else if (dragTarget.value === "bottom") {
    bottomPanelHeight.value = Math.max(100, Math.min(window.innerHeight - e.clientY, 400));
  }
};

const onMouseUp = () => {
  isDragging.value = false;
  dragTarget.value = null;
};

// Global mouse events
window.addEventListener("mousemove", onMouseMove);
window.addEventListener("mouseup", onMouseUp);
</script>

<template>
  <div class="ide-layout">
    <TitleBar />
    <div class="ide-main">
      <div class="ide-sidebar" :style="{ width: `${sidebarWidth}px` }">
        <Sidebar />
      </div>
      <div class="sidebar-resize" @mousedown="onMouseDown('sidebar', $event)" />
      <div class="ide-center">
        <div class="center-editor" :style="{ height: showBottomPanel ? `calc(100% - ${bottomPanelHeight}px)` : '100%' }">
          <EditorPanel />
        </div>
        <div v-if="showBottomPanel" class="bottom-resize" @mousedown="onMouseDown('bottom', $event)" />
        <div v-if="showBottomPanel" class="center-bottom" :style="{ height: `${bottomPanelHeight}px` }">
          <BottomPanel />
        </div>
      </div>
      <div v-if="showAIPanel" class="ai-resize" @mousedown="onMouseDown('ai', $event)" />
      <div v-if="showAIPanel" class="ide-ai" :style="{ width: `${aiPanelWidth}px` }">
        <AIPanel />
      </div>
    </div>
  </div>
</template>

<style scoped>
.ide-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}

.ide-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.ide-sidebar {
  flex-shrink: 0;
  overflow: hidden;
}

.sidebar-resize {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background 0.15s;
}

.sidebar-resize:hover {
  background: var(--accent);
}

.ide-center {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.center-editor {
  overflow: hidden;
}

.bottom-resize {
  height: 4px;
  cursor: row-resize;
  background: transparent;
  transition: background 0.15s;
}

.bottom-resize:hover {
  background: var(--accent);
}

.center-bottom {
  flex-shrink: 0;
  overflow: hidden;
}

.ai-resize {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background 0.15s;
}

.ai-resize:hover {
  background: var(--accent);
}

.ide-ai {
  flex-shrink: 0;
  overflow: hidden;
}
</style>