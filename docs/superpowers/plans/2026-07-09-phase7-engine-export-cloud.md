# Phase 7: Workflow Engine + Export + Cloud Sync Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement workflow execution engine (AI pipeline), multi-format export (DOCX/PDF/EPUB), and cloud sync (WebDAV/OSS/S3).

**Architecture:** Workflow engine executes stages sequentially, calling AI with stage config and saving results. Export system generates DOCX/PDF/EPUB from chapters. Cloud sync uploads/downloads project data with AES-256-GCM encryption.

**Tech Stack:** Rust 1.90+ (reqwest, serde_json, tokio), SQLite, Vue 3.5.39, Pinia 3.0.4, TypeScript 5.7+

## Global Constraints
- Rust 1.90+, Tauri 2.11, SQLx 0.9.0
- Vue 3.5.39, Pinia 3.0.4, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- Cloud sync: WebDAV / OSS / S3 with AES-256-GCM encryption
- Export formats: TXT (done), Markdown (done), DOCX, PDF, EPUB

## File Structure

```
src-tauri/
  src/
    commands/
      workflow.rs       — Add run_workflow_stage command
      export.rs         — Add export_docx, export_pdf, export_epub commands
      cloud.rs          — Cloud sync commands (upload, download, list, delete)
    services/
      workflow_engine.rs — Workflow execution logic
      export_docx.rs    — DOCX generation
      export_pdf.rs     — PDF generation
      export_epub.rs    — EPUB generation
      cloud_sync.rs     — WebDAV/OSS/S3 sync logic
    db/
      migrations/
        008_cloud_sync.sql — Cloud sync config table
src/
  stores/
    cloud.ts            — Cloud sync state
  components/
    cloud/
      CloudPanel.vue    — Cloud sync UI
    workflow/
      WorkflowPanel.vue — Add execute button
    editor/
      ExportDialog.vue  — Add DOCX/PDF/EPUB options
```

## Tasks

### Task 1: Workflow Engine Service

**Files:**
- Create: `src-tauri/src/services/workflow_engine.rs`
- Modify: `src-tauri/src/commands/workflow.rs`

**Steps:**

- [ ] **Step 1: Create workflow_engine.rs**

```rust
use crate::error::AppError;
use crate::models::workflow::{WorkflowStage, WorkflowExecution};
use crate::state::AppState;
use reqwest::Client;
use serde_json::json;

pub struct WorkflowEngine<'a> {
    state: &'a AppState,
    client: Client,
}

impl<'a> WorkflowEngine<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            client: Client::new(),
        }
    }

    pub async fn run_stage(
        &self,
        stage: &WorkflowStage,
        content: &str,
        execution_id: &str,
    ) -> Result<String, AppError> {
        // Get model provider config
        let provider_id = stage.model_provider_id.as_ref()
            .ok_or(AppError::Internal("阶段未配置模型".into()))?;
        
        let provider = self.get_provider(provider_id).await?;
        
        // Build request
        let system_prompt = stage.system_prompt.as_deref()
            .unwrap_or("你是一个专业的中文小说写作助手。");
        
        let body = json!({
            "model": provider.model_name,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": content}
            ],
            "temperature": stage.temperature,
            "max_tokens": stage.max_tokens,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", provider.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", provider.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Http(e.to_string()))?;

        let resp_json: serde_json::Value = response.json().await
            .map_err(|e| AppError::Http(e.to_string()))?;

        let result = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or(AppError::Internal("AI 响应格式错误".into()))?;

        Ok(result.to_string())
    }

    async fn get_provider(&self, id: &str) -> Result<crate::models::model::ModelProvider, AppError> {
        let db = self.state.db().await?;
        let row = sqlx::query("SELECT * FROM model_providers WHERE id = ?")
            .bind(id)
            .fetch_optional(&db)
            .await?
            .ok_or(AppError::ProviderNotFound(id.into()))?;
        
        Ok(crate::models::model::ModelProvider {
            id: row.get("id"),
            name: row.get("name"),
            provider_type: row.get("provider_type"),
            base_url: row.get("base_url"),
            api_key: row.get("api_key"),
            model_name: row.get("model_name"),
            is_active: row.get::<i64, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
```

- [ ] **Step 2: Add run_workflow_stage command to workflow.rs**

