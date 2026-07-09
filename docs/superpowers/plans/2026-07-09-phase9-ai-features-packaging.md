# Phase 9: AI Features + Packaging Optimization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement advanced AI writing features (续写、改写、扩写) and optimize packaging for all platforms (Windows NSIS, macOS DMG, Linux AppImage).

**Architecture:** AI features use streaming chat with specialized prompts for each operation. Packaging uses Tauri 2's bundler with platform-specific configurations.

**Tech Stack:** Rust 1.90+ (reqwest, tokio), Tauri 2.11, Vue 3.5.39, TypeScript 5.7+, Bun 1.2+

## Global Constraints
- Rust 1.90+, Tauri 2.11, SQLx 0.9.0
- Vue 3.5.39, Pinia 3.0.4, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- Platforms: Windows (NSIS + MSI), macOS (DMG for Intel + Apple Silicon), Linux (AppImage + deb + pkg-tar.zst)
- AI features: 续写 (continue writing), 改写 (rewrite), 扩写 (expand), 缩写 (condense), 风格转换 (style transfer)

## File Structure

```
src-tauri/
  src/
    commands/
      ai.rs            — Add continue_writing, rewrite, expand, condense, style_transfer commands
    services/
      ai_features.rs   — AI writing feature implementations
  tauri.conf.json      — Update bundle targets
  Cargo.toml           — Add bundler dependencies
src/
  components/
    ai/
      AIWritePanel.vue — AI writing features UI
    editor/
      EditorPanel.vue  — Add AI writing toolbar buttons
  stores/
    ai.ts              — Add AI writing actions
```

## Tasks

### Task 1: AI Features Service

**Files:**
- Create: `src-tauri/src/services/ai_features.rs`
- Modify: `src-tauri/src/services/mod.rs`

**Steps:**

- [ ] **Step 1: Create ai_features.rs**

```rust
use crate::error::AppError;
use crate::state::AppState;
use reqwest::Client;
use serde_json::json;

pub struct AIFeaturesService<'a> {
    state: &'a AppState,
    client: Client,
}

impl<'a> AIFeaturesService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            client: Client::new(),
        }
    }

    pub async fn continue_writing(
        &self,
        content: &str,
        provider_id: &str,
        style: Option<&str>,
    ) -> Result<String, AppError> {
        let style_prompt = style.unwrap_or("保持原文风格");
        let prompt = format!(
            r#"请继续以下小说内容，{style_prompt}，保持情节连贯，字数约500字：

{content}

请直接输出续写内容，不要添加任何解释或前缀。"#,
            style_prompt = style_prompt,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn rewrite(
        &self,
        content: &str,
        provider_id: &str,
        instruction: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请改写以下内容，要求：{instruction}

原文：
{content}

请直接输出改写后的内容，不要添加任何解释或前缀。"#,
            instruction = instruction,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn expand(
        &self,
        content: &str,
        provider_id: &str,
        target_words: Option<u32>,
    ) -> Result<String, AppError> {
        let words = target_words.unwrap_or(1000);
        let prompt = format!(
            r#"请扩写以下内容，目标字数约{words}字，丰富细节描写、心理活动和环境描写：

{content}

请直接输出扩写后的内容，不要添加任何解释或前缀。"#,
            words = words,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn condense(
        &self,
        content: &str,
        provider_id: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请精简以下内容，保留核心情节和人物，删除冗余描写：

{content}

请直接输出精简后的内容，不要添加任何解释或前缀。"#,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn style_transfer(
        &self,
        content: &str,
        provider_id: &str,
        target_style: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请将以下内容转换为{target_style}风格：

{content}

请直接输出转换后的内容，不要添加任何解释或前缀。"#,
            target_style = target_style,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    async fn call_ai(&self, prompt: &str, provider_id: &str) -> Result<String, AppError> {
        let db = self.state.db().await?;
        let row = sqlx::query("SELECT * FROM model_providers WHERE id = ?")
            .bind(provider_id)
            .fetch_optional(&db)
            .await?
            .ok_or(AppError::ProviderNotFound(provider_id.into()))?;

        let base_url: String = row.get("base_url");
        let api_key: String = row.get("api_key");
        let model_name: String = row.get("model_name");

        let body = json!({
            "model": model_name,
            "messages": [
                {"role": "system", "content": "你是一个专业的中文小说写作助手，擅长各种写作风格。"},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.7,
            "max_tokens": 2000,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
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
}
```

