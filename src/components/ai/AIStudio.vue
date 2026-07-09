<!-- src/components/ai/AIStudio.vue -->
<script setup lang="ts">
</script>

<template>
  <div class="ai-studio">
    <div class="ai-studio-header">AI Studio</div>
    <div class="ai-studio-content">
      <div class="section">
        <div class="section-header">Task</div>
      </div>
      <div class="section">
        <div class="section-header">Workflow</div>
      </div>
      <div class="section">
        <div class="section-header">Context</div>
      </div>
      <div class="section">
        <div class="section-header">Execution</div>
      </div>
      <div class="section">
        <div class="section-header">Conversation</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ai-studio {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-panel);
}

.ai-studio-header {
  display: flex;
  align-items: center;
  height: 36px;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
  color: var(--gray-100);
}

.ai-studio-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-3);
}

.section {
  margin-bottom: var(--spacing-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}

.section-header {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 var(--spacing-3);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--gray-400);
  cursor: pointer;
}
</style>