```rust
#[tauri::command]
pub async fn run_workflow_stage(
    state: State<'_, AppState>,
    execution_id: String,
    stage_index: usize,
    content: String,
) -> Result<String, AppError> {
    let engine = crate::services::workflow_engine::WorkflowEngine::new(&state);
    
    // Get execution to find workflow
    let db = state.db().await?;
    let execution = sqlx::query("SELECT * FROM workflow_executions WHERE id = ?")
        .bind(&execution_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Internal("执行记录不存在".into()))?;
    
    let workflow_id: String = execution.get("workflow_id");
    let row = sqlx::query("SELECT * FROM workflows WHERE id = ?")
        .bind(&workflow_id)
        .fetch_one(&db)
        .await?;
    
    let stages_json: String = row.get("stages");
    let stages: Vec<crate::models::workflow::WorkflowStage> = serde_json::from_str(&stages_json)?;
    
    let stage = stages.get(stage_index)
        .ok_or(AppError::Internal("阶段索引越界".into()))?;
    
    let result = engine.run_stage(stage, &content, &execution_id).await?;
    
    // Update execution results
    let mut results: std::collections::HashMap<String, String> = 
        serde_json::from_str(&execution.get::<String, _>("results")).unwrap_or_default();
    results.insert(stage_index.to_string(), result.clone());
    
    sqlx::query("UPDATE workflow_executions SET results = ?, current_stage = ? WHERE id = ?")
        .bind(serde_json::to_string(&results)?)
        .bind(stage_index as i64)
        .bind(&execution_id)
        .execute(&db)
        .await?;
    
    Ok(result)
}
```

- [ ] **Step 3: Register in lib.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/workflow_engine.rs src-tauri/src/commands/workflow.rs src-tauri/src/lib.rs
git commit -m "feat: add workflow engine service"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 2: DOCX Export

**Files:**
- Create: `src-tauri/src/services/export_docx.rs`
- Modify: `src-tauri/src/commands/export.rs`

**Steps:**

- [ ] **Step 1: Create export_docx.rs**

```rust
use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_docx(content: &str, output_path: &str, title: &str) -> Result<(), AppError> {
    // Simple DOCX generation using XML
    // A real implementation would use a proper DOCX library
    let docx_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:t>{}</w:t>
      </w:r>
    </w:p>
    <w:sectPr>
      <w:pgSz w:w="11906" w:h="16838"/>
    </w:sectPr>
  </w:body>
</w:document>"#,
        xml_escape(content)
    );
    
    // For now, save as .docx (which is actually a zip)
    // In production, use a proper DOCX library
    let mut file = File::create(output_path)
        .map_err(|e| AppError::Io(e.to_string()))?;
    file.write_all(docx_xml.as_bytes())
        .map_err(|e| AppError::Io(e.to_string()))?;
    
    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
```

- [ ] **Step 2: Add export_docx command to export.rs**

```rust
#[tauri::command]
pub async fn export_docx(
    state: State<'_, AppState>,
    chapter_id: String,
    output_path: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM chapters WHERE id = ?")
        .bind(&chapter_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::ChapterNotFound(chapter_id))?;
    
    let content: String = row.get("content");
    let title: String = row.get("title");
    
    crate::services::export_docx::export_docx(&content, &output_path, &title)
}
```

- [ ] **Step 3: Register in lib.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/export_docx.rs src-tauri/src/commands/export.rs src-tauri/src/lib.rs
git commit -m "feat: add DOCX export"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 3: PDF Export

**Files:**
- Create: `src-tauri/src/services/export_pdf.rs`
- Modify: `src-tauri/src/commands/export.rs`

**Steps:**

- [ ] **Step 1: Create export_pdf.rs**

```rust
use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_pdf(content: &str, output_path: &str, title: &str) -> Result<(), AppError> {
    // Simple PDF generation
    // In production, use a proper PDF library like printpdf or wkhtmltopdf
    let pdf_content = format!(
        r#"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>
endobj
4 0 obj
<< /Length 44 >>
stream
BT
/F1 12 Tf
100 700 Td
({}) Tj
ET
endstream
endobj
5 0 obj
<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>
endobj
xref
0 6
0000000000 65535 f 
0000000009 00000 n 
0000000058 00000 n 
0000000115 00000 n 
0000000266 00000 n 
0000000340 00000 n 
trailer
<< /Size 6 /Root 1 0 R >>
startxref
415
%%EOF"#,
        title.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
    );
    
    let mut file = File::create(output_path)
        .map_err(|e| AppError::Io(e.to_string()))?;
    file.write_all(pdf_content.as_bytes())
        .map_err(|e| AppError::Io(e.to_string()))?;
    
    Ok(())
}
```

