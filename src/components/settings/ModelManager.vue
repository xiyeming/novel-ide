<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useModelStore, type ModelProvider } from "../../stores/model";

const modelStore = useModelStore();

const providerTypes = [
  { value: "openai", label: "OpenAI" },
  { value: "anthropic", label: "Anthropic" },
  { value: "deepseek", label: "DeepSeek" },
  { value: "zhipu", label: "智谱" },
  { value: "moonshot", label: "Moonshot" },
  { value: "custom", label: "自定义" },
];

const showDialog = ref(false);
const editingId = ref<string | null>(null);
const form = ref({
  name: "",
  provider_type: "openai",
  api_url: "",
  api_key: "",
  model_name: "",
  is_default: false,
});
const testStatus = ref<{ id: string; success: boolean; message: string } | null>(null);
const testLoading = ref<string | null>(null);

onMounted(() => {
  modelStore.fetchProviders();
});

const openAddDialog = () => {
  editingId.value = null;
  form.value = {
    name: "",
    provider_type: "openai",
    api_url: "",
    api_key: "",
    model_name: "",
    is_default: false,
  };
  showDialog.value = true;
};

const openEditDialog = (provider: ModelProvider) => {
  editingId.value = provider.id;
  form.value = {
    name: provider.name,
    provider_type: provider.provider_type,
    api_url: provider.api_url,
    api_key: provider.api_key || "",
    model_name: provider.model_name,
    is_default: provider.is_default,
  };
  showDialog.value = true;
};

const handleSave = async () => {
  if (!form.value.name.trim() || !form.value.model_name.trim()) return;
  if (editingId.value) {
    await modelStore.updateProvider(editingId.value, { ...form.value });
  } else {
    await modelStore.createProvider({ ...form.value });
  }
  showDialog.value = false;
};

const handleDelete = async (id: string) => {
  await modelStore.deleteProvider(id);
};

const handleSetDefault = async (provider: ModelProvider) => {
  await modelStore.updateProvider(provider.id, { is_default: true });
};

const handleTestConnection = async (provider: ModelProvider) => {
  testLoading.value = provider.id;
  testStatus.value = null;
  try {
    const result = await modelStore.testConnection(provider.id);
    testStatus.value = { id: provider.id, ...result };
  } catch {
    testStatus.value = { id: provider.id, success: false, message: "连接失败" };
  } finally {
    testLoading.value = null;
  }
};
</script>

