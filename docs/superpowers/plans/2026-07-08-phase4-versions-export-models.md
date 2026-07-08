# Phase 4: Version History + Export + Model Management

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add chapter version history with rollback, export chapters to TXT/Markdown, and implement model provider management.

**Architecture:** Three independent features: (1) chapter_versions table + CRUD + version diff UI, (2) export command writing files to disk, (3) model_providers table + CRUD + connection testing.

**Tech Stack:** Vue 3.5.39, Pinia 3.0.4, Tauri 2.11, SQLx 0.9, TypeScript 5.7+

---

## Task 1: Database Migration — chapter_versions + model_providers

**Files:**
- Create: `src-tauri/migrations/004_versions_models.sql`

**Interfaces:**
- Consumes: existing `chapters` and `projects` tables
- Produces: `chapter_versions` and `model_providers` tables

- [ ] **Step 1: Create migration**

```sql
-- migrations/004_versions_models.sql

-- Chapter version history
CREATE TABLE IF NOT EXISTS chapter_versions (
    id TEXT PRIMARY KEY,
    chapter_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_versions_chapter ON chapter_versions(chapter_id);
CREATE INDEX IF NOT EXISTS idx_versions_number ON chapter_versions(chapter_id, version_number);

-- Model providers
CREATE TABLE IF NOT EXISTS model_providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    api_url TEXT NOT NULL,
    api_key TEXT,
    model_name TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    config TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_providers_default ON model_providers(is_default);
```

- [ ] **Step 2: Add migration to db/mod.rs**

- [ ] **Step 3: Verify — `cargo check`**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/004_versions_models.sql src-tauri/src/db/mod.rs
git commit -m "feat: add chapter_versions and model_providers tables"
```

---

## Task 2: Chapter Version Model + Commands

**Files:**
- Create: `src-tauri/src/db/models/version.rs`
- Create: `src-tauri/src/commands/version.rs`
- Modify: `src-tauri/src/db/models/mod.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `chapter_versions` table, `chapters` table
- Produces: `save_version`, `list_versions`, `restore_version` commands

- [ ] **Step 1: Create version model**

```rust
// src-tauri/src/db/models/version.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChapterVersion {
    pub id: String,
    pub chapter_id: String,
    pub version_number: i32,
    pub content: String,
    pub word_count: i32,
    pub created_at: String,
}

impl ChapterVersion {
    pub async fn save(
        db: &sqlx::SqlitePool,
        chapter_id: &str,
        content: &str,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let word_count = content.chars().count() as i32;

        // Get next version number
        let max_version: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(version_number) FROM chapter_versions WHERE chapter_id = ?"
        )
        .bind(chapter_id)
        .fetch_optional(db)
        .await?;
        let version_number = max_version.unwrap_or(0) + 1;

        let version = sqlx::query_as::<_, Self>(
            r#"INSERT INTO chapter_versions (id, chapter_id, version_number, content, word_count, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(chapter_id)
        .bind(version_number)
        .bind(content)
        .bind(word_count)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(version)
    }

    pub async fn list_by_chapter(
        db: &sqlx::SqlitePool,
        chapter_id: &str,
    ) -> Result<Vec<Self>, crate::error::AppError> {
        let versions = sqlx::query_as::<_, Self>(
            "SELECT * FROM chapter_versions WHERE chapter_id = ? ORDER BY version_number DESC"
        )
        .bind(chapter_id)
        .fetch_all(db)
        .await?;
        Ok(versions)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let version = sqlx::query_as::<_, Self>("SELECT * FROM chapter_versions WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::InvalidArgument("版本不存在".into()))?;
        Ok(version)
    }
}
```

- [ ] **Step 2: Create version commands**

```rust
// src-tauri/src/commands/version.rs
use crate::db::models::version::ChapterVersion;
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn save_version(
    state: State<'_, AppState>,
    chapter_id: String,
    content: String,
) -> AppResult<ChapterVersion> {
    let db = state.db().await?;
    let version = ChapterVersion::save(&db, &chapter_id, &content).await?;
    Ok(version)
}

#[tauri::command]
pub async fn list_versions(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<Vec<ChapterVersion>> {
    let db = state.db().await?;
    let versions = ChapterVersion::list_by_chapter(&db, &chapter_id).await?;
    Ok(versions)
}

#[tauri::command]
pub async fn restore_version(
    state: State<'_, AppState>,
    version_id: String,
) -> AppResult<String> {
    let db = state.db().await?;
    let version = ChapterVersion::find_by_id(&db, &version_id).await?;
    // Update chapter content
    sqlx::query("UPDATE chapters SET content = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&version.content)
        .bind(&version.chapter_id)
        .execute(&db)
        .await?;
    Ok(version.content)
}
```