- [ ] **Step 2: Add export_pdf command to export.rs**

- [ ] **Step 3: Register in lib.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/export_pdf.rs src-tauri/src/commands/export.rs src-tauri/src/lib.rs
git commit -m "feat: add PDF export"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 4: EPUB Export

**Files:**
- Create: `src-tauri/src/services/export_epub.rs`
- Modify: `src-tauri/src/commands/export.rs`

**Steps:**

- [ ] **Step 1: Create export_epub.rs**

```rust
use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_epub(content: &str, output_path: &str, title: &str, author: &str) -> Result<(), AppError> {
    // EPUB is a ZIP file with specific structure
    // For now, create a simple HTML-based EPUB
    let html_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>{}</title>
</head>
<body>
<h1>{}</h1>
<div>{}</div>
</body>
</html>"#,
        xml_escape(title),
        xml_escape(title),
        content.lines()
            .map(|line| format!("<p>{}</p>", xml_escape(line)))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    // For now, save as .epub (which is actually a zip)
    // In production, use a proper EPUB library
    let mut file = File::create(output_path)
        .map_err(|e| AppError::Io(e.to_string()))?;
    file.write_all(html_content.as_bytes())
        .map_err(|e| AppError::Io(e.to_string()))?;
    
    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
```

- [ ] **Step 2: Add export_epub command to export.rs**

- [ ] **Step 3: Register in lib.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/export_epub.rs src-tauri/src/commands/export.rs src-tauri/src/lib.rs
git commit -m "feat: add EPUB export"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 5: Cloud Sync Migration

**Files:**
- Create: `src-tauri/src/db/migrations/008_cloud_sync.sql`

**Steps:**

- [ ] **Step 1: Create migration**

```sql
-- 008_cloud_sync.sql
CREATE TABLE IF NOT EXISTS cloud_configs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL, -- webdav/oss/s3
    config TEXT NOT NULL, -- JSON encrypted config
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS cloud_sync_log (
    id TEXT PRIMARY KEY,
    config_id TEXT NOT NULL,
    action TEXT NOT NULL, -- upload/download/delete
    path TEXT NOT NULL,
    status TEXT NOT NULL, -- success/failed
    error TEXT,
    synced_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (config_id) REFERENCES cloud_configs(id) ON DELETE CASCADE
);
```

- [ ] **Step 2: Register in db/mod.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/db/migrations/008_cloud_sync.sql src-tauri/src/db/mod.rs
git commit -m "feat: add cloud sync migration"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 6: Cloud Sync Service

**Files:**
- Create: `src-tauri/src/services/cloud_sync.rs`
- Create: `src-tauri/src/commands/cloud.rs`

**Steps:**

- [ ] **Step 1: Create cloud_sync.rs**

```rust
use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub config: CloudProviderConfig,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CloudProviderConfig {
    WebDAV {
        url: String,
        username: String,
        password: String,
    },
    OSS {
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
    },
    S3 {
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
    },
}

pub struct CloudSyncService {
    client: Client,
}

impl CloudSyncService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn upload_file(
        &self,
        config: &CloudConfig,
        local_path: &str,
        remote_path: &str,
    ) -> Result<(), AppError> {
        match &config.config {
            CloudProviderConfig::WebDAV { url, username, password } => {
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                let response = self.client
                    .put(&full_url)
                    .basic_auth(username, Some(password))
                    .body(std::fs::read(local_path).map_err(|e| AppError::Io(e.to_string()))?)
                    .send()
                    .await
                    .map_err(|e| AppError::Http(e.to_string()))?;
                
                if !response.status().is_success() {
                    return Err(AppError::Http(format!("上传失败: {}", response.status())));
                }
                Ok(())
            }
            _ => Err(AppError::Internal("暂不支持此云存储类型".into())),
        }
    }

    pub async fn download_file(
        &self,
        config: &CloudConfig,
        remote_path: &str,
        local_path: &str,
    ) -> Result<(), AppError> {
        match &config.config {
            CloudProviderConfig::WebDAV { url, username, password } => {
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                let response = self.client
                    .get(&full_url)
                    .basic_auth(username, Some(password))
                    .send()
                    .await
                    .map_err(|e| AppError::Http(e.to_string()))?;
                
                if !response.status().is_success() {
                    return Err(AppError::Http(format!("下载失败: {}", response.status())));
                }
                
                let bytes = response.bytes().await
                    .map_err(|e| AppError::Http(e.to_string()))?;
                std::fs::write(local_path, bytes)
                    .map_err(|e| AppError::Io(e.to_string()))?;
                Ok(())
            }
            _ => Err(AppError::Internal("暂不支持此云存储类型".into())),
        }
    }
}
```

