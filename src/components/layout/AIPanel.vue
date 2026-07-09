<!-- src/components/layout/AIPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";
function escapeHtml(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}
import { useAIStore } from "../../stores/ai";
import AIWritePanel from "../ai/AIWritePanel.vue";

const aiStore = useAIStore();
const input = ref("");
const activeTab = ref<"chat" | "write">("chat");

const models = [
  { value: "deepseek-chat", label: "DeepSeek Chat" },
  { value: "gpt-4o", label: "GPT-4o" },
  { value: "claude-3.5-sonnet", label: "Claude 3.5 Sonnet" },
  { value: "glm-4", label: "GLM-4" },
  { value: "qwen-max", label: "Qwen Max" },
];

function renderMarkdown(text: string): string {
  // Escape HTML as the FIRST step (Critical 1: XSS prevention)
  let html = escapeHtml(text);
  // Code blocks (``` ... ```)
  html = html.replace(/```(\w*)\n([\s\S]*?)```/g, (_m, lang: string, code: string) => {
    return `<pre class="md-code-block"><code class="language-${lang}">${code}</code></pre>`;
  });
  // Inline code (Important 5: content is already escaped above)
  html = html.replace(/`([^`\n]+)`/g, '<code class="md-inline-code">$1</code>');
  // Headers
  html = html.replace(/^### (.+)$/gm, "<h3>$1</h3>");
  html = html.replace(/^## (.+)$/gm, "<h2>$1</h2>");
  html = html.replace(/^# (.+)$/gm, "<h1>$1</h1>");
  // Bold + italic
  html = html.replace(/\*\*\*(.+?)\*\*\*/g, "<strong><em>$1</em></strong>");
  html = html.replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>");
  html = html.replace(/\*(.+?)\*/g, "<em>$1</em>");
  // Links
  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
  // Unordered lists
  html = html.replace(/^[*-] (.+)$/gm, "<li>$1</li>");
  html = html.replace(/(<li>.*<\/li>\n?)+/g, (m) => `<ul>${m}</ul>`);
  // Ordered lists (Important 4: wrap consecutive <li> in <ol>)
  html = html.replace(/^\d+\. (.+)$/gm, "<li>$1</li>");
  html = html.replace(/(<li>.*<\/li>\n?)+/g, (m) => {
    if (m.includes("<ul>")) return m;
    return `<ol>${m}</ol>`;
  });
  // Paragraphs: double newline → paragraph break
  html = html.replace(/\n{2,}/g, "</p><p>");
  // Single newlines → <br>
  html = html.replace(/\n/g, "<br>");
  // Wrap in <p> if not already block-level
  if (!/^<(h[1-6]|pre|ul|ol|p)/.test(html)) {
    html = `<p>${html}</p>`;
  }
  return html;
}

const sendMessage = () => {
  if (!input.value.trim() || aiStore.streaming) return;
  const text = input.value;
  input.value = "";
  aiStore.sendMessage(text);
};
</script>

<template>
  <div class="ai-panel">
    <div class="panel-tabs">
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'chat' }"
        @click="activeTab = 'chat'"
      >
        AI 对话
      </button>
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'write' }"
        @click="activeTab = 'write'"
      >
        AI 写作
      </button>
    </div>

    <template v-if="activeTab === 'chat'">
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
          <div
            v-if="msg.role === 'assistant'"
            class="message-content md-rendered"
            v-html="renderMarkdown(msg.content)"
          ></div>
          <div v-else class="message-content">{{ msg.content }}</div>
        </div>
      </div>
      <div v-if="aiStore.streaming" class="streaming-indicator">
        <span class="dot"></span><span class="dot"></span><span class="dot"></span>
      </div>
      <div class="ai-input">
        <input
          v-model="input"
          type="text"
          placeholder="输入消息..."
          :disabled="aiStore.streaming"
          @keydown.enter="sendMessage"
        />
        <button @click="sendMessage" :disabled="aiStore.streaming">发送</button>
      </div>
    </template>

    <template v-else>
      <AIWritePanel />
    </template>
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

.panel-tabs {
  display: flex;
  border-bottom: 1px solid var(--border);
}

.tab-btn {
  flex: 1;
  padding: 10px 8px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn:hover {
  color: var(--text-primary);
  background: var(--bg-surface);
}

.tab-btn.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
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

.md-rendered :deep(h1),
.md-rendered :deep(h2),
.md-rendered :deep(h3) {
  margin: 0.5em 0 0.25em;
  font-weight: 600;
}
.md-rendered :deep(h1) { font-size: 1.2em; }
.md-rendered :deep(h2) { font-size: 1.1em; }
.md-rendered :deep(h3) { font-size: 1em; }

.md-rendered :deep(pre) {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: var(--spacing-sm);
  overflow-x: auto;
  font-size: var(--font-size-sm);
}
.md-rendered :deep(code.md-inline-code) {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 3px;
  padding: 0.1em 0.3em;
  font-size: 0.9em;
}
.md-rendered :deep(p) {
  margin: 0.4em 0;
}
.md-rendered :deep(ul) {
  margin: 0.4em 0;
  padding-left: 1.5em;
}
.md-rendered :deep(li) {
  margin: 0.2em 0;
}
.md-rendered :deep(strong) {
  font-weight: 600;
}
.md-rendered :deep(a) {
  color: var(--accent);
  text-decoration: none;
}
.md-rendered :deep(a:hover) {
  text-decoration: underline;
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

.ai-input input:disabled,
.ai-input button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.streaming-indicator {
  display: flex;
  gap: 4px;
  padding: var(--spacing-sm) var(--spacing-md);
  border-top: 1px solid var(--border);
}

.streaming-indicator .dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--text-muted);
  animation: blink 1.4s infinite both;
}

.streaming-indicator .dot:nth-child(2) {
  animation-delay: 0.2s;
}

.streaming-indicator .dot:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes blink {
  0%, 80%, 100% { opacity: 0.3; }
  40% { opacity: 1; }
}
</style>