- [ ] **Step 3: Register modules and commands**

- [ ] **Step 4: Verify — `cargo check`**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/db/models/version.rs src-tauri/src/commands/version.rs src-tauri/src/db/models/mod.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add chapter version history commands"
```

---

## Task 3: Version History UI

**Files:**
- Create: `src/components/editor/VersionHistory.vue`
- Modify: `src/components/layout/EditorPanel.vue`

**Interfaces:**
- Consumes: `useTauriIPC`, chapter data
- Produces: version list panel, restore button, version diff view

- [ ] **Step 1: Create VersionHistory component**

Show version list with:
- Version number, timestamp, word count
- Click to view version content
- "恢复此版本" button
- Comparison view (current vs selected)

- [ ] **Step 2: Add version history toggle to EditorPanel**

Add a button in the editor toolbar to toggle the version history panel.

- [ ] **Step 3: Verify — `bun run build`**

- [ ] **Step 4: Commit**

```bash
git add src/components/editor/VersionHistory.vue src/components/layout/EditorPanel.vue
git commit -m "feat: add version history UI"
```

---

## Task 4: Model Provider Model + Commands

**Files:**
- Create: `src-tauri/src/db/models/model_provider.rs`
- Create: `src-tauri/src/commands/model.rs`
- Modify: `src-tauri/src/db/models/mod.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `model_providers` table
- Produces: CRUD commands + `test_connection` command

- [ ] **Step 1: Create model provider model**

```rust
// src-tauri/src/db/models/model_provider.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModelProvider {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub is_default: bool,
    pub config: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub is_default: Option<bool>,
    pub config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub is_default: Option<bool>,
    pub config: Option<String>,
}
```

- [ ] **Step 2: Create model commands**

5 commands: `create_provider`, `list_providers`, `update_provider`, `delete_provider`, `test_connection`

`test_connection` will make a simple HTTP request to the API URL with the API key to verify connectivity.

- [ ] **Step 3: Register modules and commands**

- [ ] **Step 4: Verify — `cargo check`**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/db/models/model_provider.rs src-tauri/src/commands/model.rs src-tauri/src/db/models/mod.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add model provider management commands"
```

---

## Task 5: Model Management UI

**Files:**
- Create: `src/components/settings/ModelManager.vue`
- Create: `src/stores/model.ts`
- Modify: `src/components/layout/Sidebar.vue`

**Interfaces:**
- Consumes: model provider commands
- Produces: model list, add/edit/delete/test connection UI

- [ ] **Step 1: Create model store**

```typescript
// src/stores/model.ts
import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface ModelProvider {
  id: string;
  name: string;
  provider_type: string;
  api_url: string;
  api_key: string | null;
  model_name: string;
  is_default: boolean;
  config: string | null;
  created_at: string;
  updated_at: string;
}

export const useModelStore = defineStore("model", () => {
  const { call } = useTauriIPC();
  const providers = ref<ModelProvider[]>([]);
  const loading = ref(false);

  const fetchProviders = async () => {
    loading.value = true;
    try {
      providers.value = await call<ModelProvider[]>("list_providers");
    } finally {
      loading.value = false;
    }
  };

  const createProvider = async (params: {
    name: string;
    provider_type: string;
    api_url: string;
    api_key?: string;
    model_name: string;
    is_default?: boolean;
  }) => {
    const provider = await call<ModelProvider>("create_provider", params);
    providers.value.push(provider);
    return provider;
  };

  const updateProvider = async (id: string, params: Record<string, unknown>) => {
    const provider = await call<ModelProvider>("update_provider", { id, ...params });
    const idx = providers.value.findIndex((p) => p.id === id);
    if (idx !== -1) providers.value[idx] = provider;
  };

  const deleteProvider = async (id: string) => {
    await call("delete_provider", { id });
    providers.value = providers.value.filter((p) => p.id !== id);
  };

  const testConnection = async (id: string) => {
    return await call<{ success: boolean; message: string }>("test_connection", { id });
  };

  return { providers, loading, fetchProviders, createProvider, updateProvider, deleteProvider, testConnection };
});
```

- [ ] **Step 2: Create ModelManager component**

Show model providers with:
- List of configured models
- Add new model dialog (name, type dropdown, URL, API key, model name)
- Edit/delete buttons
- Test connection button with status indicator
- Set as default button

- [ ] **Step 3: Add model settings tab to Sidebar**

- [ ] **Step 4: Verify — `bun run build`**

- [ ] **Step 5: Commit**

```bash
git add src/components/settings/ModelManager.vue src/stores/model.ts src/components/layout/Sidebar.vue
git commit -m "feat: add model management UI"
```

---

## Task 6: Export Command + UI

**Files:**
- Create: `src-tauri/src/commands/export.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Create: `src/components/editor/ExportDialog.vue`

