# Phase 6: Workflow + Agent System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a workflow pipeline system for automated writing stages and an AI agent system with different specialized roles.

**Architecture:** Workflow system allows creating/configuring pipelines (e.g., 大纲→初稿→校对→定稿) with configurable stages and AI model per stage. Agent system provides specialized AI roles (大纲助手、初稿作家、校对编辑) with system prompts and knowledge base integration.

**Tech Stack:** Rust 1.90+ (serde_json, reqwest, tokio), SQLite, Vue 3.5.39, Pinia 3.0.4, TypeScript 5.7+

## Global Constraints
- Rust 1.90+, Tauri 2.11, SQLx 0.9.0, LanceDB 0.31.0
- Vue 3.5.39, Pinia 3.0.4, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- 14 preset model providers
- 11 error types defined in AppError enum

## File Structure

```
src-tauri/
  src/
    commands/
      workflow.rs       — Workflow CRUD commands
      agent.rs          — Agent CRUD commands
    db/
      migrations/
        007_workflow_agent.sql — Workflow + Agent tables
    lib.rs              — Register new commands
src/
  stores/
    workflow.ts         — Workflow state
    agent.ts            — Agent state
  components/
    workflow/
      WorkflowPanel.vue — Pipeline configuration UI
      WorkflowStage.vue — Stage configuration component
    agent/
      AgentPanel.vue    — Agent management UI
```

## Tasks

### Task 1: Workflow + Agent Migration

**Files:**
- Create: `src-tauri/src/db/migrations/007_workflow_agent.sql`

**Steps:**

- [ ] **Step 1: Write migration SQL**

```sql
-- 007_workflow_agent.sql
CREATE TABLE IF NOT EXISTS workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    stages TEXT NOT NULL DEFAULT '[]', -- JSON array of stage configs
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS workflow_executions (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    chapter_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending/running/completed/failed
    current_stage INTEGER NOT NULL DEFAULT 0,
    results TEXT NOT NULL DEFAULT '{}', -- JSON map of stage results
    error TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    role TEXT NOT NULL, -- outline/writer/proofreader/editor/custom
    system_prompt TEXT NOT NULL,
    model_provider_id TEXT,
    temperature REAL NOT NULL DEFAULT 0.7,
    max_tokens INTEGER NOT NULL DEFAULT 2000,
    knowledge_base_ids TEXT NOT NULL DEFAULT '[]', -- JSON array of KB IDs
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_workflow_executions_workflow_id ON workflow_executions(workflow_id);
CREATE INDEX IF NOT EXISTS idx_workflow_executions_chapter_id ON workflow_executions(chapter_id);
```

- [ ] **Step 2: Register migration in db/mod.rs**

Add migration 007 to the migrations list.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/db/migrations/007_workflow_agent.sql src-tauri/src/db/mod.rs
git commit -m "feat: add workflow + agent migration"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 2: Workflow + Agent Models

**Files:**
- Create: `src-tauri/src/models/workflow.rs`
- Create: `src-tauri/src/models/agent.rs`
- Modify: `src-tauri/src/models/mod.rs`

**Steps:**

- [ ] **Step 1: Create workflow.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<WorkflowStage>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub name: String,
    pub stage_type: String, // outline/draft/proofread/edit/custom
    pub agent_id: Option<String>,
    pub model_provider_id: Option<String>,
    pub system_prompt: Option<String>,
    pub temperature: f64,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    pub chapter_id: String,
    pub status: String, // pending/running/completed/failed
    pub current_stage: usize,
    pub results: std::collections::HashMap<String, String>,
    pub error: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}
```

- [ ] **Step 2: Create agent.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: String, // outline/writer/proofreader/editor/custom
    pub system_prompt: String,
    pub model_provider_id: Option<String>,
    pub temperature: f64,
    pub max_tokens: u32,
    pub knowledge_base_ids: Vec<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

- [ ] **Step 3: Register in mod.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models/
git commit -m "feat: add workflow + agent models"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 3: Workflow Commands

**Files:**
- Create: `src-tauri/src/commands/workflow.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Steps:**

- [ ] **Step 1: Create workflow.rs with commands**

