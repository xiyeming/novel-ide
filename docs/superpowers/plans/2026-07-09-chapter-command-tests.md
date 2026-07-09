# Chapter Command Tests Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create library-level tests for the Chapter model (CRUD operations) using in-memory SQLite, following the existing project_test.rs pattern.

**Architecture:** Mirror the project_test.rs pattern: in-memory SQLite DB, direct model method calls, no CLI/Tauri layer. Tests cover Chapter::create, list_by_project, find_by_id, update, delete, and error cases.

**Tech Stack:** Rust, sqlx (SQLite), tokio, tempfile, novel_ide_lib

## Global Constraints

- Tests use in-memory SQLite (no filesystem side effects beyond TempDir)
- Follow exact patterns from project_test.rs
- Load migrations via `include_str!("../../migrations/001_initial.sql")` + `002_chapters.sql`
- Chapter table has FOREIGN KEY to projects, so tests must create a project first
- All tests are `#[tokio::test]` async

---

### Task 1: Create chapter_test.rs

**Files:**
- Create: `src-tauri/tests/commands/chapter_test.rs`
- Modify: `src-tauri/tests/commands/mod.rs` (add `mod chapter_test;`)

**Interfaces:**
- Consumes: `novel_ide_lib::db::models::chapter::{Chapter, CreateChapterRequest, UpdateChapterRequest}`, `novel_ide_lib::db::models::project::{CreateProjectRequest, Project}`
- Produces: 7 test functions covering all CRUD + error paths

- [ ] **Step 1: Add chapter_test module to mod.rs**

Edit `src-tauri/tests/commands/mod.rs`:

```rust
mod project_test;
mod chapter_test;
```

- [ ] **Step 2: Create chapter_test.rs with setup helper**

```rust
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tempfile::TempDir;

use novel_ide_lib::db::models::chapter::{Chapter, CreateChapterRequest, UpdateChapterRequest};
use novel_ide_lib::db::models::project::{CreateProjectRequest, Project};

async fn setup_test_db() -> SqlitePool {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../../migrations/001_initial.sql"))
        .execute(&pool)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../../migrations/002_chapters.sql"))
        .execute(&pool)
        .await
        .unwrap();

    pool
}

async fn create_test_project(db: &SqlitePool) -> Project {
    let temp_dir = TempDir::new().unwrap();
    let req = CreateProjectRequest {
        name: "测试项目".to_string(),
        path: temp_dir.path().join("test-project").to_string_lossy().to_string(),
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };
    Project::create(db, &req).await.unwrap()
}
```

- [ ] **Step 3: Add test_create_chapter**

```rust
#[tokio::test]
async fn test_create_chapter() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("第一章 初入江湖".to_string()),
        content: Some("少年负剑而行。".to_string()),
        sort_order: Some(1),
    };

    let chapter = Chapter::create(&db, &req).await.unwrap();

    assert!(!chapter.id.is_empty());
    assert_eq!(chapter.project_id, project.id);
    assert_eq!(chapter.title, "第一章 初入江湖");
    assert_eq!(chapter.content, "少年负剑而行。");
    assert_eq!(chapter.sort_order, 1);
    assert_eq!(chapter.word_count, 7);
}
```

- [ ] **Step 4: Add test_create_chapter_defaults**

```rust
#[tokio::test]
async fn test_create_chapter_defaults() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: None,
        content: None,
        sort_order: None,
    };

    let chapter = Chapter::create(&db, &req).await.unwrap();

    assert_eq!(chapter.title, "未命名章节");
    assert_eq!(chapter.content, "");
    assert_eq!(chapter.sort_order, 0);
    assert_eq!(chapter.word_count, 0);
}
```

- [ ] **Step 5: Add test_list_chapters_by_project**