- [ ] **Step 2: Register in mod.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/services/ai_features.rs src-tauri/src/services/mod.rs
git commit -m "feat: add AI writing features service"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 2: AI Features Commands

**Files:**
- Modify: `src-tauri/src/commands/ai.rs`
- Modify: `src-tauri/src/lib.rs`

**Steps:**

- [ ] **Step 1: Add commands to ai.rs**

```rust
use crate::error::AppError;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn continue_writing(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    style: Option<String>,
) -> Result<String, AppError> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.continue_writing(&content, &provider_id, style.as_deref()).await
}

#[tauri::command]
pub async fn rewrite_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    instruction: String,
) -> Result<String, AppError> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.rewrite(&content, &provider_id, &instruction).await
}

#[tauri::command]
pub async fn expand_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    target_words: Option<u32>,
) -> Result<String, AppError> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.expand(&content, &provider_id, target_words).await
}

#[tauri::command]
pub async fn condense_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
) -> Result<String, AppError> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.condense(&content, &provider_id).await
}

#[tauri::command]
pub async fn style_transfer(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    target_style: String,
) -> Result<String, AppError> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.style_transfer(&content, &provider_id, &target_style).await
}
```

- [ ] **Step 2: Register in lib.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/ai.rs src-tauri/src/lib.rs
git commit -m "feat: add AI writing feature commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 3: AI Features Store

**Files:**
- Modify: `src/stores/ai.ts`

**Steps:**

- [ ] **Step 1: Add AI writing actions to ai.ts**

```typescript
// Add to useAIStore
async function continueWriting(content: string, providerId: string, style?: string) {
  return await call<string>('continue_writing', { content, providerId, style })
}

async function rewriteContent(content: string, providerId: string, instruction: string) {
  return await call<string>('rewrite_content', { content, providerId, instruction })
}

async function expandContent(content: string, providerId: string, targetWords?: number) {
  return await call<string>('expand_content', { content, providerId, targetWords })
}

async function condenseContent(content: string, providerId: string) {
  return await call<string>('condense_content', { content, providerId })
}

async function styleTransfer(content: string, providerId: string, targetStyle: string) {
  return await call<string>('style_transfer', { content, providerId, targetStyle })
}
```

- [ ] **Step 2: Commit**

```bash
git add src/stores/ai.ts
git commit -m "feat: add AI writing feature store actions"
```

**Verification:** `bun run build`

---

### Task 4: AI Write Panel UI

**Files:**
- Create: `src/components/ai/AIWritePanel.vue`
- Modify: `src/components/layout/AIPanel.vue`

**Steps:**

- [ ] **Step 1: Create AIWritePanel.vue**

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAIStore } from '../../stores/ai'
import { useModelStore } from '../../stores/model'
import { useChapterStore } from '../../stores/chapter'

const aiStore = useAIStore()
const modelStore = useModelStore()
const chapterStore = useChapterStore()

const selectedFeature = ref('continue')
const instruction = ref('')
const targetWords = ref(1000)
const targetStyle = ref('古风')
const result = ref('')
const isProcessing = ref(false)

const features = [
  { id: 'continue', name: '续写', icon: '✍️', description: '继续当前内容' },
  { id: 'rewrite', name: '改写', icon: '🔄', description: '按要求改写内容' },
  { id: 'expand', name: '扩写', icon: '📝', description: '丰富细节描写' },
  { id: 'condense', name: '缩写', icon: '✂️', description: '精简内容' },
  { id: 'style', name: '风格转换', icon: '🎨', description: '转换写作风格' },
]

const stylePresets = [
  '古风', '现代', '悬疑', '科幻', '奇幻', '武侠', '都市', '历史'
]

const currentContent = computed(() => {
  return chapterStore.currentChapter?.content || ''
})