<template>
  <div class="model-manager">
    <div class="model-header">
      <span class="panel-header-sm">模型配置</span>
      <button class="add-btn" @click="openAddDialog">+ 添加</button>
    </div>

    <div v-if="modelStore.loading" class="empty-state">加载中…</div>
    <div v-else-if="modelStore.providers.length === 0" class="empty-state">
      暂无模型配置，点击上方按钮添加
    </div>
    <div v-else class="model-list">
      <div
        v-for="provider in modelStore.providers"
        :key="provider.id"
        class="model-card"
      >
        <div class="model-card-header">
          <div class="model-info">
            <span class="model-name">{{ provider.name }}</span>
            <span v-if="provider.is_default" class="default-badge">默认</span>
          </div>
          <div class="model-actions">
            <button
              class="action-btn"
              @click="handleTestConnection(provider)"
              :disabled="testLoading === provider.id"
              title="测试连接"
            >
              {{ testLoading === provider.id ? "…" : "🔌" }}
            </button>
            <button class="action-btn" @click="openEditDialog(provider)" title="编辑">
              ✏️
            </button>
            <button class="action-btn danger" @click="handleDelete(provider.id)" title="删除">
              🗑️
            </button>
          </div>
        </div>
        <div class="model-details">
          <div class="detail-row">
            <span class="detail-label">类型</span>
            <span>{{ provider.provider_type }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">模型</span>
            <span>{{ provider.model_name }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">地址</span>
            <span class="api-url">{{ provider.api_url }}</span>
          </div>
        </div>
        <div v-if="testStatus && testStatus.id === provider.id" class="test-result" :class="testStatus.success ? 'success' : 'error'">
          {{ testStatus.message }}
        </div>
        <div v-if="!provider.is_default" class="model-footer">
          <button class="set-default-btn" @click="handleSetDefault(provider)">设为默认</button>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <div v-if="showDialog" class="dialog-overlay" @click.self="showDialog = false">
        <div class="dialog">
          <div class="dialog-title">{{ editingId ? "编辑模型" : "添加模型" }}</div>
          <div class="dialog-form">
            <div class="form-group">
              <label>名称</label>
              <input v-model="form.name" type="text" placeholder="如：我的 GPT-4" />
            </div>
            <div class="form-group">
              <label>提供方</label>
              <select v-model="form.provider_type">
                <option v-for="t in providerTypes" :key="t.value" :value="t.value">
                  {{ t.label }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label>API 地址</label>
              <input v-model="form.api_url" type="text" placeholder="https://api.openai.com/v1" />
            </div>
            <div class="form-group">
              <label>API Key</label>
              <input v-model="form.api_key" type="password" placeholder="sk-…" />
            </div>
            <div class="form-group">
              <label>模型名称</label>
              <input v-model="form.model_name" type="text" placeholder="如：gpt-4" />
            </div>
            <div class="form-group checkbox-group">
              <label>
                <input type="checkbox" v-model="form.is_default" />
                设为默认模型
              </label>
            </div>
          </div>
          <div class="dialog-actions">
            <button class="cancel-btn" @click="showDialog = false">取消</button>
            <button class="save-btn" @click="handleSave" :disabled="!form.name.trim() || !form.model_name.trim()">
              保存
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.model-manager {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.model-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header-sm {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.add-btn {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 4px;
  font-size: var(--font-size-sm);
  cursor: pointer;
}

.add-btn:hover {
  opacity: 0.9;
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
  text-align: center;
  padding: var(--spacing-xl);
}

.model-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.model-card {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: var(--spacing-md);
}

.model-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-sm);
}

.model-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.model-name {
  font-weight: 600;
  font-size: var(--font-size-sm);
}

.default-badge {
  background: var(--accent);
  color: var(--bg-primary);
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 3px;
}

.model-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.action-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 14px;
  padding: 2px 4px;
  border-radius: 3px;
  transition: background 0.15s;
}

.action-btn:hover {
  background: var(--bg-hover);
}

.action-btn.danger:hover {
  background: var(--error);
  color: white;
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.model-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  font-size: var(--font-size-sm);
}

.detail-row {
  display: flex;
  gap: var(--spacing-sm);
}

.detail-label {
  color: var(--text-muted);
  min-width: 40px;
}

.api-url {
  color: var(--text-muted);
  font-family: monospace;
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.test-result {
  margin-top: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: 4px;
  font-size: var(--font-size-sm);
}

.test-result.success {
  background: rgba(34, 197, 94, 0.15);
  color: #22c55e;
}

.test-result.error {
  background: rgba(239, 68, 68, 0.15);
  color: #ef4444;
}

.model-footer {
  margin-top: var(--spacing-sm);
}

.set-default-btn {
  background: none;
  border: 1px solid var(--border);
  color: var(--text-secondary);
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 3px;
  cursor: pointer;
}

.set-default-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: var(--spacing-lg);
  width: 420px;
  max-width: 90vw;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: var(--spacing-md);
}

.dialog-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-group label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.form-group input,
.form-group select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
  transition: border-color 0.15s;
}

.form-group input:focus,
.form-group select:focus {
  border-color: var(--accent);
}

.checkbox-group label {
  flex-direction: row;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
}

.checkbox-group input[type="checkbox"] {
  width: auto;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-lg);
}

.cancel-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  cursor: pointer;
  font-size: var(--font-size-sm);
}

.cancel-btn:hover {
  background: var(--bg-hover);
}

.save-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: var(--font-size-sm);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.save-btn:not(:disabled):hover {
  opacity: 0.9;
}
</style>
