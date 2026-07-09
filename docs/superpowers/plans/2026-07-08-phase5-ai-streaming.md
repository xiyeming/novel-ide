# Phase 5: AI Streaming + Knowledge Base + Proofreading

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add AI streaming output via OpenAI-compatible API, knowledge base document import with vector search, and text proofreading with error highlighting.

**Architecture:** Three features: (1) Rust streaming HTTP client using reqwest + SSE, Tauri events for real-time chunks, (2) document import + LanceDB vector storage + hybrid search, (3) AI proofreading with structured error output and editor decorations.

**Tech Stack:** Vue 3.5.39, Pinia 3.0.4, Tauri 2.11, SQLx 0.9, reqwest 0.12, LanceDB 0.31, TypeScript 5.7+

---

## Task 1: AI Streaming Backend

**Files:**
- Create: `src-tauri/src/commands/ai.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`

**Interfaces:**
- Consumes: `model_providers` table (API URL, key, model name)
- Produces: `chat_stream` command using SSE streaming, emits `ai:chunk` events to frontend

- [ ] **Step 1: Add reqwest features to Cargo.toml**

Add `reqwest = { version = "0.12", features = ["json", "stream"] }` and `tokio-stream = "0.1"`.

- [ ] **Step 2: Create AI streaming command**

```rust
// src-tauri/src/commands/ai.rs
use crate::error::AppResult;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{State, Emitter};

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub content: String,
    pub finish_reason: Option<String>,
}

#[tauri::command]
pub async fn chat_stream(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    provider_id: String,
    messages: Vec<ChatMessage>,
    system_prompt: Option<String>,
) -> AppResult<ChatResponse> {
    let db = state.db().await?;

    // Get provider config
    let provider = crate::db::models::model_provider::ModelProvider::find_by_id(&db, &provider_id).await?;

    // Build OpenAI-compatible request
    let mut api_messages = Vec::new();
    if let Some(sys) = &system_prompt {
        api_messages.push(serde_json::json!({
            "role": "system",
            "content": sys
        }));
    }
    for msg in &messages {
        api_messages.push(serde_json::json!({
            "role": msg.role,
            "content": msg.content
        }));
    }

    let request_body = serde_json::json!({
        "model": provider.model_name,
        "messages": api_messages,
        "stream": true
    });

    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", provider.api_url.trim_end_matches('/'));

    let mut req = client.post(&url)
        .header("Content-Type", "application/json");

    if let Some(key) = &provider.api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.json(&request_body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(crate::error::AppError::Internal(format!("API 错误 ({}): {}", status, body)));
    }

    // Stream SSE response
    let mut full_content = String::new();
    let mut buffer = String::new();

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
        buffer.push_str(&String::from_utf8_lossy(&bytes));

        // Process complete lines
        while let Some(line_end) = buffer.find('\n') {
            let line = buffer[..line_end].trim().to_string();
            buffer = buffer[line_end + 1..].to_string();

            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(choices) = json["choices"].as_array() {
                        if let Some(choice) = choices.first() {
                            if let Some(delta) = choice["delta"]["content"].as_str() {
                                full_content.push_str(delta);
                                // Emit chunk to frontend
                                let _ = app.emit("ai:chunk", serde_json::json!({
                                    "content": delta,
                                    "done": false
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    // Emit completion
    let _ = app.emit("ai:chunk", serde_json::json!({
        "content": "",
        "done": true
    }));

    Ok(ChatResponse {
        content: full_content,
        finish_reason: Some("stop".into()),
    })
}
```

- [ ] **Step 3: Register command**

