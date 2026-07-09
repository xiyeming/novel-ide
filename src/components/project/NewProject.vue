<script setup lang="ts">
import { ref, computed } from "vue";
import { useProjectStore } from "../../stores/project";
import CustomSelect from "../common/CustomSelect.vue";
import NumberInput from "../common/NumberInput.vue";

const emit = defineEmits<{ close: [] }>();
const store = useProjectStore();
const errorMsg = ref("");

const form = ref({
  name: "",
  parentPath: "",
  description: "",
  genre: "",
  sub_genre: "",
  target_readers: "",
  total_chapters: 300,
  words_per_chapter: 3000,
  narrative_pov: "第三人称有限",
  story_structure: "三幕式",
});

const genres = [
  { label: "请选择", value: "" },
  { label: "仙侠", value: "仙侠" },
  { label: "玄幻", value: "玄幻" },
  { label: "都市", value: "都市" },
  { label: "科幻", value: "科幻" },
  { label: "历史", value: "历史" },
  { label: "悬疑", value: "悬疑" },
  { label: "言情", value: "言情" },
  { label: "武侠", value: "武侠" },
  { label: "奇幻", value: "奇幻" },
  { label: "其他", value: "其他" },
];

const povs = [
  { label: "第一人称", value: "第一人称" },
  { label: "第二人称", value: "第二人称" },
  { label: "第三人称有限", value: "第三人称有限" },
  { label: "第三人称全知", value: "第三人称全知" },
  { label: "多视角", value: "多视角" },
];

const structures = [
  { label: "三幕式", value: "三幕式" },
  { label: "英雄之旅", value: "英雄之旅" },
  { label: "起承转合", value: "起承转合" },
  { label: "非线性", value: "非线性" },
  { label: "自由结构", value: "自由结构" },
];

const submitting = ref(false);

// Computed full path: parentPath/projectName
const fullPath = computed(() => {
  if (!form.value.parentPath || !form.value.name) return "";
  const parent = form.value.parentPath.replace(/\/$/, "");
  return `${parent}/${form.value.name}`;
});

const selectPath = async () => {
  // Try to import Tauri dialog - will fail in browser
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({ directory: true, title: "选择项目父目录" });
    if (selected) {
      form.value.parentPath = selected as string;
    }
  } catch (err) {
    // Browser or Tauri not available
    console.error("Failed to open directory dialog:", err);
    alert("目录选择功能需要在桌面端使用。请手动输入路径。");
  }
};

const submit = async () => {
  if (!form.value.name || !form.value.parentPath) return;
  submitting.value = true;
  errorMsg.value = "";
  try {
    // Debug: log form data before sending
    const projectData = {
      name: form.value.name,
      path: form.value.parentPath,
      genre: form.value.genre || undefined,
      sub_genre: form.value.sub_genre || undefined,
      target_readers: form.value.target_readers || undefined,
      total_chapters: form.value.total_chapters || undefined,
      words_per_chapter: form.value.words_per_chapter || undefined,
      narrative_pov: form.value.narrative_pov || undefined,
      story_structure: form.value.story_structure || undefined,
    };
    console.log("DEBUG: Sending project data:", projectData);
    
    await store.createProject(projectData);
    emit("close");
  } catch (err: any) {
    errorMsg.value = err?.message || "创建项目失败";
    console.error("Failed to create project:", err);
  } finally {
    submitting.value = false;
  }
};
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-header">
        <h3>新建项目</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="dialog-body">
        <div class="form-group">
          <label>项目名称 *</label>
          <input v-model="form.name" type="text" placeholder="请输入项目名称" maxlength="50" />
        </div>

        <div class="form-group">
          <label>项目描述</label>
          <textarea v-model="form.description" placeholder="请输入项目描述（可选）" rows="2"></textarea>
        </div>

        <div class="form-group">
          <label>存储路径 *</label>
          <div class="path-input">
            <input :value="fullPath" type="text" placeholder="请先输入项目名称并选择父目录" readonly />
            <button @click="selectPath">浏览</button>
          </div>
          <div class="path-hint" v-if="form.parentPath && form.name">
            项目将创建在: {{ fullPath }}
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>题材</label>
            <CustomSelect v-model="form.genre" :options="genres" placeholder="请选择" />
          </div>
          <div class="form-group">
            <label>叙事视角</label>
            <CustomSelect v-model="form.narrative_pov" :options="povs" />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>总章数</label>
            <NumberInput v-model="form.total_chapters" :min="1" :step="1" />
          </div>
          <div class="form-group">
            <label>单章字数</label>
            <NumberInput v-model="form.words_per_chapter" :min="500" :step="500" />
          </div>
        </div>

        <div class="form-group">
          <label>故事结构</label>
          <CustomSelect v-model="form.story_structure" :options="structures" />
        </div>

        <div v-if="errorMsg" class="error-message">
          {{ errorMsg }}
        </div>
      </div>

      <div class="dialog-footer">
        <button class="btn-secondary" @click="emit('close')">取消</button>
        <button class="btn-primary" :disabled="!form.name || !form.parentPath || submitting" @click="submit">
          {{ submitting ? "创建中..." : "创建项目" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  width: 500px;
  max-height: 90vh;
  background: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border);
}

.dialog-header h3 {
  font-size: 18px;
  font-weight: 500;
}

.close-btn {
  width: 28px;
  height: 28px;
  background: none;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 18px;
}

.close-btn:hover {
  background: var(--bg-hover);
}

.dialog-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
}

.form-group {
  margin-bottom: var(--spacing-md);
}

.form-group label {
  display: block;
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  outline: none;
}

.form-group textarea {
  resize: vertical;
  min-height: 60px;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--accent);
}

.path-hint {
  margin-top: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.form-row .form-group {
  min-width: 0;
}

.form-row :deep(.number-input) {
  max-width: 100%;
}

.form-row :deep(.number-input input) {
  min-width: 0;
  width: 0;
}

.path-input {
  display: flex;
  gap: var(--spacing-sm);
}

.path-input input {
  flex: 1;
}

.path-input button {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  cursor: pointer;
}

.path-input button:hover {
  background: var(--bg-hover);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.error-message {
  margin-top: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-md);
  background: rgba(250, 82, 82, 0.1);
  border: 1px solid var(--danger);
  border-radius: 6px;
  color: var(--danger);
  font-size: var(--font-size-sm);
}

.btn-secondary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--bg-surface);
  color: var(--text-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
