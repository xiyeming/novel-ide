<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useWorkflowStore } from "../../stores/workflow";

const store = useWorkflowStore();
const showCreateDialog = ref(false);
const newName = ref("");
const newDescription = ref("");
const newStages = ref([
  { name: "初稿", stage_type: "draft", temperature: 0.7, max_tokens: 2000 },
]);

onMounted(() => {
  store.fetchWorkflows();
});

function addStage() {
  newStages.value.push({ name: "", stage_type: "custom", temperature: 0.7, max_tokens: 2000 });
}

function removeStage(index: number) {
  newStages.value.splice(index, 1);
}

async function createWorkflow() {
  if (!newName.value) return;
  await store.createWorkflow(newName.value, newDescription.value, newStages.value);
  showCreateDialog.value = false;
  newName.value = "";
  newDescription.value = "";
  newStages.value = [{ name: "初稿", stage_type: "draft", temperature: 0.7, max_tokens: 2000 }];
}

const props = defineProps<{
  chapterId?: string | null;
}>();

async function executeWorkflow(workflowId: string) {
  if (!props.chapterId) return;
  await store.executeWorkflow(workflowId, props.chapterId);
}
</script>

<template>
  <div class="workflow-panel">
    <div class="panel-header">
      <h3>⚙️ 工作流</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 新建</button>
    </div>

    <div v-if="showCreateDialog" class="create-dialog">
      <input v-model="newName" placeholder="工作流名称" class="input" />
      <textarea v-model="newDescription" placeholder="描述（可选）" class="input" rows="2" />

      <div class="stages-list">
        <div v-for="(stage, i) in newStages" :key="i" class="stage-item">
          <input v-model="stage.name" placeholder="阶段名称" class="input-sm" />
          <select v-model="stage.stage_type" class="select-sm">
            <option value="outline">大纲</option>
            <option value="draft">初稿</option>
            <option value="proofread">校对</option>
            <option value="edit">编辑</option>
            <option value="custom">自定义</option>
          </select>
          <button class="btn-icon" @click="removeStage(i)">✕</button>
        </div>
        <button class="btn-sm" @click="addStage">+ 添加阶段</button>
      </div>

      <div class="dialog-actions">
        <button class="btn-sm" @click="showCreateDialog = false">取消</button>
        <button class="btn-primary" @click="createWorkflow">创建</button>
      </div>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>

    <div v-else-if="store.workflows.length === 0" class="empty">暂无工作流</div>

    <div v-else class="workflow-list">
      <div v-for="wf in store.workflows" :key="wf.id" class="workflow-item">
        <div class="wf-info">
          <div class="wf-name">{{ wf.name }}</div>
          <div class="wf-stages">{{ wf.stages.length }} 个阶段</div>
        </div>
        <div class="wf-actions">
          <button
            class="btn-sm btn-execute"
            :disabled="!chapterId"
            title="执行工作流"
            @click="executeWorkflow(wf.id)"
          >
            ▶
          </button>
          <button class="btn-icon danger" @click="store.deleteWorkflow(wf.id)">🗑</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.workflow-panel {
  padding: 12px;
  height: 100%;
  overflow-y: auto;
}
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.panel-header h3 {
  margin: 0;
  font-size: 14px;
}
.btn-sm {
  padding: 4px 8px;
  border: 1px solid #555;
  border-radius: 4px;
  background: #2a2a2a;
  color: #ccc;
  cursor: pointer;
  font-size: 12px;
}
.btn-primary {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  background: #4a9eff;
  color: white;
  cursor: pointer;
}
.btn-icon {
  background: none;
  border: none;
  color: #888;
  cursor: pointer;
  padding: 2px 4px;
}
.btn-icon.danger:hover {
  color: #ff4444;
}
.input {
  width: 100%;
  padding: 6px 8px;
  border: 1px solid #444;
  border-radius: 4px;
  background: #1a1a1a;
  color: #eee;
  margin-bottom: 8px;
  font-size: 13px;
  box-sizing: border-box;
}
.input-sm {
  padding: 4px 6px;
  border: 1px solid #444;
  border-radius: 4px;
  background: #1a1a1a;
  color: #eee;
  font-size: 12px;
  flex: 1;
}
.select-sm {
  padding: 4px 6px;
  border: 1px solid #444;
  border-radius: 4px;
  background: #1a1a1a;
  color: #eee;
  font-size: 12px;
}
.stages-list {
  margin: 8px 0;
}
.stage-item {
  display: flex;
  gap: 4px;
  align-items: center;
  margin-bottom: 4px;
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
  color: #888;
  padding: 20px;
  font-size: 13px;
}
.workflow-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.workflow-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border: 1px solid #333;
  border-radius: 4px;
}
.wf-actions {
  display: flex;
  gap: 4px;
  align-items: center;
}
.btn-execute {
  background: #2a4a2a;
  border: 1px solid #3a6a3a;
  color: #8f8;
  font-size: 11px;
}
.btn-execute:hover:not(:disabled) {
  background: #3a6a3a;
  color: #fff;
}
.btn-execute:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.wf-name {
  font-size: 13px;
  color: #eee;
}
.wf-stages {
  font-size: 11px;
  color: #888;
}
</style>