```rust
use crate::error::AppError;
use crate::models::workflow::{Workflow, WorkflowStage, WorkflowExecution};
use crate::state::AppState;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_workflow(
    state: State<'_, AppState>,
    name: String,
    description: Option<String>,
    stages: Vec<WorkflowStage>,
) -> Result<Workflow, AppError> {
    let db = state.db.lock().await;
    let id = Uuid::new_v4().to_string();
    let stages_json = serde_json::to_string(&stages)?;
    
    sqlx::query("INSERT INTO workflows (id, name, description, stages) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&description)
        .bind(&stages_json)
        .execute(&*db)
        .await?;

    Ok(Workflow {
        id,
        name,
        description,
        stages,
        is_active: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub async fn list_workflows(
    state: State<'_, AppState>,
) -> Result<Vec<Workflow>, AppError> {
    let db = state.db.lock().await;
    let rows = sqlx::query("SELECT * FROM workflows ORDER BY created_at DESC")
        .fetch_all(&*db)
        .await?;

    rows.into_iter().map(|row| {
        let stages_json: String = row.get("stages");
        let stages: Vec<WorkflowStage> = serde_json::from_str(&stages_json)?;
        Ok(Workflow {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            stages,
            is_active: row.get::<i64, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn delete_workflow(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db.lock().await;
    sqlx::query("DELETE FROM workflows WHERE id = ?")
        .bind(&id)
        .execute(&*db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn execute_workflow(
    state: State<'_, AppState>,
    workflow_id: String,
    chapter_id: String,
) -> Result<WorkflowExecution, AppError> {
    let db = state.db.lock().await;
    let id = Uuid::new_v4().to_string();
    
    // Get workflow
    let row = sqlx::query("SELECT * FROM workflows WHERE id = ?")
        .bind(&workflow_id)
        .fetch_optional(&*db)
        .await?
        .ok_or(AppError::Internal("工作流不存在".into()))?;
    
    let stages_json: String = row.get("stages");
    let stages: Vec<WorkflowStage> = serde_json::from_str(&stages_json)?;
    
    // Create execution
    let results = std::collections::HashMap::new();
    let results_json = serde_json::to_string(&results)?;
    
    sqlx::query("INSERT INTO workflow_executions (id, workflow_id, chapter_id, status, stages_count) VALUES (?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&workflow_id)
        .bind(&chapter_id)
        .bind("pending")
        .bind(stages.len() as i64)
        .execute(&*db)
        .await?;

    Ok(WorkflowExecution {
        id,
        workflow_id,
        chapter_id,
        status: "pending".into(),
        current_stage: 0,
        results,
        error: None,
        started_at: None,
        completed_at: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub async fn get_workflow_execution(
    state: State<'_, AppState>,
    id: String,
) -> Result<WorkflowExecution, AppError> {
    let db = state.db.lock().await;
    let row = sqlx::query("SELECT * FROM workflow_executions WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*db)
        .await?
        .ok_or(AppError::Internal("执行记录不存在".into()))?;
    
    let results_json: String = row.get("results");
    let results: std::collections::HashMap<String, String> = serde_json::from_str(&results_json)?;
    
    Ok(WorkflowExecution {
        id: row.get("id"),
        workflow_id: row.get("workflow_id"),
        chapter_id: row.get("chapter_id"),
        status: row.get("status"),
        current_stage: row.get::<i64, _>("current_stage") as usize,
        results,
        error: row.get("error"),
        started_at: row.get("started_at"),
        completed_at: row.get("completed_at"),
        created_at: row.get("created_at"),
    })
}
```

- [ ] **Step 2: Register in mod.rs and lib.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/workflow.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add workflow commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 4: Agent Commands

