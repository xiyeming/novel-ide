<!-- src/components/common/Panel.vue -->
<script setup lang="ts">
defineProps<{
  title?: string;
  width?: number;
  minWidth?: number;
}>();
</script>

<template>
  <div class="panel" :style="{ width: width ? `${width}px` : undefined }">
    <div v-if="title" class="panel-header">
      <span class="panel-title">{{ title }}</span>
    </div>
    <div class="panel-content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 var(--spacing-md);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  user-select: none;
}

.panel-title {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.panel-content {
  flex: 1;
  overflow: auto;
}
</style>