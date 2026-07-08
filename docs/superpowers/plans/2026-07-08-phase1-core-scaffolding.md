# Novel IDE Phase 1: Core Scaffolding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bootstrap the Novel IDE project with Tauri 2, Vue 3, SQLite database, basic IDE layout, and project management.

**Architecture:** Tauri 2 desktop app with Rust backend (SQLx + SQLite) and Vue 3 + TypeScript frontend. Monaco Editor for text editing. Panel-based IDE layout with file tree, editor, and AI chat panels.

**Tech Stack:** Rust 1.90+, Tauri 2.11, SQLx 0.9.0, LanceDB 0.31.0, Vue 3.5, Pinia 3.0, TypeScript 5.5, Monaco Editor 0.55, Bun 1.2+

## Global Constraints

- Rust edition: 2024, MSRV: 1.90+
- Tauri: 2.11.x (latest stable)
- SQLx: 0.9.0 with `sqlite` feature
- LanceDB: 0.31.0
- Vue: 3.5.x (stable)
- Pinia: 3.0.x
- TypeScript: 5.5+ strict mode
- Monaco Editor: 0.55.x
- Bun: 1.2+ as package manager
- All Chinese labels in Mermaid diagrams must be quoted
- SQLite per-project isolation (each project = one `novel.db`)
- API keys encrypted with AES-256

---

## File Structure

```
Novel-Ide/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   ├── src/
│   │   ├── main.rs              # Tauri entry point
│   │   ├── lib.rs               # Module declarations
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── project.rs       # Project CRUD commands
│   │   │   └── settings.rs      # Settings commands
│   │   ├── db/
│   │   │   ├── mod.rs           # DB connection pool
│   │   │   ├── migrations.rs    # Schema migrations
│   │   │   └── models/
│   │   │       ├── mod.rs
│   │   │       ├── project.rs   # Project model
│   │   │       └── settings.rs  # Settings model
│   │   ├── error.rs             # Unified error type
│   │   └── state.rs             # Tauri managed state
│   └── migrations/
│       └── 001_initial.sql      # Initial schema
├── src/
│   ├── App.vue                  # Root component
│   ├── main.ts                  # Vue entry
│   ├── stores/
│   │   ├── project.ts           # Project store
│   │   └── settings.ts          # Settings store
│   ├── components/
│   │   ├── layout/
│   │   │   ├── IDELayout.vue    # Main 4-panel layout
│   │   │   ├── TitleBar.vue     # Custom title bar
│   │   │   ├── Sidebar.vue      # Left sidebar (file tree)
│   │   │   ├── EditorPanel.vue  # Center editor panel
│   │   │   ├── AIPanel.vue      # Right AI chat panel
│   │   │   └── BottomPanel.vue  # Bottom terminal/output
│   │   ├── editor/
│   │   │   └── MonacoEditor.vue # Monaco Editor wrapper
│   │   ├── project/
│   │   │   ├── ProjectList.vue  # Project list/home
│   │   │   └── NewProject.vue   # New project dialog
│   │   └── common/
│   │       ├── Panel.vue        # Resizable panel
│   │       └── Icon.vue         # Icon component
│   ├── composables/
│   │   ├── usePanelResize.ts    # Panel resize logic
│   │   └── useTauriIPC.ts       # Tauri invoke wrapper
│   └── styles/
│       ├── main.css             # Global styles
│       └── variables.css        # CSS variables (theme)
├── package.json
├── tsconfig.json
├── vite.config.ts
└── index.html
```

---

### Task 1: Initialize Tauri 2 Project

**Files:**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/build.rs`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `package.json`
- Create: `tsconfig.json`
- Create: `vite.config.ts`
- Create: `index.html`
- Create: `src/main.ts`
- Create: `src/App.vue`

**Interfaces:**
- Consumes: None (first task)
- Produces: Running Tauri 2 app with Vue 3 frontend

- [ ] **Step 1: Create Cargo.toml**

```toml
# src-tauri/Cargo.toml
[package]
name = "novel-ide"
version = "0.1.0"
edition = "2024"
rust-version = "1.90"

