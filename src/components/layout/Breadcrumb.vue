<script setup lang="ts">
defineProps<{
  items: Array<{ label: string }>;
}>();
</script>

<template>
  <div class="breadcrumb">
    <span
      v-for="(item, index) in items"
      :key="index"
      class="breadcrumb-item"
    >
      <span v-if="index > 0" class="breadcrumb-separator">/</span>
      <span class="breadcrumb-label">{{ item.label }}</span>
    </span>
  </div>
</template>

<style scoped>
.breadcrumb {
  display: flex;
  align-items: center;
  height: 24px;
  padding: 0 var(--spacing-3);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.breadcrumb-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.breadcrumb-separator {
  color: var(--gray-500);
}

.breadcrumb-label {
  cursor: pointer;
}

.breadcrumb-label:hover {
  color: var(--text-primary);
}
</style>
