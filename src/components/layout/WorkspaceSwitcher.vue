<script setup lang="ts">
import { useWorkspaceStore } from "../../stores/workspace";

const workspaceStore = useWorkspaceStore();
</script>

<template>
  <div class="workspace-switcher">
    <button
      v-for="preset in workspaceStore.presets"
      :key="preset.id"
      :class="['preset-btn', { active: workspaceStore.activePresetId === preset.id }]"
      @click="workspaceStore.setActivePreset(preset.id)"
      :title="preset.name"
    >
      <span class="preset-icon">{{ preset.icon }}</span>
      <span class="preset-name">{{ preset.name }}</span>
    </button>
  </div>
</template>

<style scoped>
.workspace-switcher {
  display: flex;
  gap: var(--spacing-1);
  padding: var(--spacing-2);
  background: var(--bg-sidebar);
  border-bottom: 1px solid var(--border-default);
}

.preset-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-2);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.preset-btn:hover {
  background: var(--bg-hover);
}

.preset-btn.active {
  background: var(--blue-500);
  color: white;
}

.preset-icon {
  font-size: 14px;
}

.preset-name {
  font-size: var(--font-size-xs);
}
</style>