- [ ] **Step 4: Verify — `cargo check`**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/ai.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat: add AI streaming backend with OpenAI-compatible API"
```

---

## Task 2: AI Streaming Frontend

**Files:**
- Modify: `src/components/layout/AIPanel.vue`
- Modify: `src/stores/ai.ts`

**Interfaces:**
- Consumes: `chat_stream` command, `ai:chunk` Tauri events
- Produces: real-time streaming display in AI chat

- [ ] **Step 1: Update AI store**

Add `streaming` ref and method to handle chunk events.

- [ ] **Step 2: Update AIPanel**

Wire `sendMessage` to call `chat_stream`, listen for `ai:chunk` events, append chunks to assistant message in real-time.

- [ ] **Step 3: Verify — `bun run build`**

- [ ] **Step 4: Commit**

```bash
git add src/components/layout/AIPanel.vue src/stores/ai.ts
git commit -m "feat: add AI streaming frontend with real-time display"
```

---

## Task 3: Knowledge Base Migration + Model

**Files:**
- Create: `src-tauri/migrations/005_knowledge_base.sql`
- Create: `src-tauri/src/db/models/knowledge.rs`
- Modify: `src-tauri/src/db/models/mod.rs`
- Modify: `src-tauri/src/db/mod.rs`

**Interfaces:**
- Consumes: `projects` table
- Produces: `knowledge_documents` table + model

- [ ] **Step 1: Create migration**

```sql
-- migrations/005_knowledge_base.sql
CREATE TABLE IF NOT EXISTS knowledge_documents (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    file_path TEXT,
    doc_type TEXT NOT NULL DEFAULT 'text',
    chunk_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_knowledge_project ON knowledge_documents(project_id);
```

- [ ] **Step 2: Create knowledge model**

- [ ] **Step 3: Register migration**

- [ ] **Step 4: Verify — `cargo check`**

- [ ] **Step 5: Commit**

```bash
git add src-tauri/migrations/005_knowledge_base.sql src-tauri/src/db/models/knowledge.rs src-tauri/src/db/models/mod.rs src-tauri/src/db/mod.rs
git commit -m "feat: add knowledge base migration and model"
```

---

## Task 4: Knowledge Base Commands

**Files:**
- Create: `src-tauri/src/commands/knowledge.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `knowledge_documents` table, `chapters_fts` table
- Produces: `import_knowledge`, `list_knowledge`, `delete_knowledge`, `search_knowledge` commands

- [ ] **Step 1: Create knowledge commands**

- `import_knowledge(project_id, title, content, file_path)` — import text document
- `list_knowledge(project_id)` — list all knowledge documents
- `delete_knowledge(id)` — delete a document
- `search_knowledge(project_id, query, limit)` — FTS5 search across knowledge + chapters

- [ ] **Step 2: Register commands**

- [ ] **Step 3: Verify — `cargo check`**

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/knowledge.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add knowledge base commands"
```

---

## Task 5: Knowledge Base UI

**Files:**
- Create: `src/stores/knowledge.ts`
- Create: `src/components/knowledge/KnowledgePanel.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Interfaces:**
- Consumes: knowledge commands
- Produces: knowledge document list, import dialog, search

- [ ] **Step 1: Create knowledge store**

- [ ] **Step 2: Create KnowledgePanel component**

Show:
- Document list with title, type, chunk count
- Import button (paste text or file upload)
- Delete button
- Search input that searches across all knowledge + chapters

- [ ] **Step 3: Add "知识库" tab to Sidebar**

- [ ] **Step 4: Verify — `bun run build`**

- [ ] **Step 5: Commit**

```bash
git add src/stores/knowledge.ts src/components/knowledge/KnowledgePanel.vue src/components/layout/Sidebar.vue
git commit -m "feat: add knowledge base UI"
```

---

## Task 6: Text Proofreading

**Files:**
- Create: `src-tauri/src/commands/proofread.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Create: `src/components/editor/ProofreadPanel.vue`

**Interfaces:**
- Consumes: chapter content, AI streaming
- Produces: structured proofreading errors, editor decorations

- [ ] **Step 1: Create proofreading command**

```rust
#[derive(Debug, Serialize)]
pub struct ProofreadError {
    pub error_type: String,
    pub original: String,
    pub suggestion: String,
    pub line: usize,
    pub column: usize,
    pub confidence: f64,
}

#[tauri::command]
pub async fn proofread_chapter(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    provider_id: String,
    content: String,
) -> AppResult<Vec<ProofreadError>> {
    // Build proofreading prompt
    let system_prompt = r#"你是一个专业的中文校对编辑。请分析以下文本，找出以下类型的错误：
1. 错别字 (typo) - 同音字、形近字错误
2. 语病 (grammar) - 语法不通、成分残缺
3. 标点错误 (punctuation) - 标点使用不当
4. 用词不当 (word_choice) - 词语搭配或语境不当
5. 逻辑错误 (logic) - 前后矛盾、时间线冲突

请以 JSON 数组格式返回，每个错误包含：
- error_type: 错误类型 (typo/grammar/punctuation/word_choice/logic)
- original: 原文
- suggestion: 建议修改
- line: 行号（从1开始）
- column: 列号（从1开始）
- confidence: 置信度 (0-1)

如果没有错误，返回空数组 []"#;

    // Call AI streaming...
    // Parse response as JSON array of errors
    // Return errors
}
```

- [ ] **Step 2: Register command**

- [ ] **Step 3: Create ProofreadPanel component**

Show:
- "校对" button in editor toolbar
- Error list with type, original, suggestion
- Click error to jump to line in editor
- "修复选中" and "全部修复" buttons

- [ ] **Step 4: Add proofread button to EditorPanel**

- [ ] **Step 5: Verify — `cargo check && bun run build`**

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/proofread.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src/components/editor/ProofreadPanel.vue src/components/layout/EditorPanel.vue
git commit -m "feat: add text proofreading with error highlighting"
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

1. Select model → send message → AI streams response
2. Import knowledge document → appears in list
3. Search knowledge → results from docs + chapters
4. Click "校对" → errors highlighted → fix errors

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: Phase 5 complete — AI streaming + knowledge base + proofreading"
```
