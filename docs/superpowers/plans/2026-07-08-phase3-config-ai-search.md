# Phase 3: Project Config + AI Chat + Full-text Search

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add project configuration panel, enhance AI chat panel with model selection, and implement full-text search across chapters.

**Architecture:** Three independent features: (1) ProjectConfig component wired to project store with form fields for all 15+ config items, (2) AI Chat panel with model selector and message history, (3) FTS5-based full-text search in sidebar with real-time results.

**Tech Stack:** Vue 3.5.39, Pinia 3.0.4, Tauri 2.11, SQLx 0.9, TypeScript 5.7+

---

## Task 1: Database Migration — FTS5 Virtual Table

**Files:**
- Create: `src-tauri/migrations/003_chapters_fts.sql`

**Interfaces:**
- Consumes: existing `chapters` table
- Produces: FTS5 virtual table `chapters_fts` for full-text search

- [ ] **Step 1: Create FTS5 migration**

```sql
-- migrations/003_chapters_fts.sql
CREATE VIRTUAL TABLE IF NOT EXISTS chapters_fts USING fts5(
    chapter_id UNINDEXED,
    title,
    content,
    content=chapters,
    content_rowid=rowid
);

-- Triggers to keep FTS index in sync
CREATE TRIGGER IF NOT EXISTS chapters_ai AFTER INSERT ON chapters BEGIN
    INSERT INTO chapters_fts(rowid, chapter_id, title, content)
    VALUES (new.rowid, new.id, new.title, new.content);
END;

CREATE TRIGGER IF NOT EXISTS chapters_ad AFTER DELETE ON chapters BEGIN
    INSERT INTO chapters_fts(chapters_fts, rowid, chapter_id, title, content)
    VALUES('delete', old.rowid, old.id, old.title, old.content);
END;

CREATE TRIGGER IF NOT EXISTS chapters_au AFTER UPDATE ON chapters BEGIN
    INSERT INTO chapters_fts(chapters_fts, rowid, chapter_id, title, content)
    VALUES('delete', old.rowid, old.id, old.title, old.content);
    INSERT INTO chapters_fts(rowid, chapter_id, title, content)
    VALUES (new.rowid, new.id, new.title, new.content);
END;
```

- [ ] **Step 2: Add migration to db/mod.rs**

Add `const MIGRATION_003: &str = include_str!("../../migrations/003_chapters_fts.sql");` and execute it in `init_database`.

- [ ] **Step 3: Verify compilation**

Run: `cargo check`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/003_chapters_fts.sql src-tauri/src/db/mod.rs
git commit -m "feat: add FTS5 full-text search for chapters"
```

---

## Task 2: Search Command + Store

**Files:**
- Create: `src-tauri/src/commands/search.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Create: `src/stores/search.ts`

**Interfaces:**
- Consumes: `chapters_fts` table, `AppState`
- Produces: `search_chapters` Tauri command, `useSearchStore` with `query`, `results`, `search()`

- [ ] **Step 1: Create search command**

```rust
// src-tauri/src/commands/search.rs
use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub chapter_id: String,
    pub chapter_title: String,
    pub snippet: String,
    pub rank: f64,
}

#[tauri::command]
pub async fn search_chapters(
    state: State<'_, AppState>,
    project_id: String,
    query: String,
    limit: Option<i32>,
) -> AppResult<Vec<SearchResult>> {
    let db = state.db().await?;
    let limit = limit.unwrap_or(20);

    let results = sqlx::query_as::<_, SearchResult>(
        r#"SELECT
            c.id as chapter_id,
            c.title as chapter_title,
            snippet(chapters_fts, 2, '<mark>', '</mark>', '...', 32) as snippet,
            rank
        FROM chapters_fts
        JOIN chapters c ON c.id = chapters_fts.chapter_id
        WHERE chapters_fts MATCH ? AND c.project_id = ?
        ORDER BY rank
        LIMIT ?"#,
    )
    .bind(&query)
    .bind(&project_id)
    .bind(limit)
    .fetch_all(&db)
    .await?;

    Ok(results)
}
```

- [ ] **Step 2: Register command**

Add `pub mod search;` to `commands/mod.rs` and register in `lib.rs`.

- [ ] **Step 3: Create search store**

```typescript
// src/stores/search.ts
import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface SearchResult {
  chapter_id: string;
  chapter_title: string;
  snippet: string;
  rank: number;
}

export const useSearchStore = defineStore("search", () => {
  const { call } = useTauriIPC();
  const query = ref("");
  const results = ref<SearchResult[]>([]);
  const loading = ref(false);

  const search = async (projectId: string, q: string) => {
    query.value = q;
    if (!q.trim()) {
      results.value = [];
      return;
    }
    loading.value = true;
    try {
      results.value = await call<SearchResult[]>("search_chapters", {
        projectId,
        query: q,
      });
    } finally {
      loading.value = false;
    }
  };

  const clear = () => {
    query.value = "";
    results.value = [];
  };

  return { query, results, loading, search, clear };
});
```

