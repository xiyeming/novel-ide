<script setup lang="ts">
</script>

<template>
  <div class="ide-layout">
    <div class="sidebar">
      <h2>Novel IDE</h2>
    </div>
    <div class="editor-area">
      <p>Welcome to Novel IDE</p>
    </div>
  </div>
</template>

<style scoped>
.ide-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.sidebar {
  width: 250px;
  background-color: var(--color-sidebar);
  border-right: 1px solid var(--color-border);
  padding: 16px;
}

.editor-area {
  flex: 1;
  background-color: var(--color-editor);
  padding: 16px;
}
</style>