[dependencies]
tauri = { version = "2.11", features = ["tray-icon"] }
tauri-plugin-shell = "2.3"
tauri-plugin-dialog = "2.5"
tauri-plugin-fs = "2.2"
tauri-plugin-store = "2.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.45", features = ["full"] }
sqlx = { version = "0.9", features = ["runtime-tokio", "sqlite", "uuid"] }
uuid = { version = "1.11", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2.0"
log = "0.4"
env_logger = "0.11"

[build-dependencies]
tauri-build = { version = "2.6", features = [] }

[lib]
name = "novel_ide_lib"
crate-type = ["lib", "cdylib", "staticlib"]
```

- [ ] **Step 2: Create tauri.conf.json**

```json
{
  "$schema": "https://raw.githubusercontent.com/nicbarker/tauri-v2/main/core/tauri-config-schema/schema.json",
  "productName": "Novel IDE",
  "version": "0.1.0",
  "identifier": "com.novel-ide.app",
  "build": {
    "beforeDevCommand": "bun run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "bun run build",
    "frontendDist": "../dist"
  },
  "app": {
    "title": "Novel IDE",
    "windows": [
      {
        "title": "Novel IDE",
        "width": 1400,
        "height": 900,
        "minWidth": 1024,
        "minHeight": 680,
        "decorations": false,
        "resizable": true,
        "center": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "plugins": {
    "shell": {
      "open": true
    },
    "fs": {
      "scope": {
        "allow": ["**"],
        "deny": []
      }
    }
  }
}
```

- [ ] **Step 3: Create build.rs**

```rust
// src-tauri/build.rs
fn main() {
    tauri_build::build()
}
```

- [ ] **Step 4: Create main.rs**

```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    novel_ide_lib::run()
}
```

- [ ] **Step 5: Create lib.rs**

```rust
// src-tauri/src/lib.rs
mod commands;
mod db;
mod error;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState::default())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db = db::init_database(&handle).await?;
                let state = app.state::<AppState>();
                state.set_db(db).await;
                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::open_project,
            commands::project::delete_project,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: Create package.json**

```json
{
  "name": "novel-ide",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "vue": "^3.5.39",
    "pinia": "^3.0.4",
    "@tauri-apps/api": "^2.2.0",
    "@tauri-apps/plugin-shell": "^2.3.5",
    "@tauri-apps/plugin-dialog": "^2.7.1",
    "@tauri-apps/plugin-fs": "^2.2.0",
    "@tauri-apps/plugin-store": "^2.2.0",
    "monaco-editor": "^0.55.1"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.2.0",
    "typescript": "^5.7.0",
    "vite": "^6.3.0",
    "vue-tsc": "^2.2.0",
    "@tauri-apps/cli": "^2.2.0"
  }
}
```

- [ ] **Step 7: Create vite.config.ts**

```typescript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 5174,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

- [ ] **Step 8: Create tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2021",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2021", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 9: Create index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Novel IDE</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

- [ ] **Step 10: Create src/main.ts**

```typescript
import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./styles/main.css";

const app = createApp(App);
app.use(createPinia());
app.mount("#app");
```

- [ ] **Step 11: Create src/App.vue**

```vue
<script setup lang="ts">
import IDELayout from "./components/layout/IDELayout.vue";
</script>

<template>
  <IDELayout />
</template>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
}
</style>
```

- [ ] **Step 12: Install dependencies and verify**

Run: `bun install`
Run: `cd src-tauri && cargo check`
Expected: Both complete without errors

- [ ] **Step 13: Commit**

```bash
git init
echo "target/\nnode_modules/\ndist/\n.vite/" > .gitignore
git add -A
git commit -m "feat: initialize Tauri 2 + Vue 3 project scaffolding"
```

---

### Task 2: Database Layer

**Files:**
- Create: `src-tauri/src/db/mod.rs`
- Create: `src-tauri/src/db/migrations.rs`
- Create: `src-tauri/src/db/models/mod.rs`
- Create: `src-tauri/src/db/models/project.rs`
- Create: `src-tauri/src/db/models/settings.rs`
- Create: `src-tauri/migrations/001_initial.sql`
- Create: `src-tauri/src/error.rs`
- Create: `src-tauri/src/state.rs`

**Interfaces:**
- Consumes: Task 1 (Tauri app running)
- Produces: `DbPool` type, `Project` model, `init_database()` function

- [ ] **Step 1: Create error.rs**

```rust
// src-tauri/src/error.rs
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("项目不存在: {0}")]
    ProjectNotFound(String),

    #[error("参数错误: {0}")]
    InvalidArgument(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
```

- [ ] **Step 2: Create state.rs**

```rust
// src-tauri/src/state.rs
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    db: Arc<RwLock<Option<SqlitePool>>>,
}

impl AppState {
    pub async fn set_db(&self, pool: SqlitePool) {
        *self.db.write().await = Some(pool);
    }

    pub async fn db(&self) -> Result<SqlitePool, crate::error::AppError> {
        self.db
            .read()
            .await
            .clone()
            .ok_or_else(|| crate::error::AppError::Internal("数据库未初始化".into()))
    }
}
```

- [ ] **Step 3: Create migrations/001_initial.sql**

```sql
-- migrations/001_initial.sql
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    genre TEXT,
    sub_genre TEXT,
    target_readers TEXT,
    total_chapters INTEGER DEFAULT 0,
    words_per_chapter INTEGER DEFAULT 3000,
    narrative_pov TEXT,
    story_structure TEXT,
    core_outline TEXT,
    world_settings TEXT,
    character_profiles TEXT,
    golden_finger TEXT,
    writing_constraints TEXT,
    style_constraints TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);

CREATE TABLE IF NOT EXISTS project_settings (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, key)
);

CREATE INDEX IF NOT EXISTS idx_settings_project ON project_settings(project_id);