- [ ] **Step 4: Verify builds**

Run: `cargo check && bun run build`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/search.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src/stores/search.ts
git commit -m "feat: add search command and store"
```

---

## Task 3: Search UI in Sidebar

**Files:**
- Modify: `src/components/layout/Sidebar.vue`

**Interfaces:**
- Consumes: `useSearchStore`, `useProjectStore`
- Produces: search input in search tab, results list with click-to-open

- [ ] **Step 1: Add search functionality to Sidebar**

Update the "搜索" tab panel to include:
- Search input with debounce (300ms)
- Results list showing chapter title + snippet
- Click result to open that chapter in editor
- Loading indicator during search
- Empty state when no query

- [ ] **Step 2: Verify build**

Run: `bun run build`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/components/layout/Sidebar.vue
git commit -m "feat: add full-text search UI in sidebar"
```

---

## Task 4: Project Configuration Panel

**Files:**
- Create: `src/components/project/ProjectConfig.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Interfaces:**
- Consumes: `useProjectStore`, `useChapterStore`
- Produces: config form with all 15+ fields, auto-save on change

- [ ] **Step 1: Create ProjectConfig component**

Create a form component with these fields:
- 题材 (genre): dropdown — 玄幻/仙侠/都市/科幻/历史/言情/悬疑/恐怖/其他
- 细分类别 (sub_genre): text input
- 目标读者 (target_readers): text input
- 总章数 (total_chapters): number input (10-10000)
- 单章字数 (words_per_chapter): number input (1000-10000, default 3000)
- 叙事视角 (narrative_pov): dropdown — 第一人称/第三人称有限/第三人称全知/第二人称
- 故事结构 (story_structure): dropdown — 三幕式/英雄之旅/起承转合/非线性/自定义
- 核心大纲 (core_outline): textarea
- 世界设定 (world_settings): textarea
- 主角档案 (character_profiles): textarea
- 金手指/外挂 (golden_finger): textarea
- 全局写作要求 (writing_constraints): textarea
- 文风约束 (style_constraints): textarea

All changes auto-save with 500ms debounce.

- [ ] **Step 2: Add config tab to Sidebar**

Add a "配置" tab to the sidebar that shows ProjectConfig when a project is open.

- [ ] **Step 3: Verify build**

Run: `bun run build`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/components/project/ProjectConfig.vue src/components/layout/Sidebar.vue
git commit -m "feat: add project configuration panel"
```

---

## Task 5: AI Chat Panel Enhancement

**Files:**
- Modify: `src/components/layout/AIPanel.vue`
- Create: `src/stores/ai.ts`

**Interfaces:**
- Consumes: `useProjectStore`, `useChapterStore`
- Produces: enhanced AI chat with model selector, message history, markdown rendering

- [ ] **Step 1: Create AI store**

```typescript
// src/stores/ai.ts
import { defineStore } from "pinia";
import { ref } from "vue";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

export const useAIStore = defineStore("ai", () => {
  const messages = ref<Message[]>([]);
  const selectedModel = ref<string>("deepseek-chat");
  const isGenerating = ref(false);

  const addMessage = (role: "user" | "assistant", content: string) => {
    messages.value.push({
      id: crypto.randomUUID(),
      role,
      content,
      timestamp: Date.now(),
    });
  };

  const clearMessages = () => {
    messages.value = [];
  };

  return { messages, selectedModel, isGenerating, addMessage, clearMessages };
});
```

- [ ] **Step 2: Enhance AIPanel component**

Update the AI panel to include:
- Model selector dropdown (top of panel) with preset models
- Message list with proper styling
- Input area with send button
- Markdown rendering for assistant messages (using simple markdown-to-html)
- Clear chat button

- [ ] **Step 3: Verify build**

Run: `bun run build`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/components/layout/AIPanel.vue src/stores/ai.ts
git commit -m "feat: enhance AI chat panel with model selector"
```

---

## Task 6: Integration Verification

**Files:**
- Verify all files from Tasks 1-5 exist
- Verify full user flow

**Interfaces:**
- Consumes: all previous tasks
- Produces: verified integration

- [ ] **Step 1: Verify all files exist**

Check: migrations/003_chapters_fts.sql, commands/search.rs, stores/search.ts, stores/ai.ts, ProjectConfig.vue

- [ ] **Step 2: Verify cargo check + bun build**

Run: `cargo check && bun run build`
Expected: PASS

- [ ] **Step 3: Verify user flow**

1. Open project → sidebar shows config tab
2. Fill in project config → auto-saves
3. Switch to search tab → type query → results appear
4. Click result → editor opens that chapter
5. AI panel shows model selector → can type messages

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: Phase 3 complete — config panel + search + AI chat"
```
