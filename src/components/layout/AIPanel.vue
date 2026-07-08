<!-- src/components/layout/AIPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const messages = ref<{ role: "user" | "assistant"; content: string }[]>([]);
const input = ref("");

const sendMessage = () => {
  if (!input.value.trim()) return;
  messages.value.push({ role: "user", content: input.value });
  input.value = "";
  // AI response will be added later
};
</script>

<template>
  <div class="ai-panel">
    <div class="panel-header-sm">AI 助手</div>
    <div class="ai-messages">
      <div v-if="messages.length === 0" class="empty-state">
        <p>开始与 AI 对话</p>
      </div>
      <div
        v-for="(msg, i) in messages"
        :key="i"
        :class="['message', msg.role]"
      >
        <div class="message-content">{{ msg.content }}</div>
      </div>
    </div>
    <div class="ai-input">
      <input
        v-model="input"
        type="text"
        placeholder="输入消息..."
        @keydown.enter="sendMessage"
      />
      <button @click="sendMessage">发送</button>
    </div>
  </div>
</template>

<style scoped>
.ai-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border);
}

.panel-header-sm {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border);
}

.ai-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
}

.message {
  margin-bottom: var(--spacing-md);
}

.message.user .message-content {
  background: var(--accent);
  color: var(--bg-primary);
  margin-left: 40px;
}

.message.assistant .message-content {
  background: var(--bg-surface);
  color: var(--text-primary);
  margin-right: 40px;
}

.message-content {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 8px;
  font-size: var(--font-size-md);
  line-height: 1.5;
  word-wrap: break-word;
}

.ai-input {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  border-top: 1px solid var(--border);
}

.ai-input input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  outline: none;
}

.ai-input input:focus {
  border-color: var(--accent);
}

.ai-input button {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.ai-input button:hover {
  opacity: 0.9;
}
</style>