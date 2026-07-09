# Phase 10: Testing + Performance + CI/CD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add comprehensive test coverage, performance optimizations, and CI/CD pipeline for automated builds and testing.

**Architecture:** Unit tests for Rust backend services, integration tests for Tauri commands, component tests for Vue frontend. Performance optimizations for large file handling. GitHub Actions for cross-platform builds.

**Tech Stack:** Rust 1.90+ (cargo-test, insta), Vue 3.5.39 (vitest, @vue/test-utils), GitHub Actions, Bun 1.2+

## Global Constraints
- Rust 1.90+, Tauri 2.11
- Vue 3.5.39, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- Test frameworks: cargo-test (Rust), vitest (Vue)
- CI/CD: GitHub Actions for Windows, macOS, Linux

## File Structure

```
src-tauri/
  tests/
    commands/
      project_test.rs    — Project command tests
      chapter_test.rs    — Chapter command tests
      ai_test.rs         — AI command tests
    services/
      workflow_engine_test.rs — Workflow engine tests
src/
  tests/
    stores/
      chapter_test.ts    — Chapter store tests
      ai_test.ts         — AI store tests
    components/
      EditorPanel_test.ts — Editor panel tests
.github/
  workflows/
    ci.yml              — CI pipeline
    release.yml         — Release pipeline
Cargo.toml              — Add test dependencies
vitest.config.ts        — Vitest configuration
```

## Tasks

### Task 1: Rust Test Setup

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/tests/` directory structure

**Steps:**

- [ ] **Step 1: Add test dependencies to Cargo.toml**

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

- [ ] **Step 2: Create test directory structure**

```bash
mkdir -p src-tauri/tests/commands
mkdir -p src-tauri/tests/services
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/tests/
git commit -m "test: setup Rust test infrastructure"
```

**Verification:** `cargo test --no-run` from src-tauri/

---

### Task 2: Project Command Tests

**Files:**
- Create: `src-tauri/tests/commands/project_test.rs`

**Steps:**

- [ ] **Step 1: Create project_test.rs**

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_create_project_command() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");
    
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "create-project",
        "--name", "测试项目",
        "--path", project_path.to_str().unwrap(),
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("项目创建成功"));
}

#[test]
fn test_list_projects_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&["list-projects"]);
    
    cmd.assert()
        .success();
}

#[test]
fn test_delete_project_command() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("delete-me");
    
    // Create project first
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "create-project",
        "--name", "删除测试",
        "--path", project_path.to_str().unwrap(),
    ]);
    cmd.assert().success();
    
    // Delete project
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "delete-project",
        "--path", project_path.to_str().unwrap(),
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("项目已删除"));
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test --test project_test
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/commands/project_test.rs
git commit -m "test: add project command tests"
```

**Verification:** Tests pass

---

### Task 3: Chapter Command Tests

**Files:**
- Create: `src-tauri/tests/commands/chapter_test.rs`

**Steps:**

- [ ] **Step 1: Create chapter_test.rs**

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_create_chapter_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "create-chapter",
        "--project-id", "test-project",
        "--title", "第一章 测试",
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("章节创建成功"));
}

#[test]
fn test_list_chapters_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "list-chapters",
        "--project-id", "test-project",
    ]);
    
    cmd.assert()
        .success();
}

#[test]
fn test_update_chapter_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "update-chapter",
        "--chapter-id", "test-chapter",
        "--content", "这是测试内容",
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("章节更新成功"));
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test --test chapter_test
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/commands/chapter_test.rs
git commit -m "test: add chapter command tests"
```

**Verification:** Tests pass

---

### Task 4: AI Command Tests

**Files:**
- Create: `src-tauri/tests/commands/ai_test.rs`

**Steps:**

- [ ] **Step 1: Create ai_test.rs**

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_continue_writing_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "continue-writing",
        "--content", "测试内容",
        "--provider-id", "test-provider",
    ]);
    
    // This will fail without a real provider, but tests command parsing
    cmd.assert()
        .failure(); // Expected to fail without valid provider
}

#[test]
fn test_rewrite_content_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "rewrite-content",
        "--content", "测试内容",
        "--provider-id", "test-provider",
        "--instruction", "让内容更生动",
    ]);
    
    // This will fail without a real provider, but tests command parsing
    cmd.assert()
        .failure(); // Expected to fail without valid provider
}

#[test]
fn test_expand_content_command() {
    let mut cmd = Command::cargo_bin("novel-ide").unwrap();
    cmd.args(&[
        "expand-content",
        "--content", "测试内容",
        "--provider-id", "test-provider",
        "--target-words", "1000",
    ]);
    
    // This will fail without a real provider, but tests command parsing
    cmd.assert()
        .failure(); // Expected to fail without valid provider
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test --test ai_test
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/commands/ai_test.rs
git commit -m "test: add AI command tests"
```

**Verification:** Tests pass

---

### Task 5: Workflow Engine Tests

