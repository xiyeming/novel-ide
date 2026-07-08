# Phase 2: Monaco Editor + Markdown Editing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate Monaco Editor with Markdown language support, implement chapter file management (create/open/rename/delete), and wire the sidebar file tree to the editor.

**Architecture:** Add Monaco Editor as the core editing component with Markdown syntax highlighting, auto-completion for Markdown headings, and a custom novel-writing theme. Implement chapter CRUD operations via Tauri commands backed by SQLite, with the sidebar displaying a file tree of chapters and allowing open/create/rename/delete. The editor panel manages tabs and saves content back to SQLite.

**Tech Stack:** Monaco Editor 0.55.1, Vue 3.5.39, Pinia 3.0.4, Tauri 2.11, SQLx 0.9, TypeScript 5.7+

---

## Task 1: Database Migration — Chapters Table

**Files:**
- Create: `src-tauri/migrations/002_chapters.sql`
- Modify: `src-tauri/src/db/mod.rs`

**Interfaces:**
- Consumes: existing `projects` table schema
- Produces: `chapters` table with columns: `id TEXT PK`, `project_id TEXT FK`, `title TEXT`, `content TEXT`, `sort_order INTEGER`, `word_count INTEGER`, `created_at TEXT`, `updated_at TEXT`

- [ ] **Step 1: Create migration file**

```sql
-- migrations/002_chapters.sql
CREATE TABLE IF NOT EXISTS chapters (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '未命名章节',
    content TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0,
    word_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_chapters_project ON chapters(project_id);
CREATE INDEX IF NOT EXISTS idx_chapters_sort ON chapters(project_id, sort_order);
```

- [ ] **Step 2: Update db/mod.rs to run new migration**

Add the new migration SQL to the `init_database` function alongside the existing one.

- [ ] **Step 3: Verify compilation**

Run: `cargo check`
Expected: PASS (no errors)

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/002_chapters.sql src-tauri/src/db/mod.rs
git commit -m "feat: add chapters table migration"
```

---

## Task 2: Chapter Model + CRUD Operations

**Files:**
- Create: `src-tauri/src/db/models/chapter.rs`
- Modify: `src-tauri/src/db/models/mod.rs`

**Interfaces:**
- Consumes: `chapters` table, `AppError` type
- Produces: `Chapter` struct, `CreateChapterRequest`, `UpdateChapterRequest`, CRUD methods

- [ ] **Step 1: Create Chapter model**

```rust
// src-tauri/src/db/models/chapter.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Chapter {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub sort_order: i32,
    pub word_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChapterRequest {
    pub project_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChapterRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub sort_order: Option<i32>,
}

impl Chapter {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateChapterRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let content = req.content.clone().unwrap_or_default();
        let word_count = content.chars().count() as i32;
        let sort_order = req.sort_order.unwrap_or(0);

        let chapter = sqlx::query_as::<_, Self>(
            r#"INSERT INTO chapters (id, project_id, title, content, sort_order, word_count, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.project_id)
        .bind(req.title.as_deref().unwrap_or("未命名章节"))
        .bind(&content)
        .bind(sort_order)
        .bind(word_count)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(chapter)
    }

    pub async fn list_by_project(
        db: &sqlx::SqlitePool,
        project_id: &str,
    ) -> Result<Vec<Self>, crate::error::AppError> {
        let chapters = sqlx::query_as::<_, Self>(
            "SELECT * FROM chapters WHERE project_id = ? ORDER BY sort_order ASC",
        )
        .bind(project_id)
        .fetch_all(db)
        .await?;
        Ok(chapters)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let chapter = sqlx::query_as::<_, Self>("SELECT * FROM chapters WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::InvalidArgument("章节不存在".into()))?;
        Ok(chapter)
    }

    pub async fn update(
        db: &sqlx::SqlitePool,
        id: &str,
        req: &UpdateChapterRequest,
    ) -> Result<Self, crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Fetch existing chapter
        let existing = Self::find_by_id(db, id).await?;

        let title = req.title.clone().unwrap_or(existing.title);
        let content = req.content.clone().unwrap_or(existing.content);
        let word_count = content.chars().count() as i32;
        let sort_order = req.sort_order.unwrap_or(existing.sort_order);

        let chapter = sqlx::query_as::<_, Self>(
            r#"UPDATE chapters SET title = ?, content = ?, sort_order = ?, word_count = ?, updated_at = ?
            WHERE id = ? RETURNING *"#,
        )
        .bind(&title)
        .bind(&content)
        .bind(sort_order)
        .bind(word_count)
        .bind(&now)
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(chapter)
    }