async function processContent() {
  if (!currentContent.value || !modelStore.currentProvider) return
  
  isProcessing.value = true
  try {
    const providerId = modelStore.currentProvider.id
    
    switch (selectedFeature.value) {
      case 'continue':
        result.value = await aiStore.continueWriting(currentContent.value, providerId)
        break
      case 'rewrite':
        if (!instruction.value) return
        result.value = await aiStore.rewriteContent(currentContent.value, providerId, instruction.value)
        break
      case 'expand':
        result.value = await aiStore.expandContent(currentContent.value, providerId, targetWords.value)
        break
      case 'condense':
        result.value = await aiStore.condenseContent(currentContent.value, providerId)
        break
      case 'style':
        result.value = await aiStore.styleTransfer(currentContent.value, providerId, targetStyle.value)
        break
    }
  } finally {
    isProcessing.value = false
  }
}

function applyResult() {
  if (result.value && chapterStore.currentChapter) {
    chapterStore.updateChapter(chapterStore.currentChapter.id, {
      content: currentContent.value + '\n\n' + result.value
    })
    result.value = ''
  }
}
</script>

<template>
  <div class="ai-write-panel">
    <div class="panel-header">
      <h3>✨ AI 写作</h3>
    </div>

    <div class="feature-grid">
      <button 
        v-for="feature in features" 
        :key="feature.id"
        class="feature-btn"
        :class="{ active: selectedFeature === feature.id }"
        @click="selectedFeature = feature.id"
      >
        <span class="feature-icon">{{ feature.icon }}</span>
        <span class="feature-name">{{ feature.name }}</span>
      </button>
    </div>

    <div class="feature-options">
      <div v-if="selectedFeature === 'rewrite'" class="option-group">
        <label>改写要求</label>
        <textarea v-model="instruction" placeholder="例如：让对话更生动..." rows="3" />
      </div>

      <div v-if="selectedFeature === 'expand'" class="option-group">
        <label>目标字数: {{ targetWords }}</label>
        <input type="range" v-model.number="targetWords" min="500" max="3000" step="100" />
      </div>

      <div v-if="selectedFeature === 'style'" class="option-group">
        <label>目标风格</label>
        <div class="style-presets">
          <button 
            v-for="style in stylePresets" 
            :key="style"
            class="style-btn"
            :class="{ active: targetStyle === style }"
            @click="targetStyle = style"
          >
            {{ style }}
          </button>
        </div>
      </div>
    </div>

    <button 
      class="process-btn"
      :disabled="isProcessing || !currentContent"
      @click="processContent"
    >
      {{ isProcessing ? '处理中...' : '开始处理' }}
    </button>

    <div v-if="result" class="result-section">
      <div class="result-header">
        <h4>生成结果</h4>
        <button class="apply-btn" @click="applyResult">应用到章节</button>
      </div>
      <div class="result-content">{{ result }}</div>
    </div>
  </div>
</template>

<style scoped>
.ai-write-panel {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.panel-header {
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  margin-bottom: 16px;
}

.feature-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 8px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.feature-btn:hover {
  border-color: var(--accent);
}

.feature-btn.active {
  border-color: var(--accent);
  background: var(--accent);
  color: white;
}

.feature-icon {
  font-size: 20px;
  margin-bottom: 4px;
}

.feature-name {
  font-size: 12px;
}

.feature-options {
  margin-bottom: 16px;
}

.option-group {
  margin-bottom: 12px;
}

.option-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.option-group textarea,
.option-group input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  box-sizing: border-box;
}