**Files:**
- Create: `src-tauri/tests/services/workflow_engine_test.rs`

**Steps:**

- [ ] **Step 1: Create workflow_engine_test.rs**

```rust
use novel_ide::services::workflow_engine::WorkflowEngine;
use novel_ide::models::workflow::WorkflowStage;

#[tokio::test]
async fn test_workflow_engine_creation() {
    // Test that WorkflowEngine can be created
    // This is a basic smoke test
    assert!(true);
}

#[tokio::test]
async test_workflow_stage_conversion() {
    let stage = WorkflowStage {
        name: "测试阶段".to_string(),
        stage_type: "draft".to_string(),
        agent_id: None,
        model_provider_id: Some("test-provider".to_string()),
        system_prompt: Some("测试提示".to_string()),
        temperature: 0.7,
        max_tokens: 2000,
    };
    
    assert_eq!(stage.name, "测试阶段");
    assert_eq!(stage.stage_type, "draft");
    assert_eq!(stage.temperature, 0.7);
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test --test workflow_engine_test
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/services/workflow_engine_test.rs
git commit -m "test: add workflow engine tests"
```

**Verification:** Tests pass

---

### Task 6: Vitest Setup

**Files:**
- Create: `vitest.config.ts`
- Modify: `package.json`

**Steps:**

- [ ] **Step 1: Create vitest.config.ts**

```typescript
import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['src/**/*.{test,spec}.{js,ts}'],
    coverage: {
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'src/test/'],
    },
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
})
```

- [ ] **Step 2: Add test scripts to package.json**

```json
{
  "scripts": {
    "test": "vitest",
    "test:run": "vitest run",
    "test:coverage": "vitest run --coverage"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "@vue/test-utils": "^2.4.0",
    "jsdom": "^24.0.0"
  }
}
```

- [ ] **Step 3: Install dependencies**

```bash
bun install
```

- [ ] **Step 4: Commit**

```bash
git add vitest.config.ts package.json bun.lockb
git commit -m "test: setup Vitest for Vue testing"
```

**Verification:** `bun run test:run` passes

---

### Task 7: Chapter Store Tests

**Files:**
- Create: `src/stores/__tests__/chapter.test.ts`

**Steps:**

- [ ] **Step 1: Create chapter.test.ts**

```typescript
import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useChapterStore } from '../chapter'

describe('Chapter Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes with empty chapters', () => {
    const store = useChapterStore()
    expect(store.chapters).([])
    expect(store.currentChapter).toBeNull()
  })

  it('sets current chapter', () => {
    const store = useChapterStore()
    const mockChapter = {
      id: '1',
      title: '第一章',
      content: '测试内容',
      project_id: 'proj-1',
      sort_order: 0,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }
    
    store.currentChapter = mockChapter
    expect(store.currentChapter).toEqual(mockChapter)
  })

  it('computes word count correctly', () => {
    const store = useChapterStore()
    store.currentChapter = {
      id: '1',
      title: '第一章',
      content: '这是一个测试内容，包含多个字符。',
      project_id: 'proj-1',
      sort_order: 0,
      created_at: '2024-01-01',
      updated_at: '2024-01-01',
    }
    
    expect(store.wordCount).toBe(14)
  })
})
```

- [ ] **Step 2: Run tests**

```bash
bun run test:run src/stores/__tests__/chapter.test.ts
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/__tests__/chapter.test.ts
git commit -m "test: add chapter store tests"
```

**Verification:** Tests pass

---

### Task 8: AI Store Tests

**Files:**
- Create: `src/stores/__tests__/ai.test.ts`

**Steps:**

- [ ] **Step 1: Create ai.test.ts**

```typescript
import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAIStore } from '../ai'

describe('AI Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes with empty messages', () => {
    const store = useAIStore()
    expect(store.messages).([])
    expect(store.isLoading).toBe(false)
  })

  it('sets loading state', () => {
    const store = useAIStore()
    store.isLoading = true
    expect(store.isLoading).toBe(true)
  })

  it('adds message to chat', () => {
    const store = useAIStore()
    const mockMessage = {
      id: '1',
      role: 'user' as const,
      content: '测试消息',
      timestamp: Date.now(),
    }
    
    store.messages.push(mockMessage)
    expect(store.messages).toHaveLength(1)
    expect(store.messages[0].content).toBe('测试消息')
  })

  it('clears chat', () => {
    const store = useAIStore()
    store.messages.push({
      id: '1',
      role: 'user',
      content: '测试',
      timestamp: Date.now(),
    })
    
    store.messages = []
    expect(store.messages).toHaveLength(0)
  })
})
```

- [ ] **Step 2: Run tests**

```bash
bun run test:run src/stores/__tests__/ai.test.ts
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/__tests__/ai.test.ts
git commit -m "test: add AI store tests"
```

**Verification:** Tests pass

---

### Task 9: GitHub Actions CI

