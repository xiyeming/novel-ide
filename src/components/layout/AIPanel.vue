<!-- src/components/layout/AIPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";
import { useAIStore } from "../../stores/ai";

const aiStore = useAIStore();
const input = ref("");

const models = [
  { value: "deepseek-chat", label: "DeepSeek Chat" },
  { value: "gpt-4o", label: "GPT-4o" },
  { value: "claude-3.5-sonnet", label: "Claude 3.5 Sonnet" },
  { value: "glm-4", label: "GLM-4" },
  { value: "qwen-max", label: "Qwen Max" },
];

const sendMessage = () => {
  if (!input.value.trim()) return;
  aiStore.addMessage("user", input.value);
  input.value = "";
  // AI response will be added later
};

const clearChat = () => {
  aiStore.clearMessages();
};
</script>

<template>
  <div class="ai-panel">
    <div class="panel-header-sm">
      <span>AI 助手</span>
      <button class="clear-btn" @click="clearChat" title="清空对话">🗑</button>
    </div>
    <div class="model-selector">
      <label for="model-select">模型:</label>
      <select id="model-select" v-model="aiStore.selectedModel">
        <option v-for="model in models" :key="model.value" :value="model.value">
          {{ model.label }}
        </option>
      </select>
    </div>
    <div class="ai-messages">
      <div v-if="aiStore.messages.length === 0" class="empty-state">
        <p>开始与 AI 对话</p>
      </div>
      <div
        v-for="msg in aiStore.messages"
        :key="msg.id"
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
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border);
}

.clear-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 14px;
  padding: 2px 6px;
  border-radius: 4px;
}

.clear-btn:hover {
  background: var(--bg-surface);
}

.model-selector {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.model-selector label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.model-selector select {
  flex: 1;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
}

.model-selector select:focus {
  border-color: var(--accent);
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
