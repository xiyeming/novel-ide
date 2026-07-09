<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useProjectStore } from "../../stores/project";
import NewProject from "./NewProject.vue";
import EditProject from "./EditProject.vue";
import ProjectDetails from "./ProjectDetails.vue";
import Toast from "../common/Toast.vue";

const store = useProjectStore();
const showNewDialog = ref(false);
const showEditDialog = ref(false);
const showDetailsDialog = ref(false);
const selectedProject = ref<any>(null);
const toastRef = ref<InstanceType<typeof Toast> | null>(null);

onMounted(() => {
  store.fetchProjects();
});

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString("zh-CN");
};

const emit = defineEmits<{
  openProject: [projectId: string];
}>();

const handleViewDetails = (project: any) => {
  selectedProject.value = project;
  showDetailsDialog.value = true;
};

const handleEdit = (project: any) => {
  selectedProject.value = project;
  showEditDialog.value = true;
};

const handleDelete = async (projectId: string, projectName: string) => {
  if (confirm(`确定要删除项目「${projectName}」吗？此操作不可恢复。`)) {
    try {
      await store.deleteProject(projectId);
      toastRef.value?.addToast(`项目「${projectName}」已删除`, "success");
    } catch (error) {
      toastRef.value?.addToast("删除项目失败", "error");
    }
  }
};

const handleUpdated = () => {
  store.fetchProjects();
  toastRef.value?.addToast("项目已更新", "success");
};
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
      >
        <div class="card-content" @click="emit('openProject', project.id)">
          <div class="card-title">{{ project.name }}</div>
          <div class="card-meta">
            <span v-if="project.genre">{{ project.genre }}</span>
            <span v-if="project.total_chapters">{{ project.total_chapters }} 章</span>
          </div>
          <div class="card-date">更新于 {{ formatDate(project.updated_at) }}</div>
        </div>
        <div class="card-actions">
          <button class="action-btn" @click.stop="handleViewDetails(project)" title="查看详情">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
              <circle cx="12" cy="12" r="3"></circle>
            </svg>
          </button>
          <button class="action-btn" @click.stop="handleEdit(project)" title="编辑">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
            </svg>
          </button>
          <button class="action-btn delete-btn" @click.stop="handleDelete(project.id, project.name)" title="删除">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"></polyline>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <NewProject v-if="showNewDialog" @close="showNewDialog = false" />
    </Teleport>

    <Teleport to="body">
      <EditProject
        v-if="showEditDialog && selectedProject"
        :project="selectedProject"
        @close="showEditDialog = false"
        @updated="handleUpdated"
      />
    </Teleport>

    <Teleport to="body">
      <ProjectDetails
        v-if="showDetailsDialog && selectedProject"
        :project="selectedProject"
        @close="showDetailsDialog = false"
      />
    </Teleport>

    <Toast ref="toastRef" />
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
  background: var(--bg-surface);
  border-radius: 8px;
  transition: all 0.15s;
  border: 1px solid transparent;
  overflow: hidden;
}

.project-card:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.card-content {
  padding: var(--spacing-lg);
  cursor: pointer;
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

.card-actions {
  display: flex;
  border-top: 1px solid var(--border);
}

.action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-sm);
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.action-btn:hover {
  background: var(--bg-hover);
  color: var(--accent);
}

.action-btn.delete-btn:hover {
  color: var(--danger);
}
</style>