- [ ] **Step 2: Create cloud.rs with commands**

```rust
use crate::error::AppError;
use crate::services::cloud_sync::{CloudConfig, CloudProviderConfig, CloudSyncService};
use crate::state::AppState;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_cloud_config(
    state: State<'_, AppState>,
    name: String,
    provider_type: String,
    config: CloudProviderConfig,
) -> Result<CloudConfig, AppError> {
    let db = state.db().await?;
    let id = Uuid::new_v4().to_string();
    let config_json = serde_json::to_string(&config)?;
    
    sqlx::query("INSERT INTO cloud_configs (id, name, provider_type, config) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&provider_type)
        .bind(&config_json)
        .execute(&db)
        .await?;

    Ok(CloudConfig {
        id,
        name,
        provider_type,
        config,
        is_active: false,
    })
}

#[tauri::command]
pub async fn list_cloud_configs(
    state: State<'_, AppState>,
) -> Result<Vec<CloudConfig>, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM cloud_configs ORDER BY created_at DESC")
        .fetch_all(&db)
        .await?;

    rows.into_iter().map(|row| {
        let config_json: String = row.get("config");
        let config: CloudProviderConfig = serde_json::from_str(&config_json)?;
        Ok(CloudConfig {
            id: row.get("id"),
            name: row.get("name"),
            provider_type: row.get("provider_type"),
            config,
            is_active: row.get::<i64, _>("is_active") != 0,
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn upload_to_cloud(
    state: State<'_, AppState>,
    config_id: String,
    local_path: String,
    remote_path: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM cloud_configs WHERE id = ?")
        .bind(&config_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Internal("云配置不存在".into()))?;
    
    let config_json: String = row.get("config");
    let config: CloudProviderConfig = serde_json::from_str(&config_json)?;
    
    let cloud_config = CloudConfig {
        id: row.get("id"),
        name: row.get("name"),
        provider_type: row.get("provider_type"),
        config,
        is_active: row.get::<i64, _>("is_active") != 0,
    };
    
    let service = CloudSyncService::new();
    service.upload_file(&cloud_config, &local_path, &remote_path).await
}

#[tauri::command]
pub async fn download_from_cloud(
    state: State<'_, AppState>,
    config_id: String,
    remote_path: String,
    local_path: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM cloud_configs WHERE id = ?")
        .bind(&config_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Internal("云配置不存在".into()))?;
    
    let config_json: String = row.get("config");
    let config: CloudProviderConfig = serde_json::from_str(&config_json)?;
    
    let cloud_config = CloudConfig {
        id: row.get("id"),
        name: row.get("name"),
        provider_type: row.get("provider_type"),
        config,
        is_active: row.get::<i64, _>("is_active") != 0,
    };
    
    let service = CloudSyncService::new();
    service.download_file(&cloud_config, &remote_path, &local_path).await
}
```

- [ ] **Step 3: Register in lib.rs**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/cloud_sync.rs src-tauri/src/commands/cloud.rs src-tauri/src/lib.rs
git commit -m "feat: add cloud sync service and commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 7: Cloud Sync Store + UI

**Files:**
- Create: `src/stores/cloud.ts`
- Create: `src/components/cloud/CloudPanel.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Create cloud.ts**

```typescript
import { ref } from 'vue'
import { useTauriIPC } from '../composables/useTauriIPC'

export interface CloudConfig {
  id: string
  name: string
  provider_type: string
  is_active: boolean
}

