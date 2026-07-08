<script setup lang="ts">
import { ref, watch } from "vue";
import { useProjectStore, type Project } from "../../stores/project";

const projectStore = useProjectStore();
const project = ref<Project | null>(null);

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

const genres = ["玄幻", "仙侠", "都市", "科幻", "历史", "言情", "悬疑", "恐怖", "武侠", "奇幻", "其他"];
const povs = ["第一人称", "第三人称有限", "第三人称全知", "第二人称"];
const structures = ["三幕式", "英雄之旅", "起承转合", "非线性", "自定义"];

const syncFromStore = () => {
  if (projectStore.currentProject) {
    project.value = { ...projectStore.currentProject };
  }
};

watch(
  () => projectStore.currentProject,
  () => syncFromStore(),
  { immediate: true }
);

const scheduleSave = () => {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    save();
  }, 500);
};

const save = async () => {
  if (!project.value || !projectStore.currentProject) return;
  const id = projectStore.currentProject.id;
  const {
    genre,
    sub_genre,
    target_readers,
    total_chapters,
    words_per_chapter,
    narrative_pov,
    story_structure,
    core_outline,
    world_settings,
    character_profiles,
    golden_finger,
    writing_constraints,
    style_constraints,
  } = project.value;
  await projectStore.updateProject(id, {
    genre,
    sub_genre,
    target_readers,
    total_chapters,
    words_per_chapter,
    narrative_pov,
    story_structure,
    core_outline,
    world_settings,
    character_profiles,
    golden_finger,
    writing_constraints,
    style_constraints,
  });
};
</script>

<template>
  <div v-if="project" class="config-panel">
    <div class="config-section">
      <div class="section-title">基本信息</div>
      <div class="form-group">
        <label>题材</label>
        <select v-model="project.genre" @change="scheduleSave">
          <option :value="null">请选择</option>
          <option v-for="g in genres" :key="g" :value="g">{{ g }}</option>
        </select>
      </div>
      <div class="form-group">
        <label>细分类别</label>
        <input v-model="project.sub_genre" type="text" placeholder="如：末日废土、赛博朋克" @input="scheduleSave" />
      </div>
      <div class="form-group">
        <label>目标读者</label>
        <input v-model="project.target_readers" type="text" placeholder="如：18-35岁男性" @input="scheduleSave" />
      </div>
    </div>

    <div class="config-section">
      <div class="section-title">章节设置</div>
      <div class="form-row">
        <div class="form-group">
          <label>总章数</label>
          <input
            v-model.number="project.total_chapters"
            type="number"
            min="10"
            max="10000"
            @input="scheduleSave"
          />
        </div>
        <div class="form-group">
          <label>单章字数</label>
          <input
            v-model.number="project.words_per_chapter"
            type="number"
            min="1000"
            max="10000"
            step="500"
            @input="scheduleSave"
          />
        </div>
      </div>
    </div>

    <div class="config-section">
      <div class="section-title">叙事设定</div>
      <div class="form-group">
        <label>叙事视角</label>
        <select v-model="project.narrative_pov" @change="scheduleSave">
          <option :value="null">请选择</option>
          <option v-for="p in povs" :key="p" :value="p">{{ p }}</option>
        </select>
      </div>
      <div class="form-group">
        <label>故事结构</label>
        <select v-model="project.story_structure" @change="scheduleSave">
          <option :value="null">请选择</option>
          <option v-for="s in structures" :key="s" :value="s">{{ s }}</option>
        </select>
      </div>
    </div>

    <div class="config-section">
      <div class="section-title">创作设定</div>
      <div class="form-group">
        <label>核心大纲</label>
        <textarea
          v-model="project.core_outline"
          rows="4"
          placeholder="故事主线、核心冲突、结局走向…"
          @input="scheduleSave"
        ></textarea>
      </div>
      <div class="form-group">
        <label>世界设定</label>
        <textarea
          v-model="project.world_settings"
          rows="4"
          placeholder="世界观、力量体系、社会结构…"
          @input="scheduleSave"
        ></textarea>
      </div>
      <div class="form-group">
        <label>主角档案</label>
        <textarea
          v-model="project.character_profiles"
          rows="4"
          placeholder="主要角色的背景、性格、动机…"
          @input="scheduleSave"
        ></textarea>
      </div>
      <div class="form-group">
        <label>金手指/外挂</label>
        <textarea
          v-model="project.golden_finger"
          rows="3"
          placeholder="主角的特殊能力或系统…"
          @input="scheduleSave"
        ></textarea>
      </div>
    </div>

    <div class="config-section">
      <div class="section-title">写作约束</div>
      <div class="form-group">
        <label>全局写作要求</label>
        <textarea
          v-model="project.writing_constraints"
          rows="3"
          placeholder="如：每章必须有至少一个冲突、禁止水字数…"
          @input="scheduleSave"
        ></textarea>
      </div>
      <div class="form-group">
        <label>文风约束</label>
        <textarea
          v-model="project.style_constraints"
          rows="3"
          placeholder="如：简洁明快、避免冗长描写…"
          @input="scheduleSave"
        ></textarea>
      </div>
    </div>
  </div>
  <div v-else class="empty-state">未打开项目</div>
</template>

<style scoped>
.config-panel {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.config-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding-bottom: var(--spacing-xs);
  border-bottom: 1px solid var(--border);
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
.form-group select,
.form-group textarea {
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
.form-group select:focus,
.form-group textarea:focus {
  border-color: var(--accent);
}

.form-group textarea {
  resize: vertical;
  min-height: 60px;
  font-family: inherit;
  line-height: 1.5;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
  text-align: center;
  padding: var(--spacing-xl);
}
</style>