**Interfaces:**
- Consumes: chapter data, project data
- Produces: `export_chapter` command writing to disk, export dialog UI

- [ ] **Step 1: Create export command**

```rust
// src-tauri/src/commands/export.rs
use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub file_path: String,
    pub file_size: u64,
}

#[tauri::command]
pub async fn export_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
    format: String,
    output_path: String,
) -> AppResult<ExportResult> {
    let db = state.db().await?;

    // Get chapter
    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;

    // Get project for title
    let project = crate::db::models::project::Project::find_by_id(&db, &chapter.project_id).await?;

    let content = match format.as_str() {
        "txt" => format!("{}\n\n{}", chapter.title, chapter.content),
        "md" => format!("# {}\n\n{}", chapter.title, chapter.content),
        _ => return Err(crate::error::AppError::InvalidArgument("不支持的导出格式".into())),
    };

    let file_name = format!("{}_{}.{}", project.name, chapter.title, format);
    let file_path = std::path::Path::new(&output_path).join(&file_name);
    std::fs::write(&file_path, content)?;

    let file_size = std::fs::metadata(&file_path)?.len();

    Ok(ExportResult {
        file_path: file_path.to_string_lossy().to_string(),
        file_size,
    })
}

#[tauri::command]
pub async fn export_all_chapters(
    state: State<'_, AppState>,
    project_id: String,
    format: String,
    output_path: String,
) -> AppResult<Vec<ExportResult>> {
    let db = state.db().await?;
    let chapters = crate::db::models::chapter::Chapter::list_by_project(&db, &project_id).await?;
    let project = crate::db::models::project::Project::find_by_id(&db, &project_id).await?;

    let mut results = Vec::new();
    for chapter in &chapters {
        let content = match format.as_str() {
            "txt" => format!("{}\n\n{}", chapter.title, chapter.content),
            "md" => format!("# {}\n\n{}", chapter.title, chapter.content),
            _ => continue,
        };

        let file_name = format!("{}_{}.{}", project.name, chapter.title, format);
        let file_path = std::path::Path::new(&output_path).join(&file_name);
        std::fs::write(&file_path, &content)?;

        let file_size = std::fs::metadata(&file_path)?.len();
        results.push(ExportResult {
            file_path: file_path.to_string_lossy().to_string(),
            file_size,
        });
    }

    Ok(results)
}
```

- [ ] **Step 2: Create ExportDialog component**

Show export options:
- Format selector (TXT, Markdown)
- Output directory picker (using Tauri dialog)
- Export single chapter or all chapters
- Progress indicator

- [ ] **Step 3: Add export button to EditorPanel toolbar**

- [ ] **Step 4: Verify — `bun run build`**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/export.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src/components/editor/ExportDialog.vue src/components/layout/EditorPanel.vue
git commit -m "feat: add chapter export functionality"
```

---

## Task 7: Integration Verification

**Files:**
- Verify all files from Tasks 1-6 exist
- Verify all commands registered

**Interfaces:**
- Consumes: all previous tasks
- Produces: verified integration

- [ ] **Step 1: Verify all files exist**

- [ ] **Step 2: Verify `cargo check && bun run build`**

- [ ] **Step 3: Verify user flow**

1. Open chapter → version history button → see versions
2. Save version → appears in list
3. Restore version → chapter content reverts
4. Export chapter → file created on disk
5. Model management → add/edit/test connection

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: Phase 4 complete — versions + export + model management"
```