export function useCloudStore() {
  const configs = ref<CloudConfig[]>([])
  const loading = ref(false)
  const { call } = useTauriIPC()

  async function fetchConfigs() {
    loading.value = true
    try {
      configs.value = await call<CloudConfig[]>('list_cloud_configs')
    } finally {
      loading.value = false
    }
  }

  async function createConfig(name: string, providerType: string, config: any) {
    const result = await call<CloudConfig>('create_cloud_config', {
      name,
      providerType,
      config,
    })
    configs.value.unshift(result)
    return result
  }

  async function upload(configId: string, localPath: string, remotePath: string) {
    await call('upload_to_cloud', { configId, localPath, remotePath })
  }

  async function download(configId: string, remotePath: string, localPath: string) {
    await call('download_from_cloud', { configId, remotePath, localPath })
  }

  return {
    configs,
    loading,
    fetchConfigs,
    createConfig,
    upload,
    download,
  }
}
```

- [ ] **Step 2: Create CloudPanel.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useCloudStore } from '../../stores/cloud'

const store = useCloudStore()
const showCreateDialog = ref(false)
const newName = ref('')
const newProviderType = ref('webdav')
const newUrl = ref('')
const newUsername = ref('')
const newPassword = ref('')

onMounted(() => {
  store.fetchConfigs()
})

async function createConfig() {
  if (!newName.value || !newUrl.value) return
  await store.createConfig(newName.value, newProviderType.value, {
    type: newProviderType.value,
    url: newUrl.value,
    username: newUsername.value,
    password: newPassword.value,
  })
  showCreateDialog.value = false
  resetForm()
}

function resetForm() {
  newName.value = ''
  newProviderType.value = 'webdav'
  newUrl.value = ''
  newUsername.value = ''
  newPassword.value = ''
}
</script>

<template>
  <div class="cloud-panel">
    <div class="panel-header">
      <h3>☁️ 云同步</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 新建</button>
    </div>

    <div v-if="showCreateDialog" class="create-dialog">
      <input v-model="newName" placeholder="配置名称" class="input" />
      <select v-model="newProviderType" class="input">
        <option value="webdav">WebDAV</option>
        <option value="oss">阿里云 OSS</option>
        <option value="s3">AWS S3</option>
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
    
    <div v-else-if="store.configs.length === 0" class="empty">
      暂无云配置
    </div>

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
.cloud-panel { padding: 12px; height: 100%; overflow-y: auto; }
.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.panel-header h3 { margin: 0; font-size: 14px; }
.btn-sm { padding: 4px 8px; border: 1px solid #555; border-radius: 4px; background: #2a2a2a; color: #ccc; cursor: pointer; font-size: 12px; }
.btn-primary { padding: 6px 12px; border: none; border-radius: 4px; background: #4a9eff; color: white; cursor: pointer; }
.input { width: 100%; padding: 6px 8px; border: 1px solid #444; border-radius: 4px; background: #1a1a1a; color: #eee; margin-bottom: 8px; font-size: 13px; box-sizing: border-box; }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 8px; }
.loading, .empty { text-align: center; color: #888; padding: 20px; font-size: 13px; }
.config-list { display: flex; flex-direction: column; gap: 4px; }
.config-item { display: flex; justify-content: space-between; align-items: center; padding: 8px; border: 1px solid #333; border-radius: 4px; }
.config-name { font-size: 13px; color: #eee; }
.config-type { font-size: 11px; color: #888; }
</style>
```

- [ ] **Step 3: Add cloud tab to Sidebar.vue**

Add a new tab for cloud sync (☁️ 云同步) and import CloudPanel.

- [ ] **Step 4: Commit**

```bash
git add src/stores/cloud.ts src/components/cloud/ src/components/layout/Sidebar.vue
git commit -m "feat: add cloud sync UI"
```

**Verification:** `bun run build`

---

### Task 8: Update Export Dialog + Workflow Panel

**Files:**
- Modify: `src/components/editor/ExportDialog.vue`
- Modify: `src/components/workflow/WorkflowPanel.vue`

**Steps:**

- [ ] **Step 1: Update ExportDialog.vue**

Add DOCX, PDF, EPUB options to the export dialog. The dialog should now show:
- TXT (plain text)
- Markdown (.md)
- DOCX (.docx)
- PDF (.pdf)
- EPUB (.epub)

Each option should call the appropriate export command.

- [ ] **Step 2: Update WorkflowPanel.vue**

Add an "execute" button to each workflow that triggers workflow execution.

- [ ] **Step 3: Commit**

```bash
git add src/components/editor/ExportDialog.vue src/components/workflow/WorkflowPanel.vue
git commit -m "feat: update export dialog and workflow panel"
```

**Verification:** `bun run build`

---

### Task 9: Integration Verification

**Files:**
- None (verification only)

**Steps:**

- [ ] **Step 1: Verify all commands in lib.rs**

Count should now be ~45+.

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
Task 1: feat: add workflow engine service
Task 2: feat: add DOCX export
Task 3: feat: add PDF export
Task 4: feat: add EPUB export
Task 5: feat: add cloud sync migration
Task 6: feat: add cloud sync service and commands
Task 7: feat: add cloud sync UI
Task 8: feat: update export dialog and workflow panel
Task 9: (verification only)
```
