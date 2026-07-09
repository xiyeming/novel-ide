<script setup lang="ts">
import { ref } from "vue";
import { useProjectStore } from "../../stores/project";

const emit = defineEmits<{ close: [] }>();
const store = useProjectStore();

const form = ref({
  name: "",
  path: "",
  genre: "",
  sub_genre: "",
  target_readers: "",
  total_chapters: 300,
  words_per_chapter: 3000,
  narrative_pov: "第三人称有限",
  story_structure: "三幕式",
});

const genres = ["仙侠", "玄幻", "都市", "科幻", "历史", "悬疑", "言情", "武侠", "奇幻", "其他"];
const povs = ["第一人称", "第二人称", "第三人称有限", "第三人称全知", "多视角"];
const structures = ["三幕式", "英雄之旅", "起承转合", "非线性", "自由结构"];

const submitting = ref(false);

const selectPath = async () => {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({ directory: true });
    if (selected) {
      form.value.path = selected as string;
    }
  } catch {
    // Browser fallback: set default path that user can modify
    if (!form.value.path) {
      form.value.path = "~/NovelProjects";
    }
  }
};

const submit = async () => {
  if (!form.value.name || !form.value.path) return;
  submitting.value = true;
  try {
    await store.createProject(form.value);
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
        <h3>新建项目</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="dialog-body">
        <div class="form-group">
          <label>项目名称 *</label>
          <input v-model="form.name" type="text" placeholder="请输入项目名称" maxlength="50" />
        </div>

        <div class="form-group">
          <label>存储路径 *</label>
          <div class="path-input">
            <input v-model="form.path" type="text" placeholder="请输入项目目录路径" />
            <button @click="selectPath">浏览</button>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>题材</label>
            <select v-model="form.genre">
              <option value="">请选择</option>
              <option v-for="g in genres" :key="g" :value="g">{{ g }}</option>
            </select>
          </div>
          <div class="form-group">
            <label>叙事视角</label>
            <select v-model="form.narrative_pov">
              <option v-for="p in povs" :key="p" :value="p">{{ p }}</option>
            </select>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>总章数</label>
            <input v-model.number="form.total_chapters" type="number" min="1" />
          </div>
          <div class="form-group">
            <label>单章字数</label>
            <input v-model.number="form.words_per_chapter" type="number" min="500" step="500" />
          </div>
        </div>

        <div class="form-group">
          <label>故事结构</label>
          <select v-model="form.story_structure">
            <option v-for="s in structures" :key="s" :value="s">{{ s }}</option>
          </select>
        </div>
      </div>

      <div class="dialog-footer">
        <button class="btn-secondary" @click="emit('close')">取消</button>
        <button class="btn-primary" :disabled="!form.name || !form.path || submitting" @click="submit">
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
.form-group select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  outline: none;
}

.form-group input:focus,
.form-group select:focus {
  border-color: var(--accent);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
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
