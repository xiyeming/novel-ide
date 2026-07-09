<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useProjectStore } from "../../stores/project";
import CustomSelect from "../common/CustomSelect.vue";
import NumberInput from "../common/NumberInput.vue";

interface Project {
  id: string;
  name: string;
  description?: string;
  genre?: string;
  narrative_pov?: string;
  total_chapters?: number;
  words_per_chapter?: number;
  story_structure?: string;
  path: string;
}

const props = defineProps<{
  project: Project;
}>();

const emit = defineEmits<{
  close: [];
  updated: [];
}>();

const store = useProjectStore();

const form = ref({
  name: "",
  description: "",
  genre: "",
  narrative_pov: "第三人称有限",
  total_chapters: 300,
  words_per_chapter: 3000,
  story_structure: "三幕式",
});

onMounted(() => {
  if (props.project) {
    form.value = {
      name: props.project.name,
      description: props.project.description || "",
      genre: props.project.genre || "",
      narrative_pov: props.project.narrative_pov || "第三人称有限",
      total_chapters: props.project.total_chapters || 300,
      words_per_chapter: props.project.words_per_chapter || 3000,
      story_structure: props.project.story_structure || "三幕式",
    };
  }
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

const submit = async () => {
  if (!form.value.name) return;
  submitting.value = true;
  try {
    await store.updateProject(props.project.id, form.value);
    emit("updated");
    emit("close");
  } finally {
    submitting.value = false;
  }
};
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-header">
        <h3>编辑项目</h3>
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
      </div>

      <div class="dialog-footer">
        <button class="btn-secondary" @click="emit('close')">取消</button>
        <button class="btn-primary" :disabled="!form.name || submitting" @click="submit">
          {{ submitting ? "保存中..." : "保存修改" }}
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
  max-height: 80vh;
  background: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
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
.form-group textarea:focus {
  border-color: var(--accent);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border);
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