CREATE TABLE IF NOT EXISTS global_settings (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

- [ ] **Step 4: Create db/mod.rs**

```rust
// src-tauri/src/db/mod.rs
pub mod models;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tauri::AppHandle;
use tauri::PathResolver;

pub async fn init_database(app: &AppHandle) -> Result<SqlitePool, crate::error::AppError> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("novel-ide.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run migrations
    sqlx::query(include_str!("../../migrations/001_initial.sql"))
        .execute(&pool)
        .await?;

    log::info!("数据库初始化完成: {}", db_path.display());

    Ok(pool)
}
```

- [ ] **Step 5: Create db/models/mod.rs**

```rust
// src-tauri/src/db/models/mod.rs
pub mod project;
pub mod settings;

pub use project::Project;
pub use settings::{GlobalSetting, ProjectSetting};
```

- [ ] **Step 6: Create db/models/project.rs**

```rust
// src-tauri/src/db/models/project.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub genre: Option<String>,
    pub sub_genre: Option<String>,
    pub target_readers: Option<String>,
    pub total_chapters: Option<i32>,
    pub words_per_chapter: Option<i32>,
    pub narrative_pov: Option<String>,
    pub story_structure: Option<String>,
    pub core_outline: Option<String>,
    pub world_settings: Option<String>,
    pub character_profiles: Option<String>,
    pub golden_finger: Option<String>,
    pub writing_constraints: Option<String>,
    pub style_constraints: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub path: String,
    pub genre: Option<String>,
    pub sub_genre: Option<String>,
    pub target_readers: Option<String>,
    pub total_chapters: Option<i32>,
    pub words_per_chapter: Option<i32>,
    pub narrative_pov: Option<String>,
    pub story_structure: Option<String>,
}

impl Project {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateProjectRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let project = sqlx::query_as::<_, Self>(
            r#"INSERT INTO projects (id, name, path, genre, sub_genre, target_readers,
                total_chapters, words_per_chapter, narrative_pov, story_structure,
                created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&req.path)
        .bind(&req.genre)
        .bind(&req.sub_genre)
        .bind(&req.target_readers)
        .bind(req.total_chapters)
        .bind(req.words_per_chapter.unwrap_or(3000))
        .bind(&req.narrative_pov)
        .bind(&req.story_structure)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(project)
    }

    pub async fn list_all(db: &sqlx::SqlitePool) -> Result<Vec<Self>, crate::error::AppError> {
        let projects = sqlx::query_as::<_, Self>("SELECT * FROM projects ORDER BY updated_at DESC")
            .fetch_all(db)
            .await?;
        Ok(projects)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let project = sqlx::query_as::<_, Self>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::ProjectNotFound(id.to_string()))?;
        Ok(project)
    }

    pub async fn delete(db: &sqlx::SqlitePool, id: &str) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::ProjectNotFound(id.to_string()));
        }
        Ok(())
    }
}
```

- [ ] **Step 7: Create db/models/settings.rs**

```rust
// src-tauri/src/db/models/settings.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GlobalSetting {
    pub key: String,
    pub value: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectSetting {
    pub id: String,
    pub project_id: String,
    pub key: String,
    pub value: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl GlobalSetting {
    pub async fn get(
        db: &sqlx::SqlitePool,
        key: &str,
    ) -> Result<Option<String>, crate::error::AppError> {
        let row = sqlx::query_as::<_, Self>("SELECT * FROM global_settings WHERE key = ?")
            .bind(key)
            .fetch_optional(db)
            .await?;
        Ok(row.and_then(|r| r.value))
    }

    pub async fn set(
        db: &sqlx::SqlitePool,
        key: &str,
        value: &str,
    ) -> Result<(), crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            r#"INSERT INTO global_settings (key, value, updated_at) VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at"#,
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_all(db: &sqlx::SqlitePool) -> Result<Vec<Self>, crate::error::AppError> {
        let settings = sqlx::query_as::<_, Self>("SELECT * FROM global_settings")
            .fetch_all(db)
            .await?;
        Ok(settings)
    }
}
```

- [ ] **Step 8: Verify database layer compiles**

Run: `cd src-tauri && cargo check`
Expected: Compiles without errors

- [ ] **Step 9: Commit**

```bash
git add src-tauri/src/db/ src-tauri/src/error.rs src-tauri/src/state.rs src-tauri/migrations/
git commit -m "feat: add SQLite database layer with project and settings models"
```

---

### Task 3: Tauri Commands (Project Management)

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/project.rs`
- Create: `src-tauri/src/commands/settings.rs`

**Interfaces:**
- Consumes: Task 2 (db models, state)
- Produces: `create_project`, `list_projects`, `open_project`, `delete_project`, `get_settings`, `update_settings` commands

- [ ] **Step 1: Create commands/mod.rs**

```rust
// src-tauri/src/commands/mod.rs
pub mod project;
pub mod settings;
```

- [ ] **Step 2: Create commands/project.rs**

```rust
// src-tauri/src/commands/project.rs
use crate::db::models::project::{CreateProjectRequest, Project};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    name: String,
    path: String,
    genre: Option<String>,
    sub_genre: Option<String>,
    target_readers: Option<String>,
    total_chapters: Option<i32>,
    words_per_chapter: Option<i32>,
    narrative_pov: Option<String>,
    story_structure: Option<String>,
) -> AppResult<Project> {
    let db = state.db().await?;

    // Validate project name
    if name.is_empty() || name.len() > 50 {
        return Err(crate::error::AppError::InvalidArgument(
            "项目名称必须在 1-50 字之间".into(),
        ));
    }

    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(crate::error::AppError::InvalidArgument(
            "项目名称包含非法字符".into(),
        ));
    }

    // Create project directory
    let project_dir = std::path::Path::new(&path).join(&name);
    std::fs::create_dir_all(&project_dir)?;

    // Create subdirectories
    for dir in &["chapters", "drafts", "final", "assets", "prompts", "hooks", "skills", "references", "rag", "export", "logs"] {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    let req = CreateProjectRequest {
        name,
        path: project_dir.to_string_lossy().to_string(),
        genre,
        sub_genre,
        target_readers,
        total_chapters,
        words_per_chapter,
        narrative_pov,
        story_structure,
    };

    let project = Project::create(&db, &req).await?;
    Ok(project)
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> AppResult<Vec<Project>> {
    let db = state.db().await?;
    let projects = Project::list_all(&db).await?;
    Ok(projects)
}

#[tauri::command]
pub async fn open_project(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Project> {
    let db = state.db().await?;
    let project = Project::find_by_id(&db, &project_id).await?;
    Ok(project)
}

#[tauri::command]
pub async fn delete_project(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<()> {
    let db = state.db().await?;

    // Get project path before deletion
    let project = Project::find_by_id(&db, &project_id).await?;

    // Delete from database
    Project::delete(&db, &project_id).await?;

    // Delete project directory
    let _ = std::fs::remove_dir_all(&project.path);

    Ok(())
}
```

- [ ] **Step 3: Create commands/settings.rs**

```rust
// src-tauri/src/commands/settings.rs
use crate::db::models::settings::GlobalSetting;
use crate::error::AppResult;
use crate::state::AppState;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> AppResult<Vec<GlobalSetting>> {
    let db = state.db().await?;
    let settings = GlobalSetting::get_all(&db).await?;
    Ok(settings)
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    key: String,
    value: Value,
) -> AppResult<()> {
    let db = state.db().await?;
    let value_str = serde_json::to_string(&value)?;
    GlobalSetting::set(&db, &key, &value_str).await?;
    Ok(())
}
```

- [ ] **Step 4: Verify commands compile**

Run: `cd src-tauri && cargo check`
Expected: Compiles without errors

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/
git commit -m "feat: add Tauri commands for project management and settings"
```

---

### Task 4: IDE Layout Components

**Files:**
- Create: `src/components/layout/IDELayout.vue`
- Create: `src/components/layout/TitleBar.vue`
- Create: `src/components/layout/Sidebar.vue`
- Create: `src/components/layout/EditorPanel.vue`
- Create: `src/components/layout/AIPanel.vue`
- Create: `src/components/layout/BottomPanel.vue`
- Create: `src/components/common/Panel.vue`
- Create: `src/composables/usePanelResize.ts`
- Create: `src/styles/main.css`
- Create: `src/styles/variables.css`

**Interfaces:**
- Consumes: Task 1 (Vue app)
- Produces: IDELayout component with 4-panel resizable layout

- [ ] **Step 1: Create variables.css**

```css
/* src/styles/variables.css */
:root {
  /* Colors - Dark Theme */
  --bg-primary: #1e1e2e;
  --bg-secondary: #181825;
  --bg-tertiary: #11111b;
  --bg-surface: #313244;
  --bg-hover: #45475a;
  --bg-active: #585b70;

  --text-primary: #cdd6f4;
  --text-secondary: #a6adc8;
  --text-muted: #6c7086;

  --accent: #89b4fa;
  --accent-hover: #74c7ec;
  --success: #a6e3a1;
  --warning: #f9e2af;
  --error: #f38ba8;
  --info: #89dceb;

  --border: #313244;
  --border-focus: #89b4fa;

  /* Spacing */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 12px;
  --spacing-lg: 16px;
  --spacing-xl: 24px;

  /* Font */
  --font-mono: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
  --font-sans: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
  --font-size-sm: 12px;
  --font-size-md: 14px;
  --font-size-lg: 16px;

  /* Panel */
  --panel-min-width: 200px;
  --panel-min-height: 150px;
  --titlebar-height: 36px;
  --statusbar-height: 24px;

  /* Shadows */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.5);
}
```

- [ ] **Step 2: Create main.css**

```css
/* src/styles/main.css */
@import "./variables.css";

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-md);
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-secondary);
}

::-webkit-scrollbar-thumb {
  background: var(--bg-surface);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--bg-hover);
}

::selection {
  background: var(--accent);
  color: var(--bg-primary);
}
```

- [ ] **Step 3: Create usePanelResize.ts**

```typescript
// src/composables/usePanelResize.ts
import { ref, onMounted, onUnmounted } from "vue";

export function usePanelResize(
  containerRef: Ref<HTMLElement | null>,
  direction: "horizontal" | "vertical" = "horizontal"
) {
  const panels = ref<number[]>([300, 600, 300]);
  const isDragging = ref(false);
  const dragIndex = ref(-1);

  const onMouseDown = (index: number, e: MouseEvent) => {
    isDragging.value = true;
    dragIndex.value = index;
    e.preventDefault();
  };

  const onMouseMove = (e: MouseEvent) => {
    if (!isDragging.value || !containerRef.value) return;

    const rect = containerRef.value.getBoundingClientRect();
    const pos = direction === "horizontal" ? e.clientX - rect.left : e.clientY - rect.top;

    const total = panels.value.reduce((a, b) => a + b, 0);
    const before = panels.value.slice(0, dragIndex.value).reduce((a, b) => a + b, 0);

    const minWidth = 200;
    const newWidth = Math.max(minWidth, Math.min(pos - before, total - before - minWidth * (panels.value.length - dragIndex.value)));

    panels.value[dragIndex.value] = newWidth;
  };

  const onMouseUp = () => {
    isDragging.value = false;
    dragIndex.value = -1;
  };

  onMounted(() => {
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
  });

  onUnmounted(() => {
    window.removeEventListener("mousemove", onMouseMove);
    window.removeEventListener("mouseup", onMouseUp);
  });

  return { panels, isDragging, onMouseDown };
}
```

- [ ] **Step 4: Create Panel.vue**

```vue
<!-- src/components/common/Panel.vue -->
<script setup lang="ts">
defineProps<{
  title?: string;
  width?: number;
  minWidth?: number;
}>();
</script>

<template>
  <div class="panel" :style="{ width: width ? `${width}px` : undefined }">
    <div v-if="title" class="panel-header">
      <span class="panel-title">{{ title }}</span>
    </div>
    <div class="panel-content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 var(--spacing-md);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  user-select: none;
}

.panel-title {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.panel-content {
  flex: 1;
  overflow: auto;
}
</style>
```

- [ ] **Step 5: Create TitleBar.vue**

```vue
<!-- src/components/layout/TitleBar.vue -->
<script setup lang="ts">
import { ref } from "vue";

const isMaximized = ref(false);

const minimize = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  getCurrentWindow().minimize();
};

const toggleMaximize = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  const win = getCurrentWindow();
  if (isMaximized.value) {
    await win.unmaximize();
  } else {
    await win.maximize();
  }
  isMaximized.value = !isMaximized.value;
};

const close = async () => {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  getCurrentWindow().close();
};
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-title" data-tauri-drag-region>
      <span class="app-icon">📖</span>
      <span>Novel IDE</span>
    </div>
    <div class="titlebar-controls">
      <button class="titlebar-btn" @click="minimize">
        <svg width="12" height="12" viewBox="0 0 12 12"><rect y="5" width="12" height="1" fill="currentColor" /></svg>
      </button>
      <button class="titlebar-btn" @click="toggleMaximize">
        <svg v-if="!isMaximized" width="12" height="12" viewBox="0 0 12 12"><rect x="1" y="1" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1" /></svg>
        <svg v-else width="12" height="12" viewBox="0 0 12 12"><rect x="3" y="3" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1" /><rect y="0" width="4" height="4" fill="none" stroke="currentColor" stroke-width="1" /></svg>
      </button>
      <button class="titlebar-btn titlebar-btn-close" @click="close">
        <svg width="12" height="12" viewBox="0 0 12 12"><line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" /><line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" /></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: var(--titlebar-height);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  user-select: none;
}

.titlebar-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding-left: var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.app-icon {
  font-size: 16px;
}

.titlebar-controls {
  display: flex;
  height: 100%;
}

.titlebar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 100%;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s;
}

.titlebar-btn:hover {
  background: var(--bg-hover);
}

.titlebar-btn-close:hover {
  background: var(--error);
  color: white;
}
</style>
```

- [ ] **Step 6: Create Sidebar.vue**

```vue
<!-- src/components/layout/Sidebar.vue -->
<script setup lang="ts">
import { ref } from "vue";

const activeTab = ref<"files" | "search" | "settings">("files");

const tabs = [
  { id: "files" as const, icon: "📁", label: "文件" },
  { id: "search" as const, icon: "🔍", label: "搜索" },
  { id: "settings" as const, icon: "⚙️", label: "设置" },
];
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
        :title="tab.label"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
      </button>
    </div>
    <div class="sidebar-content">
      <div v-if="activeTab === 'files'" class="tab-panel">
        <div class="panel-header-sm">资源管理器</div>
        <div class="empty-state">未打开项目</div>
      </div>
      <div v-else-if="activeTab === 'search'" class="tab-panel">
        <div class="panel-header-sm">全局搜索</div>
        <div class="empty-state">输入关键词搜索</div>
      </div>
      <div v-else-if="activeTab === 'settings'" class="tab-panel">
        <div class="panel-header-sm">设置</div>
        <div class="empty-state">设置面板</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  height: 100%;
  background: var(--bg-secondary);
}

.sidebar-tabs {
  display: flex;
  flex-direction: column;
  width: 48px;
  background: var(--bg-tertiary);
  border-right: 1px solid var(--border);
}

.tab-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 48px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  background: var(--bg-hover);
}

.tab-btn.active {
  border-left-color: var(--accent);
  background: var(--bg-secondary);
}

.tab-icon {
  font-size: 20px;
}

.sidebar-content {
  flex: 1;
  overflow: auto;
}

.tab-panel {
  padding: var(--spacing-md);
}

.panel-header-sm {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--spacing-md);
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
  text-align: center;
  padding: var(--spacing-xl);
}
</style>
```

- [ ] **Step 7: Create EditorPanel.vue**

```vue
<!-- src/components/layout/EditorPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const tabs = ref<{ id: string; name: string; active: boolean }[]>([]);
const activeTabId = ref<string | null>(null);

const openTab = (id: string, name: string) => {
  const exists = tabs.value.find((t) => t.id === id);
  if (!exists) {
    tabs.value.push({ id, name, active: true });
  }
  tabs.value.forEach((t) => (t.active = t.id === id));
  activeTabId.value = id;
};

const closeTab = (id: string) => {
  tabs.value = tabs.value.filter((t) => t.id !== id);
  if (activeTabId.value === id) {
    activeTabId.value = tabs.value.length > 0 ? tabs.value[tabs.value.length - 1].id : null;
  }
};
</script>

<template>
  <div class="editor-panel">
    <div class="editor-tabs" v-if="tabs.length > 0">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        :class="['editor-tab', { active: tab.active }]"
        @click="openTab(tab.id, tab.name)"
      >
        <span class="tab-name">{{ tab.name }}</span>
        <button class="tab-close" @click.stop="closeTab(tab.id)">×</button>
      </div>
    </div>
    <div class="editor-content">
      <div v-if="!activeTabId" class="welcome-screen">
        <div class="welcome-icon">📖</div>
        <h2>Novel IDE</h2>
        <p>专业小说创作 IDE</p>
        <div class="welcome-actions">
          <button class="action-btn">新建项目</button>
          <button class="action-btn secondary">打开项目</button>
        </div>
      </div>
      <div v-else class="editor-placeholder">
        <div class="monaco-container" :id="`editor-${activeTabId}`"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.editor-tabs {
  display: flex;
  height: 36px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  overflow-x: auto;
}

.editor-tab {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 0 var(--spacing-md);
  height: 100%;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  transition: all 0.15s;
}

.editor-tab:hover {
  background: var(--bg-surface);
}

.editor-tab.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-bottom: 2px solid var(--accent);
}

.tab-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: none;
  border: none;
  border-radius: 3px;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 14px;
}

.tab-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.editor-content {
  flex: 1;
  overflow: hidden;
}

.welcome-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: var(--spacing-lg);
}

.welcome-icon {
  font-size: 64px;
}

.welcome-screen h2 {
  font-size: 24px;
  font-weight: 500;
  color: var(--text-primary);
}

.welcome-screen p {
  color: var(--text-muted);
}

.welcome-actions {
  display: flex;
  gap: var(--spacing-md);
  margin-top: var(--spacing-lg);
}

.action-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  font-size: var(--font-size-md);
  cursor: pointer;
  transition: opacity 0.15s;
}

.action-btn:hover {
  opacity: 0.9;
}

.action-btn.secondary {
  background: var(--bg-surface);
  color: var(--text-primary);
}

.monaco-container {
  width: 100%;
  height: 100%;
}
</style>
```

- [ ] **Step 8: Create AIPanel.vue**

```vue
<!-- src/components/layout/AIPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const messages = ref<{ role: "user" | "assistant"; content: string }[]>([]);
const input = ref("");

const sendMessage = () => {
  if (!input.value.trim()) return;
  messages.value.push({ role: "user", content: input.value });
  input.value = "";
  // AI response will be added later
};
</script>

<template>
  <div class="ai-panel">
    <div class="panel-header-sm">AI 助手</div>
    <div class="ai-messages">
      <div v-if="messages.length === 0" class="empty-state">
        <p>开始与 AI 对话</p>
      </div>
      <div
        v-for="(msg, i) in messages"
        :key="i"
        :class="['message', msg.role]"
      >
        <div class="message-content">{{ msg.content }}</div>
      </div>
    </div>
    <div class="ai-input">
      <input
        v-model="input"
        type="text"
        placeholder="输入消息..."
        @keydown.enter="sendMessage"
      />
      <button @click="sendMessage">发送</button>
    </div>
  </div>
</template>

<style scoped>
.ai-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border);
}

.panel-header-sm {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border);
}

.ai-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
}

.message {
  margin-bottom: var(--spacing-md);
}

.message.user .message-content {
  background: var(--accent);
  color: var(--bg-primary);
  margin-left: 40px;
}

.message.assistant .message-content {
  background: var(--bg-surface);
  color: var(--text-primary);
  margin-right: 40px;
}

.message-content {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: 8px;
  font-size: var(--font-size-md);
  line-height: 1.5;
  word-wrap: break-word;
}

.ai-input {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  border-top: 1px solid var(--border);
}

.ai-input input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  outline: none;
}

.ai-input input:focus {
  border-color: var(--accent);
}

.ai-input button {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.ai-input button:hover {
  opacity: 0.9;
}
</style>
```

- [ ] **Step 9: Create BottomPanel.vue**

```vue
<!-- src/components/layout/BottomPanel.vue -->
<script setup lang="ts">
import { ref } from "vue";

const activeTab = ref<"output" | "terminal" | "problems">("output");

const tabs = [
  { id: "output" as const, label: "输出" },
  { id: "terminal" as const, label: "终端" },
  { id: "problems" as const, label: "问题" },
];
</script>

<template>
  <div class="bottom-panel">
    <div class="panel-tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>
    <div class="panel-content">
      <div v-if="activeTab === 'output'" class="tab-body">
        <div class="empty-state">暂无输出</div>
      </div>
      <div v-else-if="activeTab === 'terminal'" class="tab-body">
        <div class="empty-state">终端功能开发中</div>
      </div>
      <div v-else-if="activeTab === 'problems'" class="tab-body">
        <div class="empty-state">暂无问题</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bottom-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border);
}

.panel-tabs {
  display: flex;
  height: 32px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
}

.tab-btn {
  padding: 0 var(--spacing-md);
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  color: var(--text-primary);
  border-bottom-color: var(--accent);
}

.panel-content {
  flex: 1;
  overflow: auto;
}

.tab-body {
  padding: var(--spacing-md);
}

.empty-state {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
}
</style>
```

- [ ] **Step 10: Create IDELayout.vue**

```vue
<!-- src/components/layout/IDELayout.vue -->
<script setup lang="ts">
import { ref } from "vue";
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar.vue";
import EditorPanel from "./EditorPanel.vue";
import AIPanel from "./AIPanel.vue";
import BottomPanel from "./BottomPanel.vue";

const sidebarWidth = ref(260);
const aiPanelWidth = ref(320);
const bottomPanelHeight = ref(200);
const showBottomPanel = ref(true);
const showAIPanel = ref(true);

const isDragging = ref(false);
const dragTarget = ref<"sidebar" | "ai" | "bottom" | null>(null);

const onMouseDown = (target: "sidebar" | "ai" | "bottom", e: MouseEvent) => {
  isDragging.value = true;
  dragTarget.value = target;
  e.preventDefault();
};

const onMouseMove = (e: MouseEvent) => {
  if (!isDragging.value) return;

  if (dragTarget.value === "sidebar") {
    sidebarWidth.value = Math.max(200, Math.min(e.clientX, 500));
  } else if (dragTarget.value === "ai") {
    aiPanelWidth.value = Math.max(250, Math.min(window.innerWidth - e.clientX, 500));
  } else if (dragTarget.value === "bottom") {
    bottomPanelHeight.value = Math.max(100, Math.min(window.innerHeight - e.clientY, 400));
  }
};

const onMouseUp = () => {
  isDragging.value = false;
  dragTarget.value = null;
};

// Global mouse events
window.addEventListener("mousemove", onMouseMove);
window.addEventListener("mouseup", onMouseUp);
</script>

<template>
  <div class="ide-layout">
    <TitleBar />
    <div class="ide-main">
      <div class="ide-sidebar" :style="{ width: `${sidebarWidth}px` }">
        <Sidebar />
      </div>
      <div class="sidebar-resize" @mousedown="onMouseDown('sidebar', $event)" />
      <div class="ide-center">
        <div class="center-editor" :style="{ height: showBottomPanel ? `calc(100% - ${bottomPanelHeight}px)` : '100%' }">
          <EditorPanel />
        </div>
        <div v-if="showBottomPanel" class="bottom-resize" @mousedown="onMouseDown('bottom', $event)" />
        <div v-if="showBottomPanel" class="center-bottom" :style="{ height: `${bottomPanelHeight}px` }">
          <BottomPanel />
        </div>
      </div>
      <div v-if="showAIPanel" class="ai-resize" @mousedown="onMouseDown('ai', $event)" />
      <div v-if="showAIPanel" class="ide-ai" :style="{ width: `${aiPanelWidth}px` }">
        <AIPanel />
      </div>
    </div>
  </div>
</template>

<style scoped>
.ide-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}

.ide-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.ide-sidebar {
  flex-shrink: 0;
  overflow: hidden;
}

.sidebar-resize {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background 0.15s;
}

.sidebar-resize:hover {
  background: var(--accent);
}

.ide-center {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.center-editor {
  overflow: hidden;
}

.bottom-resize {
  height: 4px;
  cursor: row-resize;
  background: transparent;
  transition: background 0.15s;
}

.bottom-resize:hover {
  background: var(--accent);
}

.center-bottom {
  flex-shrink: 0;
  overflow: hidden;
}

.ai-resize {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background 0.15s;
}

.ai-resize:hover {
  background: var(--accent);
}

.ide-ai {
  flex-shrink: 0;
  overflow: hidden;
}
</style>
```

- [ ] **Step 11: Verify frontend builds**

Run: `bun run build`
Expected: Builds without errors

- [ ] **Step 12: Commit**

```bash
git add src/components/ src/composables/ src/styles/
git commit -m "feat: add IDE layout with resizable panels, titlebar, sidebar, editor, AI panel"
```

---

### Task 5: Project List & New Project Dialog

**Files:**
- Create: `src/stores/project.ts`
- Create: `src/stores/settings.ts`
- Create: `src/components/project/ProjectList.vue`
- Create: `src/components/project/NewProject.vue`
- Create: `src/composables/useTauriIPC.ts`
- Modify: `src/App.vue`

**Interfaces:**
- Consumes: Task 3 (Tauri commands), Task 4 (IDE layout)
- Produces: ProjectStore with CRUD operations, ProjectList and NewProject components

- [ ] **Step 1: Create useTauriIPC.ts**

```typescript
// src/composables/useTauriIPC.ts
import { invoke } from "@tauri-apps/api/core";

export function useTauriIPC() {
  const call = async <T>(command: string, args?: Record<string, unknown>): Promise<T> => {
    return invoke<T>(command, args);
  };

  return { call };
}
```

- [ ] **Step 2: Create project store**

```typescript
// src/stores/project.ts
import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Project {
  id: string;
  name: string;
  path: string;
  genre: string | null;
  sub_genre: string | null;
  target_readers: string | null;
  total_chapters: number | null;
  words_per_chapter: number | null;
  narrative_pov: string | null;
  story_structure: string | null;
  created_at: string;
  updated_at: string;
}

export const useProjectStore = defineStore("project", () => {
  const { call } = useTauriIPC();
  const projects = ref<Project[]>([]);
  const currentProject = ref<Project | null>(null);
  const loading = ref(false);

  const fetchProjects = async () => {
    loading.value = true;
    try {
      projects.value = await call<Project[]>("list_projects");
    } finally {
      loading.value = false;
    }
  };

  const createProject = async (params: {
    name: string;
    path: string;
    genre?: string;
    sub_genre?: string;
    target_readers?: string;
    total_chapters?: number;
    words_per_chapter?: number;
    narrative_pov?: string;
    story_structure?: string;
  }) => {
    const project = await call<Project>("create_project", params);
    projects.value.unshift(project);
    return project;
  };

  const openProject = async (projectId: string) => {
    const project = await call<Project>("open_project", { projectId });
    currentProject.value = project;
    return project;
  };

  const deleteProject = async (projectId: string) => {
    await call("delete_project", { projectId });
    projects.value = projects.value.filter((p) => p.id !== projectId);
    if (currentProject.value?.id === projectId) {
      currentProject.value = null;
    }
  };

  return {
    projects,
    currentProject,
    loading,
    fetchProjects,
    createProject,
    openProject,
    deleteProject,
  };
});
```

- [ ] **Step 3: Create settings store**

```typescript
// src/stores/settings.ts
import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Setting {
  key: string;
  value: string | null;
}

export const useSettingsStore = defineStore("settings", () => {
  const { call } = useTauriIPC();
  const settings = ref<Map<string, string>>(new Map());
  const loading = ref(false);

  const fetchSettings = async () => {
    loading.value = true;
    try {
      const list = await call<Setting[]>("get_settings");
      settings.value = new Map(list.map((s) => [s.key, s.value || ""]));
    } finally {
      loading.value = false;
    }
  };

  const updateSetting = async (key: string, value: string) => {
    await call("update_settings", { key, value: JSON.stringify(value) });
    settings.value.set(key, value);
  };

  const getSetting = (key: string): string => {
    return settings.value.get(key) || "";
  };

  return { settings, loading, fetchSettings, updateSetting, getSetting };
});
```

- [ ] **Step 4: Create ProjectList.vue**

```vue
<!-- src/components/project/ProjectList.vue -->
<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useProjectStore } from "../../stores/project";

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
```

- [ ] **Step 5: Create NewProject.vue**

```vue
<!-- src/components/project/NewProject.vue -->
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
    // Fallback: use default path
    form.value.path = "~/NovelProjects";
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
            <input v-model="form.path" type="text" placeholder="选择项目目录" readonly />
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
```

- [ ] **Step 6: Update App.vue to show ProjectList or IDE**

```vue
<!-- src/App.vue (updated) -->
<script setup lang="ts">
import { ref } from "vue";
import IDELayout from "./components/layout/IDELayout.vue";
import ProjectList from "./components/project/ProjectList.vue";
import { useProjectStore } from "./stores/project";

const projectStore = useProjectStore();
const showIDE = ref(false);

const openProject = async (projectId: string) => {
  await projectStore.openProject(projectId);
  showIDE.value = true;
};

const backToHome = () => {
  showIDE.value = false;
  projectStore.currentProject = null;
};
</script>

<template>
  <ProjectList v-if="!showIDE" @open-project="openProject" />
  <IDELayout v-else @back="backToHome" />
</template>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
}
</style>
```

- [ ] **Step 7: Verify full build**

Run: `bun run build && cd src-tauri && cargo check`
Expected: Both complete without errors

- [ ] **Step 8: Commit**

```bash
git add src/stores/ src/components/project/ src/composables/useTauriIPC.ts src/App.vue
git commit -m "feat: add project list, new project dialog, and project/settings stores"
```

---

### Task 6: Integration Test & Final Verification

**Files:**
- Modify: `src-tauri/src/lib.rs` (ensure all commands registered)

**Interfaces:**
- Consumes: Tasks 1-5
- Produces: Running Novel IDE app with project management

- [ ] **Step 1: Verify all commands are registered in lib.rs**

```rust
// Ensure src-tauri/src/lib.rs has all commands:
.invoke_handler(tauri::generate_handler![
    commands::project::create_project,
    commands::project::list_projects,
    commands::project::open_project,
    commands::project::delete_project,
    commands::settings::get_settings,
    commands::settings::update_settings,
])
```

- [ ] **Step 2: Run full build**

Run: `cd src-tauri && cargo build --release`
Expected: Build succeeds (may take a few minutes on first build)

- [ ] **Step 3: Run frontend dev server**

Run: `bun run dev`
Expected: Vite dev server starts on http://localhost:5173

- [ ] **Step 4: Run Tauri dev**

Run: `cd src-tauri && cargo tauri dev`
Expected: App window opens with IDE layout

- [ ] **Step 5: Test project creation flow**

1. Click "新建项目"
2. Enter project name "测试小说"
3. Select a directory
4. Click "创建项目"
5. Verify project appears in list
6. Click on project to open IDE

- [ ] **Step 6: Commit final state**

```bash
git add -A
git commit -m "feat: Phase 1 complete - core scaffolding with project management"
```

---

## Summary

After completing all 6 tasks, you will have:

1. **Tauri 2 project** with Vue 3 + TypeScript frontend
2. **SQLite database** with project and settings tables
3. **IDE layout** with resizable 4-panel design (sidebar, editor, AI panel, bottom panel)
4. **Project management** - create, list, open, delete projects
5. **Settings system** - global settings with key-value storage
6. **Custom title bar** with minimize/maximize/close
7. **Dark theme** with CSS variables
8. **Project cards** with genre, chapter count, and last updated

**Next phases:**
- Phase 2: Monaco Editor integration + Markdown editing
- Phase 3: World building + Character management
- Phase 4: AI model management + Provider presets
- Phase 5: AI writing pipeline + Agent system
- Phase 6: Knowledge base (RAG) + Vector search
- Phase 7: Cloud sync + Config backup
- Phase 8: Text proofreading + Export
