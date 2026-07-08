<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useProjectStore } from "../../stores/project";
import NewProject from "./NewProject.vue";

const store = useProjectStore();
const showNewDialog = ref(false);

onMounted(() => {
  store.fetchProjects();
});

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString("zh-CN");
};

const emit = defineEmits<{
  openProject: [projectId: string];
}>();
</script>

<template>
  <div class="project-list">
    <div class="list-header">
      <h2>我的项目</h2>
      <button class="btn-primary" @click="showNewDialog = true">新建项目</button>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>

    <div v-else-if="store.projects.length === 0" class="empty">
      <div class="empty-icon">📚</div>
      <p>还没有项目</p>
      <p class="empty-sub">点击"新建项目"开始创作</p>
    </div>

    <div v-else class="project-grid">
      <div
        v-for="project in store.projects"
        :key="project.id"
        class="project-card"
        @click="emit('openProject', project.id)"
      >
        <div class="card-title">{{ project.name }}</div>
        <div class="card-meta">
          <span v-if="project.genre">{{ project.genre }}</span>
          <span v-if="project.total_chapters">{{ project.total_chapters }} 章</span>
        </div>
        <div class="card-date">更新于 {{ formatDate(project.updated_at) }}</div>
      </div>
    </div>

    <Teleport to="body">
      <NewProject v-if="showNewDialog" @close="showNewDialog = false" />
    </Teleport>
  </div>
</template>

<style scoped>
.project-list {
  padding: var(--spacing-xl);
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-xl);
}

.list-header h2 {
  font-size: 20px;
  font-weight: 500;
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: var(--font-size-md);
}

.btn-primary:hover {
  opacity: 0.9;
}

.empty {
  text-align: center;
  padding: 60px 0;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: var(--spacing-lg);
}

.empty-sub {
  font-size: var(--font-size-sm);
  margin-top: var(--spacing-sm);
}

.project-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: var(--spacing-lg);
}

.project-card {
  padding: var(--spacing-lg);
  background: var(--bg-surface);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s;
  border: 1px solid transparent;
}

.project-card:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.card-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: var(--spacing-sm);
}

.card-meta {
  display: flex;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-sm);
}

.card-date {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}
</style>
