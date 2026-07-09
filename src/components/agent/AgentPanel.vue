<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAgentStore } from "../../stores/agent";

const store = useAgentStore();
const showCreateDialog = ref(false);
const newName = ref("");
const newRole = ref("writer");
const newSystemPrompt = ref("");
const newTemperature = ref(0.7);
const newMaxTokens = ref(2000);

const roles = [
  { value: "outline", label: "大纲助手" },
  { value: "writer", label: "初稿作家" },
  { value: "proofreader", label: "校对编辑" },
  { value: "editor", label: "终审编辑" },
  { value: "custom", label: "自定义" },
];

onMounted(() => {
  store.fetchAgents();
});

async function createAgent() {
  if (!newName.value || !newSystemPrompt.value) return;
  await store.createAgent({
    name: newName.value,
    role: newRole.value,
    systemPrompt: newSystemPrompt.value,
    temperature: newTemperature.value,
    maxTokens: newMaxTokens.value,
    knowledgeBaseIds: [],
  });
  resetForm();
  showCreateDialog.value = false;
}

function resetForm() {
  newName.value = "";
  newRole.value = "writer";
  newSystemPrompt.value = "";
  newTemperature.value = 0.7;
  newMaxTokens.value = 2000;
}

function getRoleLabel(role: string) {
  return roles.find((r) => r.value === role)?.label || role;
}
</script>

<template>
  <div class="agent-panel">
    <div class="panel-header">
      <h3>🤖 智能体</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 新建</button>
    </div>

    <div v-if="showCreateDialog" class="create-dialog">
      <input v-model="newName" placeholder="智能体名称" class="input" />
      <select v-model="newRole" class="input">
        <option v-for="r in roles" :key="r.value" :value="r.value">
          {{ r.label }}
        </option>
      </select>
      <textarea
        v-model="newSystemPrompt"
        placeholder="系统提示词..."
        class="input"
        rows="4"
      />

      <div class="param-row">
        <label>温度: {{ newTemperature }}</label>
        <input
          type="range"
          v-model.number="newTemperature"
          min="0"
          max="2"
          step="0.1"
        />
      </div>
      <div class="param-row">
        <label>最大Token: {{ newMaxTokens }}</label>
        <input
          type="range"
          v-model.number="newMaxTokens"
          min="100"
          max="8000"
          step="100"
        />
      </div>

      <div class="dialog-actions">
        <button class="btn-sm" @click="showCreateDialog = false; resetForm()">
          取消
        </button>
        <button class="btn-primary" @click="createAgent">创建</button>
      </div>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>

    <div v-else-if="store.agents.length === 0" class="empty">暂无智能体</div>

    <div v-else class="agent-list">
      <div
        v-for="agent in store.agents"
        :key="agent.id"
        class="agent-item"
      >
        <div class="agent-info">
          <div class="agent-name">{{ agent.name }}</div>
          <div class="agent-role">{{ getRoleLabel(agent.role) }}</div>
        </div>
        <button class="btn-icon danger" @click="store.deleteAgent(agent.id)">
          🗑
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.agent-panel {
  padding: var(--spacing-md);
  height: 100%;
  overflow-y: auto;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.panel-header h3 {
  margin: 0;
  font-size: var(--font-size-sm);
}

.btn-sm {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: var(--font-size-xs);
}

.btn-sm:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-primary {
  padding: var(--spacing-xs) var(--spacing-md);
  border: none;
  border-radius: 4px;
  background: var(--accent);
  color: white;
  cursor: pointer;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-icon {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: var(--spacing-xs);
}

.btn-icon.danger:hover {
  color: var(--error);
}

.input {
  width: 100%;
  padding: var(--spacing-xs) var(--spacing-sm);
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
  font-size: var(--font-size-xs);
  box-sizing: border-box;
}

.input:focus {
  outline: none;
  border-color: var(--accent);
}

.param-row {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  margin-bottom: var(--spacing-sm);
}

.param-row label {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.param-row input[type="range"] {
  width: 100%;
}

.dialog-actions {
  display: flex;
  gap: var(--spacing-sm);
  justify-content: flex-end;
  margin-top: var(--spacing-sm);
}

.loading,
.empty {
  text-align: center;
  color: var(--text-muted);
  padding: var(--spacing-xl);
  font-size: var(--font-size-xs);
}

.agent-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.agent-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  border: 1px solid var(--border);
  border-radius: 4px;
}

.agent-info {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.agent-name {
  font-size: var(--font-size-xs);
  color: var(--text-primary);
}

.agent-role {
  font-size: 11px;
  color: var(--text-muted);
}
</style>