    pub async fn delete(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM chapters WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::InvalidArgument("章节不存在".into()));
        }
        Ok(())
    }
}
```

- [ ] **Step 2: Register in models/mod.rs**

Add `pub mod chapter;` to `src-tauri/src/db/models/mod.rs`.

- [ ] **Step 3: Verify compilation**

Run: `cargo check`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/db/models/chapter.rs src-tauri/src/db/models/mod.rs
git commit -m "feat: add Chapter model with CRUD operations"
```

---

## Task 3: Tauri Chapter Commands

**Files:**
- Create: `src-tauri/src/commands/chapter.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `Chapter` model, `AppState`
- Produces: `create_chapter`, `list_chapters`, `get_chapter`, `update_chapter`, `delete_chapter` commands

- [ ] **Step 1: Create chapter commands**

```rust
// src-tauri/src/commands/chapter.rs
use crate::db::models::chapter::{Chapter, CreateChapterRequest, UpdateChapterRequest};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_chapter(
    state: State<'_, AppState>,
    project_id: String,
    title: Option<String>,
    content: Option<String>,
    sort_order: Option<i32>,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let req = CreateChapterRequest {
        project_id,
        title,
        content,
        sort_order,
    };
    let chapter = Chapter::create(&db, &req).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn list_chapters(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<Chapter>> {
    let db = state.db().await?;
    let chapters = Chapter::list_by_project(&db, &project_id).await?;
    Ok(chapters)
}

#[tauri::command]
pub async fn get_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let chapter = Chapter::find_by_id(&db, &chapter_id).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn update_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
    title: Option<String>,
    content: Option<String>,
    sort_order: Option<i32>,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let req = UpdateChapterRequest {
        title,
        content,
        sort_order,
    };
    let chapter = Chapter::update(&db, &chapter_id, &req).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn delete_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<()> {
    let db = state.db().await?;
    Chapter::delete(&db, &chapter_id).await?;
    Ok(())
}
```

- [ ] **Step 2: Register in commands/mod.rs**

Add `pub mod chapter;` to `src-tauri/src/commands/mod.rs`.

- [ ] **Step 3: Register commands in lib.rs**

Add to the `invoke_handler` macro:
```rust
commands::chapter::create_chapter,
commands::chapter::list_chapters,
commands::chapter::get_chapter,
commands::chapter::update_chapter,
commands::chapter::delete_chapter,
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/chapter.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add chapter Tauri commands"
```

---

## Task 4: Chapter Pinia Store

**Files:**
- Create: `src/stores/chapter.ts`

**Interfaces:**
- Consumes: `useTauriIPC` composable
- Produces: `useChapterStore` with `chapters`, `currentChapter`, `fetchChapters`, `createChapter`, `openChapter`, `updateChapterContent`, `updateChapterTitle`, `deleteChapter`

- [ ] **Step 1: Create chapter store**

```typescript
// src/stores/chapter.ts
import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Chapter {
  id: string;
  project_id: string;
  title: string;
  content: string;
  sort_order: number;
  word_count: number;
  created_at: string;
  updated_at: string;
}

export const useChapterStore = defineStore("chapter", () => {
  const { call } = useTauriIPC();
  const chapters = ref<Chapter[]>([]);
  const currentChapter = ref<Chapter | null>(null);
  const loading = ref(false);

  const fetchChapters = async (projectId: string) => {
    loading.value = true;
    try {
      chapters.value = await call<Chapter[]>("list_chapters", { projectId });
    } finally {
      loading.value = false;
    }
  };

  const createChapter = async (
    projectId: string,
    title?: string,
    sortOrder?: number
  ) => {
    const chapter = await call<Chapter>("create_chapter", {
      projectId,
      title,
      sortOrder,
    });
    chapters.value.push(chapter);
    chapters.value.sort((a, b) => a.sort_order - b.sort_order);
    return chapter;
  };

  const openChapter = async (chapterId: string) => {
    const chapter = await call<Chapter>("get_chapter", { chapterId });
    currentChapter.value = chapter;
    return chapter;
  };

  const updateChapterContent = async (chapterId: string, content: string) => {
    const chapter = await call<Chapter>("update_chapter", {
      chapterId,
      content,
    });
    // Update in list
    const idx = chapters.value.findIndex((c) => c.id === chapterId);
    if (idx !== -1) chapters.value[idx] = chapter;
    if (currentChapter.value?.id === chapterId) currentChapter.value = chapter;
  };

  const updateChapterTitle = async (chapterId: string, title: string) => {
    const chapter = await call<Chapter>("update_chapter", {
      chapterId,
      title,
    });
    const idx = chapters.value.findIndex((c) => c.id === chapterId);
    if (idx !== -1) chapters.value[idx] = chapter;
    if (currentChapter.value?.id === chapterId) currentChapter.value = chapter;
  };

  const deleteChapter = async (chapterId: string) => {
    await call("delete_chapter", { chapterId });
    chapters.value = chapters.value.filter((c) => c.id !== chapterId);
    if (currentChapter.value?.id === chapterId) {
      currentChapter.value = chapters.value[0] || null;
    }
  };

  return {
    chapters,
    currentChapter,
    loading,
    fetchChapters,
    createChapter,
    openChapter,
    updateChapterContent,
    updateChapterTitle,
    deleteChapter,
  };
});
```

- [ ] **Step 2: Verify build**

Run: `bun run build`
Expected: PASS (no TypeScript errors)

- [ ] **Step 3: Commit**

```bash
git add src/stores/chapter.ts
git commit -m "feat: add chapter Pinia store"
```

---

## Task 5: Monaco Editor Integration

**Files:**
- Create: `src/components/editor/MonacoEditor.vue`
- Create: `src/composables/useMonaco.ts`
- Modify: `src/components/layout/EditorPanel.vue`

**Interfaces:**
- Consumes: `monaco-editor` package (already in package.json)
- Produces: `MonacoEditor` component with props: `modelValue`, `language`, `readOnly`; emits: `update:modelValue`, `save`

- [ ] **Step 1: Create Monaco composable**

```typescript
// src/composables/useMonaco.ts
import { onMounted, onUnmounted, ref, shallowRef } from "vue";
import * as monaco from "monaco-editor";

export function useMonaco() {
  const editor = shallowRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const container = ref<HTMLDivElement | null>(null);

  const init = (
    containerEl: HTMLDivElement,
    options?: {
      value?: string;
      language?: string;
      readOnly?: boolean;
      theme?: string;
    }
  ) => {
    container.value = containerEl;
    editor.value = monaco.editor.create(containerEl, {
      value: options?.value ?? "",
      language: options?.language ?? "markdown",
      theme: options?.theme ?? "novel-ide-dark",
      readOnly: options?.readOnly ?? false,
      minimap: { enabled: false },
      fontSize: 16,
      lineHeight: 28,
      fontFamily: "'PingFang SC', 'Microsoft YaHei', sans-serif",
      wordWrap: "on",
      padding: { top: 16, bottom: 16 },
      scrollBeyondLastLine: false,
      renderLineHighlight: "gutter",
      automaticLayout: true,
      tabSize: 2,
      suggest: {
        showWords: false,
      },
    });
    return editor.value;
  };

  const dispose = () => {
    editor.value?.dispose();
    editor.value = null;
  };

  const setValue = (value: string) => {
    if (editor.value) {
      const model = editor.value.getModel();
      if (model) {
        model.setValue(value);
      }
    }
  };

  const getValue = (): string => {
    return editor.value?.getValue() ?? "";
  };

  const setReadOnly = (readOnly: boolean) => {
    editor.value?.updateOptions({ readOnly });
  };

  onUnmounted(() => {
    dispose();
  });

  return { editor, container, init, dispose, setValue, getValue, setReadOnly };
}
```

- [ ] **Step 2: Create MonacoEditor component**

```vue
<!-- src/components/editor/MonacoEditor.vue -->
<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useMonaco } from "../../composables/useMonaco";

const props = withDefaults(
  defineProps<{
    modelValue?: string;
    language?: string;
    readOnly?: boolean;
  }>(),
  {
    modelValue: "",
    language: "markdown",
    readOnly: false,
  }
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
  save: [];
}>();

const { editor, container, init, dispose, setValue, getValue } = useMonaco();
const editorContainer = ref<HTMLDivElement>();

onMounted(() => {
  if (editorContainer.value) {
    const ed = init(editorContainer.value, {
      value: props.modelValue,
      language: props.language,
      readOnly: props.readOnly,
    });

    ed.onDidChangeModelContent(() => {
      emit("update:modelValue", getValue());
    });

    // Ctrl+S / Cmd+S to save
    ed.addAction({
      id: "save-document",
      label: "保存",
      keybindings: [2048 | 49], // CtrlOrMeta + S
      run: () => {
        emit("save");
      },
    });
  }
});

watch(
  () => props.modelValue,
  (newVal) => {
    if (editor.value && newVal !== getValue()) {
      setValue(newVal);
    }
  }
);

watch(
  () => props.readOnly,
  (newVal) => {
    setReadOnly(newVal);
  }
);
</script>

<template>
  <div ref="editorContainer" class="monaco-editor-wrapper"></div>
</template>

<style scoped>
.monaco-editor-wrapper {
  width: 100%;
  height: 100%;
}
</style>
```

- [ ] **Step 3: Update EditorPanel to use MonacoEditor**

Replace the `monaco-container` div with `<MonacoEditor>` component. Wire tabs to load/save chapter content.

- [ ] **Step 4: Verify build**

Run: `bun run build`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/editor/MonacoEditor.vue src/composables/useMonaco.ts src/components/layout/EditorPanel.vue
git commit -m "feat: integrate Monaco Editor with Markdown support"
```

---

## Task 6: Sidebar File Tree (Chapter List)

**Files:**
- Modify: `src/components/layout/Sidebar.vue`
- Modify: `src/components/layout/EditorPanel.vue`
- Modify: `src/components/layout/IDELayout.vue`

**Interfaces:**
- Consumes: `useChapterStore`, `useProjectStore`
- Produces: sidebar file tree with chapter list, context menu for create/rename/delete, click to open in editor

- [ ] **Step 1: Update Sidebar to show chapter list**

Replace the "未打开项目" empty state with a chapter list that reads from `useChapterStore.chapters`. Add:
- Chapter list items with click to open
- "新建章节" button at bottom
- Right-click context menu for rename/delete
- Empty state when no chapters

- [ ] **Step 2: Connect IDELayout to pass project context**

Ensure `IDELayout` reads `currentProject` from the project store and passes it to `Sidebar` and `EditorPanel`.

- [ ] **Step 3: Wire EditorPanel to load/save chapters**

When a chapter is opened from the sidebar:
1. `EditorPanel` calls `chapterStore.openChapter(chapterId)` to load content
2. `MonacoEditor` displays the content
3. On content change, debounce 1s then call `chapterStore.updateChapterContent(chapterId, content)`
4. On tab switch, save current chapter content before loading new one

- [ ] **Step 4: Verify build**

Run: `bun run build`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/layout/Sidebar.vue src/components/layout/EditorPanel.vue src/components/layout/IDELayout.vue
git commit -m "feat: add sidebar file tree with chapter management"
```

---

## Task 7: Integration Verification

**Files:**
- Modify: `src-tauri/src/lib.rs` (verify all commands registered)
- Modify: `src/App.vue` (verify project->IDE flow works)

**Interfaces:**
- Consumes: all previous tasks
- Produces: full integration test — create project → open IDE → see file tree → create chapter → editor opens → type → save → verify persistence

- [ ] **Step 1: Verify all Tauri commands are registered**

Check `lib.rs` has all 11 commands: 4 project + 2 settings + 5 chapter.

- [ ] **Step 2: Verify the full user flow**

Manual test checklist:
1. Launch app → project list shows
2. Create new project → appears in list
3. Open project → IDE layout shows
4. Sidebar shows "资源管理器" with empty state
5. Click "新建章节" → chapter appears in list
6. Click chapter → editor opens with Monaco
7. Type Markdown content → auto-saves after 1s
8. Close and reopen → content persists
9. Right-click chapter → rename/delete works
10. Back button → returns to project list

- [ ] **Step 3: Run cargo check + bun build**

Run: `cargo check && bun run build`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: Phase 2 complete — Monaco Editor + chapter management"
```