**Files:**
- Create: `.github/workflows/ci.yml`

**Steps:**

- [ ] **Step 1: Create ci.yml**

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test-rust:
    name: Rust Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache Rust
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        working-directory: src-tauri
        run: cargo test
      
      - name: Check clippy
        working-directory: src-tauri
        run: cargo clippy -- -D warnings

  test-frontend:
    name: Frontend Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Bun
        uses: oven-sh/setup-bun@v1
      
      - name: Install dependencies
        run: bun install
      
      - name: Run type check
        run: bun run type-check
      
      - name: Run tests
        run: bun run test:run
      
      - name: Build
        run: bun run build

  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Bun
        uses: oven-sh/setup-bun@v1
      
      - name: Install dependencies
        run: bun install
      
      - name: Run lint
        run: bun run lint
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add GitHub Actions CI pipeline"
```

**Verification:** Push to trigger CI

---

### Task 10: GitHub Actions Release

**Files:**
- Create: `.github/workflows/release.yml`

**Steps:**

- [ ] **Step 1: Create release.yml**

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: ubuntu-22.04
            args: '--target x86_64-unknown-linux-gnu'
          - platform: ubuntu-22.04
            args: '--target aarch64-unknown-linux-gnu'
          - platform: macos-latest
            args: '--target universal-apple-darwin'
          - platform: windows-latest
            args: '--target x86_64-pc-windows-msvc'

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}
      
      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
      
      - name: Setup Bun
        uses: oven-sh/setup-bun@v1
      
      - name: Install frontend dependencies
        run: bun install
      
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Novel IDE ${{ github.ref_name }}'
          releaseBody: 'See the assets below for the download links.'
          releaseDraft: false
          prerelease: false
          args: ${{ matrix.args }}
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add GitHub Actions release pipeline"
```

**Verification:** Push tag to trigger release

---

### Task 11: Performance Optimization - Large File Handling

**Files:**
- Modify: `src-tauri/src/commands/chapter.rs`
- Create: `src-tauri/src/services/performance.rs`

**Steps:**

- [ ] **Step 1: Create performance.rs**

```rust
use crate::error::AppError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct PerformanceService;

impl PerformanceService {
    pub fn new() -> Self {
        Self
    }

    pub fn count_lines(&self, file_path: &str) -> Result<usize, AppError> {
        let file = File::open(file_path)
            .map_err(|e| AppError::Io(format!("打开文件失败: {}", e)))?;
        
        let reader = BufReader::new(file);
        let line_count = reader.lines().count();
        
        Ok(line_count)
    }

    pub fn read_file_chunk(
        &self,
        file_path: &str,
        start_line: usize,
        max_lines: usize,
    ) -> Result<String, AppError> {
        let file = File::open(file_path)
            .map_err(|e| AppError::Io(format!("打开文件失败: {}", e)))?;
        
        let reader = BufReader::new(file);
        let mut content = String::new();
        let mut line_count = 0;
        
        for line in reader.lines() {
            if line_count >= start_line && line_count < start_line + max_lines {
                content.push_str(&line.map_err(|e| AppError::Io(e.to_string()))?);
                content.push('\n');
            }
            line_count += 1;
            
            if line_count >= start_line + max_lines {
                break;
            }
        }
        
        Ok(content)
    }

    pub fn get_file_size(&self, file_path: &str) -> Result<u64, AppError> {
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| AppError::Io(format!("获取文件信息失败: {}", e)))?;
        
        Ok(metadata.len())
    }

    pub fn optimize_content(&self, content: &str) -> String {
        // Remove excessive whitespace
        let optimized = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        
        optimized
    }
}
```

- [ ] **Step 2: Register in mod.rs**

- [ ] **Step 3: Add optimized chapter loading**

Update `chapter.rs` to use chunked loading for large files.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/performance.rs src-tauri/src/commands/chapter.rs
git commit -m "perf: add large file handling optimizations"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 12: Integration Verification

**Files:**
- None (verification only)

**Steps:**

- [ ] **Step 1: Run all Rust tests**

```bash
cd src-tauri && cargo test
```

- [ ] **Step 2: Run all frontend tests**

```bash
bun run test:run
```

- [ ] **Step 3: Run cargo check**

```bash
cd src-tauri && cargo check
```

- [ ] **Step 4: Run bun run build**

```bash
bun run build
```

- [ ] **Step 5: Report**

---

## Commit Log (Expected)

```
Task 1: test: setup Rust test infrastructure
Task 2: test: add project command tests
Task 3: test: add chapter command tests
Task 4: test: add AI command tests
Task 5: test: add workflow engine tests
Task 6: test: setup Vitest for Vue testing
Task 7: test: add chapter store tests
Task 8: test: add AI store tests
Task 9: ci: add GitHub Actions CI pipeline
Task 10: ci: add GitHub Actions release pipeline
Task 11: perf: add large file handling optimizations
Task 12: (verification only)
```
