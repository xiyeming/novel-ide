<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useCloudStore } from "../../stores/cloud";

const store = useCloudStore();
const showCreateDialog = ref(false);
const newName = ref("");
const newProviderType = ref("webdav");
const newUrl = ref("");
const newUsername = ref("");
const newPassword = ref("");

const providerTypes = [
  { value: "webdav", label: "WebDAV" },
  { value: "oss", label: "阿里云 OSS" },
  { value: "s3", label: "AWS S3" },
];

onMounted(() => {
  store.fetchConfigs();
});

async function createConfig() {
  if (!newName.value || !newUrl.value) return;
  await store.createConfig(newName.value, newProviderType.value, {
    type: newProviderType.value,
    url: newUrl.value,
    username: newUsername.value,
    password: newPassword.value,
  });
  showCreateDialog.value = false;
  resetForm();
}

function resetForm() {
  newName.value = "";
  newProviderType.value = "webdav";
  newUrl.value = "";
  newUsername.value = "";
  newPassword.value = "";
}
</script>

<template>
  <div class="cloud-panel">
    <div class="panel-header">
      <h3>云同步</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 新建</button>
    </div>

    <div v-if="showCreateDialog" class="create-dialog">
      <input v-model="newName" placeholder="配置名称" class="input" />
      <select v-model="newProviderType" class="input">
        <option v-for="p in providerTypes" :key="p.value" :value="p.value">{{ p.label }}</option>
      </select>
      <input v-model="newUrl" placeholder="服务器地址" class="input" />
      <input v-model="newUsername" placeholder="用户名" class="input" />
      <input v-model="newPassword" type="password" placeholder="密码" class="input" />

      <div class="dialog-actions">
        <button class="btn-sm" @click="showCreateDialog = false; resetForm()">取消</button>
        <button class="btn-primary" @click="createConfig">创建</button>
      </div>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>

    <div v-else-if="store.configs.length === 0" class="empty">暂无云配置</div>

    <div v-else class="config-list">
      <div v-for="config in store.configs" :key="config.id" class="config-item">
        <div class="config-info">
          <div class="config-name">{{ config.name }}</div>
          <div class="config-type">{{ config.provider_type }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cloud-panel {
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
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.btn-sm {
  padding: 4px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.15s;
}

.btn-sm:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--accent);
}

.btn-primary {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  background: var(--accent);
  color: var(--bg-primary);
  cursor: pointer;
  font-size: 12px;
  transition: opacity 0.15s;
}

.btn-primary:hover {
  opacity: 0.9;
}

.input {
  width: 100%;
  padding: 6px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  margin-bottom: 8px;
  font-size: 13px;
  box-sizing: border-box;
  outline: none;
  transition: border-color 0.15s;
}

.input:focus {
  border-color: var(--accent);
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 8px;
}

.loading,
.empty {
  text-align: center;
  color: var(--text-muted);
  padding: var(--spacing-xl);
  font-size: var(--font-size-sm);
}

.config-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.config-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  transition: background 0.15s;
}

.config-item:hover {
  background: var(--bg-hover);
}

.config-name {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.config-type {
  font-size: 11px;
  color: var(--text-muted);
}
</style>
