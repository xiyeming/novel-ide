<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "../../stores/settings";

const settingsStore = useSettingsStore();

const editorFont = ref("JetBrains Mono");
const editorFontSize = ref(14);
const editorLineHeight = ref(1.6);
const autoSaveInterval = ref(30);

const defaultModelId = ref("");
const aiTemperature = ref(0.7);
const aiMaxTokens = ref(2000);
const aiSystemPrompt = ref("你是一个专业的中文小说写作助手。");

const defaultExportFormat = ref("txt");
const exportPath = ref("");
const exportIncludeMetadata = ref(true);

const saving = ref(false);

onMounted(async () => {
  await settingsStore.fetchSettings();
  loadSettings();
});

const loadSettings = () => {
  editorFont.value = settingsStore.getSetting("editor_font") || "JetBrains Mono";
  editorFontSize.value = parseInt(settingsStore.getSetting("editor_font_size") || "14");
  editorLineHeight.value = parseFloat(settingsStore.getSetting("editor_line_height") || "1.6");
  autoSaveInterval.value = parseInt(settingsStore.getSetting("auto_save_interval") || "30");

  defaultModelId.value = settingsStore.getSetting("default_model_id") || "";
  aiTemperature.value = parseFloat(settingsStore.getSetting("ai_temperature") || "0.7");
  aiMaxTokens.value = parseInt(settingsStore.getSetting("ai_max_tokens") || "2000");
  aiSystemPrompt.value = settingsStore.getSetting("ai_system_prompt") || "你是一个专业的中文小说写作助手。";

  defaultExportFormat.value = settingsStore.getSetting("default_export_format") || "txt";
  exportPath.value = settingsStore.getSetting("export_path") || "";
  exportIncludeMetadata.value = settingsStore.getSetting("export_include_metadata") !== "false";
};

const saveSettings = async () => {
  saving.value = true;
  try {
    await settingsStore.updateSetting("editor_font", editorFont.value);
    await settingsStore.updateSetting("editor_font_size", editorFontSize.value.toString());
    await settingsStore.updateSetting("editor_line_height", editorLineHeight.value.toString());
    await settingsStore.updateSetting("auto_save_interval", autoSaveInterval.value.toString());

    await settingsStore.updateSetting("default_model_id", defaultModelId.value);
    await settingsStore.updateSetting("ai_temperature", aiTemperature.value.toString());
    await settingsStore.updateSetting("ai_max_tokens", aiMaxTokens.value.toString());
    await settingsStore.updateSetting("ai_system_prompt", aiSystemPrompt.value);

    await settingsStore.updateSetting("default_export_format", defaultExportFormat.value);
    await settingsStore.updateSetting("export_path", exportPath.value);
    await settingsStore.updateSetting("export_include_metadata", exportIncludeMetadata.value.toString());
  } finally {
    saving.value = false;
  }
};
</script>

<template>
  <div class="settings-panel">
    <div class="panel-header">设置</div>

    <div class="settings-section">
      <h3>编辑器设置</h3>
      <div class="setting-item">
        <label>字体</label>
        <select v-model="editorFont">
          <option value="JetBrains Mono">JetBrains Mono</option>
          <option value="Fira Code">Fira Code</option>
          <option value="Source Code Pro">Source Code Pro</option>
          <option value="Monaco">Monaco</option>
        </select>
      </div>
      <div class="setting-item">
        <label>字号 (px)</label>
        <input type="number" v-model="editorFontSize" min="10" max="24" />
      </div>
      <div class="setting-item">
        <label>行高</label>
        <input type="number" v-model="editorLineHeight" min="1" max="2" step="0.1" />
      </div>
      <div class="setting-item">
        <label>自动保存间隔 (秒)</label>
        <input type="number" v-model="autoSaveInterval" min="10" max="300" />
      </div>
    </div>

    <div class="settings-section">
      <h3>AI 设置</h3>
      <div class="setting-item">
        <label>默认模型</label>
        <input type="text" v-model="defaultModelId" placeholder="模型 ID" />
      </div>
      <div class="setting-item">
        <label>温度 (0-2)</label>
        <input type="number" v-model="aiTemperature" min="0" max="2" step="0.1" />
      </div>
      <div class="setting-item">
        <label>最大 Token</label>
        <input type="number" v-model="aiMaxTokens" min="100" max="8000" step="100" />
      </div>
      <div class="setting-item">
        <label>系统提示词</label>
        <textarea v-model="aiSystemPrompt" rows="3"></textarea>
      </div>
    </div>

    <div class="settings-section">
      <h3>导出设置</h3>
      <div class="setting-item">
        <label>默认格式</label>
        <select v-model="defaultExportFormat">
          <option value="txt">TXT</option>
          <option value="md">Markdown</option>
          <option value="docx">DOCX</option>
          <option value="pdf">PDF</option>
          <option value="epub">EPUB</option>
        </select>
      </div>
      <div class="setting-item">
        <label>导出路径</label>
        <input type="text" v-model="exportPath" placeholder="/path/to/exports" />
      </div>
      <div class="setting-item">
        <label>
          <input type="checkbox" v-model="exportIncludeMetadata" />
          包含元数据
        </label>
      </div>
    </div>

    <div class="settings-actions">
      <button @click="saveSettings" :disabled="saving">
        {{ saving ? "保存中..." : "保存设置" }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  padding: var(--spacing-md);
}

.panel-header {
  font-size: var(--font-size-lg);
  font-weight: 600;
  margin-bottom: var(--spacing-lg);
}

.settings-section {
  margin-bottom: var(--spacing-xl);
  padding: var(--spacing-md);
  background: var(--bg-secondary);
  border-radius: 8px;
}

.settings-section h3 {
  font-size: var(--font-size-md);
  margin-bottom: var(--spacing-md);
  color: var(--text-primary);
}

.setting-item {
  display: flex;
  align-items: center;
  margin-bottom: var(--spacing-sm);
}

.setting-item label {
  width: 120px;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.setting-item input,
.setting-item select,
.setting-item textarea {
  flex: 1;
  padding: var(--spacing-sm);
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.setting-item textarea {
  resize: vertical;
  min-height: 60px;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
}

.settings-actions button {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.settings-actions button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
