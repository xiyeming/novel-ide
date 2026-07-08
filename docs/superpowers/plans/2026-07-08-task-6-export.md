# Task 6: Export Command + UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add chapter export functionality — export single chapter or all chapters to TXT/Markdown files via a dialog UI.

**Architecture:** New Rust command `export_chapter`/`export_all_chapters` handles file I/O. Vue `ExportDialog.vue` component provides format selector and directory picker using Tauri dialog plugin. Export button added to EditorPanel toolbar.

**Tech Stack:** Rust (tauri::command, serde), Vue 3 + Composition API, Tauri dialog plugin

## Global Constraints

- Rust edition 2024, rust-version 1.90
- Tauri 2.11, tauri-plugin-dialog 2.5
- Vue 3 + TypeScript + Pinia + Composition API
- Follow existing code patterns (chinese error messages, `AppResult<T>`, `useTauriIPC` composable)
- CSS variables: `--bg-primary`, `--accent`, `--text-primary`, etc.

---

### Task 1: Create export.rs command file

**Files:**
- Create: `src-tauri/src/commands/export.rs`

**Interfaces:**
- Consumes: `AppState`, `Chapter`, `Project` models
- Produces: `ExportResult` struct, `export_chapter` and `export_all_chapters` commands

- [ ] **Step 1: Create export.rs**

```rust
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

    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;
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

- [ ] **Step 2: Verify file is syntactically correct**

Run: `cargo check --lib` in `src-tauri/`
Expected: no compile errors (may warn about unused until registered)

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/export.rs
git commit -m "feat(export): add export_chapter and export_all_chapters commands"
```

---

### Task 2: Register export commands in mod.rs and lib.rs

**Files:**
- Modify: `src-tauri/src/commands/mod.rs:1-6`
- Modify: `src-tauri/src/lib.rs:28-50`

**Interfaces:**
- Consumes: export.rs commands from Task 1
- Produces: Commands registered in Tauri invoke handler

- [ ] **Step 1: Add `pub mod export;` to commands/mod.rs**

Add at line 1:
```rust
pub mod chapter;
pub mod export;
pub mod model;
pub mod project;
pub mod search;
pub mod settings;
pub mod version;
```

- [ ] **Step 2: Register commands in lib.rs invoke_handler**

Add to the `tauri::generate_handler![]` macro (after `commands::chapter::delete_chapter,`):
```rust
            commands::export::export_chapter,
            commands::export::export_all_chapters,
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check --lib` in `src-tauri/`
Expected: `Finished` with no errors

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat(export): register export commands in mod and lib"
```

---

### Task 3: Create ExportDialog.vue component

**Files:**
- Create: `src/components/editor/ExportDialog.vue`

**Interfaces:**
- Consumes: `useTauriIPC`, `useProjectStore`, `useChapterStore`
- Produces: `ExportDialog` component with `open()` method

- [ ] **Step 1: Create ExportDialog.vue**

```vue
<!-- src/components/editor/ExportDialog.vue -->
<script setup lang="ts">
import { ref } from "vue";
import { useTauriIPC } from "../../composables/useTauriIPC";
import { useProjectStore } from "../../stores/project";
import { useChapterStore } from "../../stores/chapter";

const emit = defineEmits<{
  exported: [];
}>();

const { call } = useTauriIPC();
const projectStore = useProjectStore();
const chapterStore = useChapterStore();

const visible = ref(false);
const format = ref("txt");
const outputDir = ref("");
const exportMode = ref<"current" | "all">("current");
const exporting = ref(false);
const results = ref<{ file_path: string; file_size: number }[]>([]);

const open = () => {
  visible.value = true;
  results.value = [];
  outputDir.value = "";
};

const close = () => {
  visible.value = false;
  results.value = [];
};

const pickDirectory = async () => {
  try {
    const selected = await call<string | null>("plugin:dialog|open", {
      directory: true,
      multiple: false,
      title: "选择导出目录",
    });
    if (selected) {
      outputDir.value = selected;
    }
  } catch {
    // User cancelled or error
  }
};

const doExport = async () => {
  if (!outputDir.value) {
    alert("请先选择导出目录");
    return;
  }

  exporting.value = true;
  results.value = [];

  try {
    if (exportMode.value === "current") {
      const chapterId = chapterStore.currentChapter?.id;
      if (!chapterId) {
        alert("请先打开一个章节");
        return;
      }
      const result = await call<{ file_path: string; file_size: number }>(
        "export_chapter",
        {
          chapterId,
          format: format.value,
          outputPath: outputDir.value,
        }
      );
      results.value = [result];
    } else {
      const projectId = projectStore.currentProject?.id;
      if (!projectId) {
        alert("请先打开一个项目");
        return;
      }
      const result = await call<{ file_path: string; file_size: number }[]>(
        "export_all_chapters",
        {
          projectId,
          format: format.value,
          outputPath: outputDir.value,
        }
      );
      results.value = result;
    }
    emit("exported");
  } catch (e) {
    alert("导出失败: " + String(e));
  } finally {
    exporting.value = false;
  }
};

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  return (bytes / (1024 * 1024)).toFixed(1) + " MB";
};