```rust
#[tokio::test]
async fn test_list_chapters_by_project() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req1 = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("第二章".to_string()),
        content: None,
        sort_order: Some(2),
    };
    let req2 = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("第一章".to_string()),
        content: None,
        sort_order: Some(1),
    };

    Chapter::create(&db, &req1).await.unwrap();
    Chapter::create(&db, &req2).await.unwrap();

    let chapters = Chapter::list_by_project(&db, &project.id).await.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0].title, "第一章");
    assert_eq!(chapters[1].title, "第二章");
}
```

- [ ] **Step 6: Add test_find_chapter_by_id**

```rust
#[tokio::test]
async fn test_find_chapter_by_id() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("查找测试".to_string()),
        content: Some("内容".to_string()),
        sort_order: None,
    };

    let created = Chapter::create(&db, &req).await.unwrap();
    let found = Chapter::find_by_id(&db, &created.id).await.unwrap();

    assert_eq!(found.id, created.id);
    assert_eq!(found.title, "查找测试");
}
```

- [ ] **Step 7: Add test_find_nonexistent_chapter**

```rust
#[tokio::test]
async fn test_find_nonexistent_chapter() {
    let db = setup_test_db().await;
    let result = Chapter::find_by_id(&db, "nonexistent-id").await;
    assert!(result.is_err());
}
```

- [ ] **Step 8: Add test_update_chapter**

```rust
#[tokio::test]
async fn test_update_chapter() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("原始标题".to_string()),
        content: Some("原始内容".to_string()),
        sort_order: Some(1),
    };

    let created = Chapter::create(&db, &req).await.unwrap();

    let update_req = UpdateChapterRequest {
        title: Some("更新标题".to_string()),
        content: Some("更新内容，增加了字数。".to_string()),
        sort_order: Some(5),
    };

    let updated = Chapter::update(&db, &created.id, &update_req).await.unwrap();

    assert_eq!(updated.title, "更新标题");
    assert_eq!(updated.content, "更新内容，增加了字数。");
    assert_eq!(updated.sort_order, 5);
    assert_eq!(updated.word_count, 11);
}
```

- [ ] **Step 9: Add test_update_chapter_partial**

```rust
#[tokio::test]
async fn test_update_chapter_partial() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("原标题".to_string()),
        content: Some("原内容".to_string()),
        sort_order: Some(1),
    };

    let created = Chapter::create(&db, &req).await.unwrap();

    let update_req = UpdateChapterRequest {
        title: Some("新标题".to_string()),
        content: None,
        sort_order: None,
    };

    let updated = Chapter::update(&db, &created.id, &update_req).await.unwrap();

    assert_eq!(updated.title, "新标题");
    assert_eq!(updated.content, "原内容");
    assert_eq!(updated.sort_order, 1);
}
```

- [ ] **Step 10: Add test_delete_chapter**

```rust
#[tokio::test]
async fn test_delete_chapter() {
    let db = setup_test_db().await;
    let project = create_test_project(&db).await;

    let req = CreateChapterRequest {
        project_id: project.id.clone(),
        title: Some("删除测试".to_string()),
        content: None,
        sort_order: None,
    };

    let created = Chapter::create(&db, &req).await.unwrap();
    Chapter::delete(&db, &created.id).await.unwrap();

    let result = Chapter::find_by_id(&db, &created.id).await;
    assert!(result.is_err());
}
```

- [ ] **Step 11: Add test_delete_nonexistent_chapter**

```rust
#[tokio::test]
async fn test_delete_nonexistent_chapter() {
    let db = setup_test_db().await;
    let result = Chapter::delete(&db, "nonexistent-id").await;
    assert!(result.is_err());
}
```

- [ ] **Step 12: Run tests**

Run: `cargo test --test chapter_test` from `src-tauri/`
Expected: 9 tests PASS

- [ ] **Step 13: Commit**

```bash
git add src-tauri/tests/commands/chapter_test.rs src-tauri/tests/commands/mod.rs
git commit -m "test: add chapter model CRUD tests"
```
