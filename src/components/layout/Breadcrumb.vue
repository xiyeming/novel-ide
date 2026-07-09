<!-- src/components/layout/Breadcrumb.vue -->
<script setup lang="ts">
defineProps<{
  items: string[];
}>();
</script>

<template>
  <div class="breadcrumb">
    <template v-for="(item, index) in items" :key="index">
      <span class="breadcrumb-item">{{ item }}</span>
      <span v-if="index < items.length - 1" class="breadcrumb-separator">/</span>
    </template>
  </div>
</template>

<style scoped>
.breadcrumb {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 var(--spacing-3);
  background: var(--bg-background);
  border-bottom: 1px solid var(--border-default);
  font-size: var(--font-size-sm);
  color: var(--gray-500);
  overflow: hidden;
}

.breadcrumb-item {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.breadcrumb-separator {
  margin: 0 var(--spacing-1);
  color: var(--gray-600);
}
</style>
