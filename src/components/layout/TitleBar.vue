<!-- src/components/layout/TitleBar.vue -->
<script setup lang="ts">
import { ref } from "vue";

const isMaximized = ref(false);

const minimize = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  getCurrentWindow().minimize();
};

const toggleMaximize = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  const win = getCurrentWindow();
  if (isMaximized.value) {
    await win.unmaximize();
  } else {
    await win.maximize();
  }
  isMaximized.value = !isMaximized.value;
};

const close = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  getCurrentWindow().close();
};
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-title" data-tauri-drag-region>
      <span class="app-icon">📖</span>
      <span>Novel IDE</span>
    </div>
    <div class="titlebar-controls">
      <button class="titlebar-btn" @click="minimize">
        <svg width="12" height="12" viewBox="0 0 12 12"><rect y="5" width="12" height="1" fill="currentColor" /></svg>
      </button>
      <button class="titlebar-btn" @click="toggleMaximize">
        <svg v-if="!isMaximized" width="12" height="12" viewBox="0 0 12 12"><rect x="1" y="1" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1" /></svg>
        <svg v-else width="12" height="12" viewBox="0 0 12 12"><rect x="3" y="3" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1" /><rect y="0" width="4" height="4" fill="none" stroke="currentColor" stroke-width="1" /></svg>
      </button>
      <button class="titlebar-btn titlebar-btn-close" @click="close">
        <svg width="12" height="12" viewBox="0 0 12 12"><line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" /><line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" /></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: var(--titlebar-height);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  user-select: none;
}

.titlebar-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding-left: var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.app-icon {
  font-size: 16px;
}

.titlebar-controls {
  display: flex;
  height: 100%;
}

.titlebar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 100%;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s;
}

.titlebar-btn:hover {
  background: var(--bg-hover);
}

.titlebar-btn-close:hover {
  background: var(--error);
  color: white;
}
</style>