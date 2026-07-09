<!-- src/components/layout/StatusBar.vue -->
<script setup lang="ts">
</script>

<template>
  <div class="status-bar">
    <div class="status-bar-left">
      <span class="status-item">🤖 Novel IDE</span>
    </div>
    <div class="status-bar-right">
      <span class="status-item">Ln 1, Col 1</span>
      <span class="status-item">UTF-8</span>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 24px;
  padding: 0 var(--spacing-2);
  background: var(--bg-background);
  border-top: 1px solid var(--border-default);
  font-size: var(--font-size-xs);
  color: var(--gray-500);
}

.status-bar-left,
.status-bar-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.status-item {
  white-space: nowrap;
  padding: 0 var(--spacing-1);
}
</style>