.style-presets {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.style-btn {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 16px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.style-btn:hover {
  border-color: var(--accent);
}

.style-btn.active {
  border-color: var(--accent);
  background: var(--accent);
  color: white;
}

.process-btn {
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 8px;
  background: var(--accent);
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  margin-bottom: 16px;
}

.process-btn:hover:not(:disabled) {
  background: var(--accent-hover);
}

.process-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.result-section {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
}

.result-header h4 {
  margin: 0;
  font-size: 14px;
}

.apply-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  background: var(--success);
  color: white;
  cursor: pointer;
  font-size: 12px;
}

.result-content {
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
  max-height: 300px;
  overflow-y: auto;
}
</style>
```

- [ ] **Step 2: Add AIWritePanel to AIPanel.vue as a tab**

- [ ] **Step 3: Commit**

```bash
git add src/components/ai/AIWritePanel.vue src/components/layout/AIPanel.vue
git commit -m "feat: add AI write panel UI"
```

**Verification:** `bun run build`

---

### Task 5: Editor AI Toolbar

**Files:**
- Modify: `src/components/layout/EditorPanel.vue`

**Steps:**

- [ ] **Step 1: Add AI writing buttons to EditorPanel toolbar**

Add buttons for:
- ✍️ 续写
- 🔄 改写
- 📝 扩写
- ✂️ 缩写
- 🎨 风格

Each button should open the AI panel with the corresponding feature selected.

- [ ] **Step 2: Commit**

```bash
git add src/components/layout/EditorPanel.vue
git commit -m "feat: add AI writing toolbar buttons"
```

**Verification:** `bun run build`

---

### Task 6: Packaging Configuration

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src-tauri/Cargo.toml`

**Steps:**

- [ ] **Step 1: Update tauri.conf.json bundle targets**

```json
{
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "nsis": {
        "installMode": "both",
        "languages": ["zh_CN", "en_US"]
      },
      "msi": {
        "languages": ["zh-CN", "en-US"]
      }
    },
    "macOS": {
      "minimumSystemVersion": "10.15",
      "dmg": {
        "appPosition": { "x": 180, "y": 170 },
        "applicationFolderPosition": { "x": 480, "y": 170 },
        "windowSize": { "width": 660, "height": 400 }
      }
    },
    "linux": {
      "appimage": {
        "bundleMediaFramework": false
      },
      "deb": {
        "depends": ["libwebkit2gtk-4.1-dev", "libgtk-3-dev", "libappindicator3-dev"]
      }
    }
  }
}
```

- [ ] **Step 2: Add bundler dependencies to Cargo.toml**

```toml
[dependencies]
tauri-plugin-global-shortcut = "2"
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-store = "2"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "s"
strip = true
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json src-tauri/Cargo.toml
git commit -m "feat: configure packaging for all platforms"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 7: Windows Packaging

**Files:**
- Create: `src-tauri/nsis/` directory with custom NSIS scripts (if needed)
- Modify: `src-tauri/tauri.conf.json`

**Steps:**

- [ ] **Step 1: Configure Windows NSIS installer**

Update tauri.conf.json for Windows-specific settings:
- Custom installer icon
- Start menu shortcut
- Desktop shortcut
- Chinese language support

- [ ] **Step 2: Test Windows build**

```bash
cargo tauri build --target x86_64-pc-windows-msvc
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/
git commit -m "feat: configure Windows NSIS packaging"
```

**Verification:** Build succeeds on Windows

---

### Task 8: macOS Packaging

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Create: `src-tauri/Info.plist` (if needed)

**Steps:**

- [ ] **Step 1: Configure macOS DMG**

Update tauri.conf.json for macOS:
- Universal binary (Intel + Apple Silicon)
- Code signing (optional)
- Notarization (optional)
- Custom DMG background

- [ ] **Step 2: Test macOS build**

```bash
cargo tauri build --target universal-apple-darwin
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/
git commit -m "feat: configure macOS DMG packaging"
```

**Verification:** Build succeeds on macOS

---

### Task 9: Linux Packaging

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Create: `src-tauri/debian/` directory (if needed)

**Steps:**

- [ ] **Step 1: Configure Linux AppImage and deb**

Update tauri.conf.json for Linux:
- AppImage with bundled dependencies
- deb package with proper dependencies
- Desktop file and icons

- [ ] **Step 2: Test Linux build**

```bash
cargo tauri build --target x86_64-unknown-linux-gnu
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/
git commit -m "feat: configure Linux AppImage and deb packaging"
```

**Verification:** Build succeeds on Linux

---

### Task 10: Integration Verification

**Files:**
- None (verification only)

**Steps:**

- [ ] **Step 1: Verify all commands in lib.rs**

Count should now be ~60+.

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
Task 1: feat: add AI writing features service
Task 2: feat: add AI writing feature commands
Task 3: feat: add AI writing feature store actions
Task 4: feat: add AI write panel UI
Task 5: feat: add AI writing toolbar buttons
Task 6: feat: configure packaging for all platforms
Task 7: feat: configure Windows NSIS packaging
Task 8: feat: configure macOS DMG packaging
Task 9: feat: configure Linux AppImage and deb packaging
Task 10: (verification only)
```