defineExpose({ open });
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="dialog-overlay" @click.self="close">
      <div class="dialog">
        <div class="dialog-header">
          <span class="dialog-title">导出章节</span>
          <button class="dialog-close" @click="close">×</button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label class="form-label">导出格式</label>
            <div class="format-options">
              <label class="format-option">
                <input type="radio" v-model="format" value="txt" />
                <span class="format-label">TXT 纯文本</span>
              </label>
              <label class="format-option">
                <input type="radio" v-model="format" value="md" />
                <span class="format-label">Markdown</span>
              </label>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">导出范围</label>
            <div class="format-options">
              <label class="format-option">
                <input type="radio" v-model="exportMode" value="current" />
                <span class="format-label">当前章节</span>
              </label>
              <label class="format-option">
                <input type="radio" v-model="exportMode" value="all" />
                <span class="format-label">全部章节</span>
              </label>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">输出目录</label>
            <div class="dir-picker">
              <input
                class="dir-input"
                :value="outputDir"
                readonly
                placeholder="未选择目录"
              />
              <button class="dir-btn" @click="pickDirectory">选择</button>
            </div>
          </div>
        </div>
        <div v-if="results.length > 0" class="dialog-results">
          <div class="results-title">导出完成</div>
          <div class="results-list">
            <div v-for="(r, i) in results" :key="i" class="result-item">
              <span class="result-name">{{ r.file_path.split(/[/\\]/).pop() }}</span>
              <span class="result-size">{{ formatSize(r.file_size) }}</span>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn-secondary" @click="close">关闭</button>
          <button
            class="btn-primary"
            :disabled="exporting || !outputDir"
            @click="doExport"
          >
            {{ exporting ? "导出中..." : "导出" }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 8px;
  width: 420px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border);
}

.dialog-title {
  font-size: var(--font-size-md);
  font-weight: 600;
  color: var(--text-primary);
}

.dialog-close {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
}

.dialog-close:hover {
  color: var(--text-primary);
}

.dialog-body {
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-label {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-secondary);
}

.format-options {
  display: flex;
  gap: var(--spacing-md);
}

.format-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.dir-picker {
  display: flex;
  gap: var(--spacing-sm);
}

.dir-input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.dir-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  cursor: pointer;
}

.dir-btn:hover {
  background: var(--bg-hover);
}

.dialog-results {
  padding: 0 var(--spacing-lg);
}

.results-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: #22c55e;
  margin-bottom: var(--spacing-xs);
}

.results-list {
  max-height: 120px;
  overflow-y: auto;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-xs) 0;
  font-size: var(--font-size-sm);
}

.result-name {
  color: var(--text-primary);
}

.result-size {
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border);
}

.btn-secondary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  cursor: pointer;
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  border: none;
  border-radius: 4px;
  color: var(--bg-primary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  font-weight: 500;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
```

- [ ] **Step 2: Verify no TypeScript errors**

Run: `bun run build` from project root
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add src/components/editor/ExportDialog.vue
git commit -m "feat(export): add ExportDialog.vue component"
```

---

### Task 4: Add export button to EditorPanel toolbar

**Files:**
- Modify: `src/components/layout/EditorPanel.vue:2-22, 142-150`

**Interfaces:**
- Consumes: `ExportDialog` component
- Produces: Export button in toolbar, export dialog integration

- [ ] **Step 1: Update EditorPanel script to import ExportDialog**

Add after line 5 (`import VersionHistory from "../editor/VersionHistory.vue";`):
```typescript
import ExportDialog from "../editor/ExportDialog.vue";
```

Add after line 22 (`const versionHistoryRef = ref<...>`):
```typescript
const exportDialogRef = ref<InstanceType<typeof ExportDialog> | null>(null);

const openExportDialog = () => {
  exportDialogRef.value?.open();
};
```

- [ ] **Step 2: Add ExportDialog component to template**

Add after the `VersionHistory` component closing tag (line 175), before the closing `</div>`:
```html
      <ExportDialog ref="exportDialogRef" />
```

- [ ] **Step 3: Add export button to toolbar**

Add after the version history button (line 149), before the closing `</div>` of `editor-toolbar`:
```html
      <button
        class="toolbar-btn"
        @click="openExportDialog"
        title="导出"
      >
        📤
      </button>
```

- [ ] **Step 4: Verify build**

Run: `bun run build` from project root
Expected: Build succeeds

- [ ] **Step 5: Commit**

```bash
git add src/components/layout/EditorPanel.vue
git commit -m "feat(export): add export button to EditorPanel toolbar"
```

---

### Task 5: Full verification

**Files:** None (verification only)

- [ ] **Step 1: Rust check**

Run: `cargo check --lib` in `src-tauri/`
Expected: `Finished` with no errors

- [ ] **Step 2: Frontend build**

Run: `bun run build` from project root
Expected: Build succeeds with no errors

- [ ] **Step 3: Final commit if any fixes needed**

```bash
git add -A
git commit -m "fix(export): address build issues"
```

---

### Task 6: Final commit

- [ ] **Step 1: Stage all export-related files and commit**

```bash
git add src-tauri/src/commands/export.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src/components/editor/ExportDialog.vue src/components/layout/EditorPanel.vue
git commit -m "feat: add chapter export functionality"
```

- [ ] **Step 2: Report completion**