**Files:**
- Create: `src-tauri/src/commands/agent.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Steps:**

- [ ] **Step 1: Create agent.rs with commands**

```rust
use crate::error::AppError;
use crate::models::agent::Agent;
use crate::state::AppState;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_agent(
    state: State<'_, AppState>,
    name: String,
    role: String,
    system_prompt: String,
    model_provider_id: Option<String>,
    temperature: f64,
    max_tokens: u32,
    knowledge_base_ids: Vec<String>,
) -> Result<Agent, AppError> {
    let db = state.db.lock().await;
    let id = Uuid::new_v4().to_string();
    let kb_ids_json = serde_json::to_string(&knowledge_base_ids)?;
    
    sqlx::query("INSERT INTO agents (id, name, role, system_prompt, model_provider_id, temperature, max_tokens, knowledge_base_ids) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&role)
        .bind(&system_prompt)
        .bind(&model_provider_id)
        .bind(temperature)
        .bind(max_tokens as i64)
        .bind(&kb_ids_json)
        .execute(&*db)
        .await?;

    Ok(Agent {
        id,
        name,
        role,
        system_prompt,
        model_provider_id,
        temperature,
        max_tokens,
        knowledge_base_ids,
        is_active: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub async fn list_agents(
    state: State<'_, AppState>,
) -> Result<Vec<Agent>, AppError> {
    let db = state.db.lock().await;
    let rows = sqlx::query("SELECT * FROM agents ORDER BY created_at DESC")
        .fetch_all(&*db)
        .await?;

    rows.into_iter().map(|row| {
        let kb_ids_json: String = row.get("knowledge_base_ids");
        let knowledge_base_ids: Vec<String> = serde_json::from_str(&kb_ids_json)?;
        Ok(Agent {
            id: row.get("id"),
            name: row.get("name"),
            role: row.get("role"),
            system_prompt: row.get("system_prompt"),
            model_provider_id: row.get("model_provider_id"),
            temperature: row.get("temperature"),
            max_tokens: row.get::<i64, _>("max_tokens") as u32,
            knowledge_base_ids,
            is_active: row.get::<i64, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn update_agent(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<u32>,
    knowledge_base_ids: Option<Vec<String>>,
) -> Result<Agent, AppError> {
    let db = state.db.lock().await;
    
    if let Some(n) = name {
        sqlx::query("UPDATE agents SET name = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&n).bind(&id).execute(&*db).await?;
    }
    if let Some(sp) = system_prompt {
        sqlx::query("UPDATE agents SET system_prompt = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&sp).bind(&id).execute(&*db).await?;
    }
    if let Some(t) = temperature {
        sqlx::query("UPDATE agents SET temperature = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(t).bind(&id).execute(&*db).await?;
    }
    if let Some(mt) = max_tokens {
        sqlx::query("UPDATE agents SET max_tokens = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(mt as i64).bind(&id).execute(&*db).await?;
    }
    if let Some(kb) = knowledge_base_ids {
        let json = serde_json::to_string(&kb)?;
        sqlx::query("UPDATE agents SET knowledge_base_ids = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&json).bind(&id).execute(&*db).await?;
    }

    // Return updated agent
    let row = sqlx::query("SELECT * FROM agents WHERE id = ?")
        .bind(&id)
        .fetch_one(&*db)
        .await?;
    
    let kb_ids_json: String = row.get("knowledge_base_ids");
    let knowledge_base_ids: Vec<String> = serde_json::from_str(&kb_ids_json)?;
    
    Ok(Agent {
        id: row.get("id"),
        name: row.get("name"),
        role: row.get("role"),
        system_prompt: row.get("system_prompt"),
        model_provider_id: row.get("model_provider_id"),
        temperature: row.get("temperature"),
        max_tokens: row.get::<i64, _>("max_tokens") as u32,
        knowledge_base_ids,
        is_active: row.get::<i64, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn delete_agent(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db.lock().await;
    sqlx::query("DELETE FROM agents WHERE id = ?")
        .bind(&id)
        .execute(&*db)
        .await?;
    Ok(())
}
```

- [ ] **Step 2: Register in mod.rs and lib.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/agent.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add agent commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 5: Workflow + Agent Stores

**Files:**
- Create: `src/stores/workflow.ts`
- Create: `src/stores/agent.ts`

**Steps:**

- [ ] **Step 1: Create workflow.ts**

```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export interface WorkflowStage {
  name: string
  stage_type: string
  agent_id?: string
  model_provider_id?: string
  system_prompt?: string
  temperature: number
  max_tokens: number
}

export interface Workflow {
  id: string
  name: string
  description?: string
  stages: WorkflowStage[]
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface WorkflowExecution {
  id: string
  workflow_id: string
  chapter_id: string
  status: string
  current_stage: number
  results: Record<string, string>
  error?: string
  started_at?: string
  completed_at?: string
  created_at: string
}

export const useWorkflowStore = defineStore('workflow', {
  state: () => ({
    workflows: [] as Workflow[],
    currentExecution: null as WorkflowExecution | null,
    loading: false,
  }),
  actions: {
    async fetchWorkflows() {
      this.loading = true
      try {
        this.workflows = await invoke('list_workflows')
      } finally {
        this.loading = false
      }
    },
    async createWorkflow(name: string, description: string, stages: WorkflowStage[]) {
      const workflow = await invoke<Workflow>('create_workflow', { name, description, stages })
      this.workflows.unshift(workflow)
      return workflow
    },
    async deleteWorkflow(id: string) {
      await invoke('delete_workflow', { id })
      this.workflows = this.workflows.filter(w => w.id !== id)
    },
    async executeWorkflow(workflowId: string, chapterId: string) {
      const execution = await invoke<WorkflowExecution>('execute_workflow', { workflowId, chapterId })
      this.currentExecution = execution
      return execution
    },
  },
})
```

- [ ] **Step 2: Create agent.ts**

```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export interface Agent {
  id: string
  name: string
  role: string
  system_prompt: string
  model_provider_id?: string
  temperature: number
  max_tokens: number
  knowledge_base_ids: string[]
  is_active: boolean
  created_at: string
  updated_at: string
}

export const useAgentStore = defineStore('agent', {
  state: () => ({
    agents: [] as Agent[],
    loading: false,
  }),
  actions: {
    async fetchAgents() {
      this.loading = true
      try {
        this.agents = await invoke('list_agents')
      } finally {
        this.loading = false
      }
    },
    async createAgent(data: {
      name: string
      role: string
      systemPrompt: string
      modelProviderId?: string
      temperature: number
      maxTokens: number
      knowledgeBaseIds: string[]
    }) {
      const agent = await invoke<Agent>('create_agent', {
        name: data.name,
        role: data.role,
        systemPrompt: data.systemPrompt,
        modelProviderId: data.modelProviderId,
        temperature: data.temperature,
        maxTokens: data.maxTokens,
        knowledgeBaseIds: data.knowledgeBaseIds,
      })
      this.agents.unshift(agent)
      return agent
    },
    async updateAgent(id: string, data: Partial<Agent>) {
      const agent = await invoke<Agent>('update_agent', { id, ...data })
      const idx = this.agents.findIndex(a => a.id === id)
      if (idx >= 0) this.agents[idx] = agent
      return agent
    },
    async deleteAgent(id: string) {
      await invoke('delete_agent', { id })
      this.agents = this.agents.filter(a => a.id !== id)
    },
  },
})
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/workflow.ts src/stores/agent.ts
git commit -m "feat: add workflow + agent stores"
```

**Verification:** `bun run build`

---

### Task 6: Workflow UI

**Files:**
- Create: `src/components/workflow/WorkflowPanel.vue`
- Create: `src/components/workflow/WorkflowStage.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Create WorkflowPanel.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWorkflowStore } from '../../stores/workflow'

const store = useWorkflowStore()
const showCreateDialog = ref(false)
const newName = ref('')
const newDescription = ref('')
const newStages = ref([
  { name: '初稿', stage_type: 'draft', temperature: 0.7, max_tokens: 2000 },
])

onMounted(() => {
  store.fetchWorkflows()
})

function addStage() {
  newStages.value.push({ name: '', stage_type: 'custom', temperature: 0.7, max_tokens: 2000 })
}

function removeStage(index: number) {
  newStages.value.splice(index, 1)
}

async function createWorkflow() {
  if (!newName.value) return
  await store.createWorkflow(newName.value, newDescription.value, newStages.value)
  showCreateDialog.value = false
  newName.value = ''
  newDescription.value = ''
  newStages.value = [{ name: '初稿', stage_type: 'draft', temperature: 0.7, max_tokens: 2000 }]
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
    
    <div v-else-if="store.workflows.length === 0" class="empty">
      暂无工作流
    </div>

    <div v-else class="workflow-list">
      <div v-for="wf in store.workflows" :key="wf.id" class="workflow-item">
        <div class="wf-info">
          <div class="wf-name">{{ wf.name }}</div>
          <div class="wf-stages">{{ wf.stages.length }} 个阶段</div>
        </div>
        <button class="btn-icon danger" @click="store.deleteWorkflow(wf.id)">🗑</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.workflow-panel { padding: 12px; height: 100%; overflow-y: auto; }
.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.panel-header h3 { margin: 0; font-size: 14px; }
.btn-sm { padding: 4px 8px; border: 1px solid #555; border-radius: 4px; background: #2a2a2a; color: #ccc; cursor: pointer; font-size: 12px; }
.btn-primary { padding: 6px 12px; border: none; border-radius: 4px; background: #4a9eff; color: white; cursor: pointer; }
.btn-icon { background: none; border: none; color: #888; cursor: pointer; padding: 2px 4px; }
.btn-icon.danger:hover { color: #ff4444; }
.input { width: 100%; padding: 6px 8px; border: 1px solid #444; border-radius: 4px; background: #1a1a1a; color: #eee; margin-bottom: 8px; font-size: 13px; box-sizing: border-box; }
.input-sm { padding: 4px 6px; border: 1px solid #444; border-radius: 4px; background: #1a1a1a; color: #eee; font-size: 12px; flex: 1; }
.select-sm { padding: 4px 6px; border: 1px solid #444; border-radius: 4px; background: #1a1a1a; color: #eee; font-size: 12px; }
.stages-list { margin: 8px 0; }
.stage-item { display: flex; gap: 4px; align-items: center; margin-bottom: 4px; }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 8px; }
.loading, .empty { text-align: center; color: #888; padding: 20px; font-size: 13px; }
.workflow-list { display: flex; flex-direction: column; gap: 4px; }
.workflow-item { display: flex; justify-content: space-between; align-items: center; padding: 8px; border: 1px solid #333; border-radius: 4px; }
.wf-name { font-size: 13px; color: #eee; }
.wf-stages { font-size: 11px; color: #888; }
</style>
```

- [ ] **Step 2: Add workflow tab to Sidebar.vue**

Add a new tab for workflows (⚙️ 工作流) and import WorkflowPanel.

- [ ] **Step 3: Commit**

```bash
git add src/components/workflow/ src/components/layout/Sidebar.vue
git commit -m "feat: add workflow UI"
```

**Verification:** `bun run build`

---

### Task 7: Agent UI

**Files:**
- Create: `src/components/agent/AgentPanel.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Create AgentPanel.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAgentStore } from '../../stores/agent'

const store = useAgentStore()
const showCreateDialog = ref(false)
const editingAgent = ref<string | null>(null)
const newName = ref('')
const newRole = ref('writer')
const newSystemPrompt = ref('')
const newTemperature = ref(0.7)
const newMaxTokens = ref(2000)

const roles = [
  { value: 'outline', label: '大纲助手' },
  { value: 'writer', label: '初稿作家' },
  { value: 'proofreader', label: '校对编辑' },
  { value: 'editor', label: '终审编辑' },
  { value: 'custom', label: '自定义' },
]

onMounted(() => {
  store.fetchAgents()
})

async function createAgent() {
  if (!newName.value || !newSystemPrompt.value) return
  await store.createAgent({
    name: newName.value,
    role: newRole.value,
    systemPrompt: newSystemPrompt.value,
    temperature: newTemperature.value,
    maxTokens: newMaxTokens.value,
    knowledgeBaseIds: [],
  })
  resetForm()
  showCreateDialog.value = false
}

function resetForm() {
  newName.value = ''
  newRole.value = 'writer'
  newSystemPrompt.value = ''
  newTemperature.value = 0.7
  newMaxTokens.value = 2000
}

function getRoleLabel(role: string) {
  return roles.find(r => r.value === role)?.label || role
}
</script>

<template>
  <div class="agent-panel">
    <div class="panel-header">
      <h3>🤖 智能体</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 新建</button>
    </div>

    <div v-if="showCreateDialog" class="create-dialog">
      <input v-model="newName" placeholder="智能体名称" class="input" />
      <select v-model="newRole" class="input">
        <option v-for="r in roles" :key="r.value" :value="r.value">{{ r.label }}</option>
      </select>
      <textarea v-model="newSystemPrompt" placeholder="系统提示词..." class="input" rows="4" />
      
      <div class="param-row">
        <label>温度: {{ newTemperature }}</label>
        <input type="range" v-model.number="newTemperature" min="0" max="2" step="0.1" />
      </div>
      <div class="param-row">
        <label>最大Token: {{ newMaxTokens }}</label>
        <input type="range" v-model.number="newMaxTokens" min="100" max="8000" step="100" />
      </div>

      <div class="dialog-actions">
        <button class="btn-sm" @click="showCreateDialog = false; resetForm()">取消</button>
        <button class="btn-primary" @click="createAgent">创建</button>
      </div>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>
    
    <div v-else-if="store.agents.length === 0" class="empty">
      暂无智能体
    </div>

    <div v-else class="agent-list">
      <div v-for="agent in store.agents" :key="agent.id" class="agent-item">
        <div class="agent-info">
          <div class="agent-name">{{ agent.name }}</div>
          <div class="agent-role">{{ getRoleLabel(agent.role) }}</div>
        </div>
        <button class="btn-icon danger" @click="store.deleteAgent(agent.id)">🗑</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.agent-panel { padding: 12px; height: 100%; overflow-y: auto; }
.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.panel-header h3 { margin: 0; font-size: 14px; }
.btn-sm { padding: 4px 8px; border: 1px solid #555; border-radius: 4px; background: #2a2a2a; color: #ccc; cursor: pointer; font-size: 12px; }
.btn-primary { padding: 6px 12px; border: none; border-radius: 4px; background: #4a9eff; color: white; cursor: pointer; }
.btn-icon { background: none; border: none; color: #888; cursor: pointer; padding: 2px 4px; }
.btn-icon.danger:hover { color: #ff4444; }
.input { width: 100%; padding: 6px 8px; border: 1px solid #444; border-radius: 4px; background: #1a1a1a; color: #eee; margin-bottom: 8px; font-size: 13px; box-sizing: border-box; }
.param-row { display: flex; flex-direction: column; gap: 4px; margin-bottom: 8px; }
.param-row label { font-size: 12px; color: #aaa; }
.param-row input[type="range"] { width: 100%; }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 8px; }
.loading, .empty { text-align: center; color: #888; padding: 20px; font-size: 13px; }
.agent-list { display: flex; flex-direction: column; gap: 4px; }
.agent-item { display: flex; justify-content: space-between; align-items: center; padding: 8px; border: 1px solid #333; border-radius: 4px; }
.agent-name { font-size: 13px; color: #eee; }
.agent-role { font-size: 11px; color: #888; }
</style>
```

- [ ] **Step 2: Add agent tab to Sidebar.vue**

Add a new tab for agents (🤖 智能体) and import AgentPanel.

- [ ] **Step 3: Commit**

```bash
git add src/components/agent/ src/components/layout/Sidebar.vue
git commit -m "feat: add agent UI"
```

**Verification:** `bun run build`

---

### Task 8: Integration Verification

**Files:**
- None (verification only)

**Steps:**

- [ ] **Step 1: Verify all commands in lib.rs**

Count should now be ~35+.

- [ ] **Step 2: Run cargo check**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 3: Run bun run build**

```bash
bun run build
```

- [ ] **Step 4: Verify all new files exist**

- [ ] **Step 5: Report**

---

## Commit Log (Expected)

```
Task 1: feat: add workflow + agent migration
Task 2: feat: add workflow + agent models
Task 3: feat: add workflow commands
Task 4: feat: add agent commands
Task 5: feat: add workflow + agent stores
Task 6: feat: add workflow UI
Task 7: feat: add agent UI
Task 8: (verification only)
```
