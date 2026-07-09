<script setup lang="ts">
import { ref, computed } from "vue";
import { useAIStore } from "../../stores/ai";
import { useProjectStore } from "../../stores/project";

const aiStore = useAIStore();
const projectStore = useProjectStore();

const activeSection = ref<'task' | 'workflow' | 'context' | 'execution' | 'conversation'>('task');
const input = ref("");

const sections = [
  { id: 'task', label: '任务', icon: '📋' },
  { id: 'workflow', label: '工作流', icon: '⚙️' },
  { id: 'context', label: '上下文', icon: '📚' },
  { id: 'execution', label: '执行', icon: '🚀' },
  { id: 'conversation', label: '对话', icon: '💬' },
];

const currentProject = computed(() => projectStore.currentProject);
const hasProject = computed(() => !!currentProject.value);

const sendMessage = () => {
  if (!input.value.trim() || aiStore.streaming) return;
  aiStore.sendMessage(input.value);
  input.value = "";
};
</script>

<template>
  <div class="ai-studio">
    <div class="studio-header">
      <h3>AI 工作室</h3>
    </div>

    <div class="studio-sections">
      <div
        v-for="section in sections"
        :key="section.id"
        :class="['section-header', { active: activeSection === section.id }]"
        @click="activeSection = section.id as any"
      >
        <span class="section-icon">{{ section.icon }}</span>
        <span class="section-label">{{ section.label }}</span>
        <span class="section-arrow">{{ activeSection === section.id ? '▼' : '▶' }}</span>
      </div>

      <!-- Task Section -->
      <div v-if="activeSection === 'task'" class="section-content">
        <div v-if="hasProject" class="task-info">
          <div class="task-label">当前项目</div>
          <div class="task-name">{{ currentProject?.name }}</div>
          <div class="task-hint">选择章节开始写作</div>
        </div>
        <div v-else class="empty-state">
          <div class="empty-icon">📝</div>
          <div class="empty-text">请先打开或创建一个项目</div>
        </div>
      </div>

      <!-- Workflow Section -->
      <div v-if="activeSection === 'workflow'" class="section-content">
        <div v-if="hasProject" class="workflow-steps">
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">① 项目配置</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">② 故事前提</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">③ 世界观</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">④ 角色</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">⑤ 蓝图</span>
          </div>
        </div>
        <div v-else class="empty-state">
          <div class="empty-icon">⚙️</div>
          <div class="empty-text">请先打开一个项目</div>
        </div>
      </div>

      <!-- Context Section -->
      <div v-if="activeSection === 'context'" class="section-content">
        <div v-if="hasProject" class="context-items">
          <div class="context-item">
            <span class="context-check">○</span>
            <span class="context-label">世界观</span>
          </div>
          <div class="context-item">
            <span class="context-check">○</span>
            <span class="context-label">角色</span>
          </div>
          <div class="context-item">
            <span class="context-check">○</span>
            <span class="context-label">已写章节</span>
          </div>
          <div class="context-item">
            <span class="context-check">○</span>
            <span class="context-label">知识库</span>
          </div>
          <div class="context-item">
            <span class="context-check">○</span>
            <span class="context-label">提示词</span>
          </div>
        </div>
        <div v-else class="empty-state">
          <div class="empty-icon">📚</div>
          <div class="empty-text">请先打开一个项目</div>
        </div>
      </div>

      <!-- Execution Section -->
      <div v-if="activeSection === 'execution'" class="section-content">
        <div class="execution-items">
          <div class="execution-item">
            <span class="execution-label">智能体:</span>
            <span class="execution-value">{{ aiStore.selectedModel || '未选择' }}</span>
          </div>
          <div class="execution-item">
            <span class="execution-label">状态:</span>
            <span class="execution-value">{{ aiStore.streaming ? '生成中...' : '空闲' }}</span>
          </div>
        </div>
      </div>

      <!-- Conversation Section -->
      <div v-if="activeSection === 'conversation'" class="section-content conversation-section">
        <div class="conversation-messages">
          <div v-if="aiStore.messages.length === 0" class="empty-state">
            <div class="empty-icon">💬</div>
            <div class="empty-text">暂无对话</div>
          </div>
          <div
            v-for="msg in aiStore.messages"
            :key="msg.id"
            :class="['message', msg.role]"
          >
            <div class="message-content">{{ msg.content }}</div>
          </div>
        </div>
        <div class="conversation-input">
          <input
            v-model="input"
            type="text"
            placeholder="输入消息..."
            :disabled="aiStore.streaming"
            @keydown.enter="sendMessage"
          />
          <button @click="sendMessage" :disabled="aiStore.streaming">发送</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ai-studio {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-left: 1px solid var(--border-default);
}

.studio-header {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
}

.studio-header h3 {
  margin: 0;
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
}

.studio-sections {
  flex: 1;
  overflow-y: auto;
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-2) var(--spacing-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-out);
}

.section-header:hover {
  background: var(--bg-hover);
}

.section-header.active {
  background: var(--bg-active);
}

.section-icon {
  font-size: 14px;
}

.section-label {
  flex: 1;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
}

.section-arrow {
  font-size: 10px;
  color: var(--text-secondary);
}

.section-content {
  padding: var(--spacing-3);
  border-top: 1px solid var(--border-divider);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-6) 0;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 32px;
  margin-bottom: var(--spacing-2);
}

.empty-text {
  font-size: var(--font-size-sm);
}

/* Task Styles */
.task-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.task-label {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.task-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
}

.task-hint {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* Workflow Styles */
.workflow-steps {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.workflow-step {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-1) var(--spacing-2);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.step-icon {
  width: 16px;
  text-align: center;
}

/* Context Styles */
.context-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.context-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-1) var(--spacing-2);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

/* Execution Styles */
.execution-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.execution-item {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-sm);
}

.execution-label {
  color: var(--text-secondary);
}

.execution-value {
  color: var(--text-primary);
}

/* Conversation Styles */
.conversation-section {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.conversation-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-3);
}

.message {
  margin-bottom: var(--spacing-3);
}

.message.user .message-content {
  background: var(--blue-500);
  color: white;
  margin-left: 40px;
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-md);
}

.message.assistant .message-content {
  background: var(--bg-card);
  color: var(--text-primary);
  margin-right: 40px;
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-md);
}

.conversation-input {
  display: flex;
  gap: var(--spacing-2);
  padding: var(--spacing-3);
  border-top: 1px solid var(--border-default);
}

.conversation-input input {
  flex: 1;
  background: var(--bg-card);
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.conversation-input input:focus {
  border-color: var(--blue-500);
  outline: none;
}

.conversation-input button {
  background: var(--blue-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  cursor: pointer;
}

.conversation-input button:hover {
  background: var(--blue-600);
}
</style>
