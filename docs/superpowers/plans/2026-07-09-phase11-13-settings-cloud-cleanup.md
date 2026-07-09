# Phase 11-13: Settings + Cloud + Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement complete settings page, OSS/S3 cloud storage, and clean up dead code.

**Architecture:** Vue component for settings UI, Rust services for cloud storage, code cleanup for unused modules.

**Tech Stack:** Rust 1.90+, Tauri 2.11, Vue 3.5.39, TypeScript 5.7+, aliyun-oss, rusoto-s3

## Global Constraints
- Rust 1.90+, Tauri 2.11
- Vue 3.5.39, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- Use existing patterns from codebase

## File Structure

```
src-tauri/
  Cargo.toml              — Add cloud storage dependencies
  src/services/
    cloud_sync.rs         — Update with OSS/S3 support
    hyprland.rs           — DELETE (dead code)
    performance.rs        — Remove optimize_content method
src/
  components/settings/
    SettingsPanel.vue     — NEW: Complete settings page
  stores/
    settings.ts           — Update with new settings fields
```

## Tasks

### Task 1: Rust Dependencies Setup

**Files:**
- Modify: `src-tauri/Cargo.toml`

**Steps:**

- [ ] **Step 1: Add cloud storage dependencies**

```toml
[dependencies]
# ... existing dependencies ...
rusoto_s3 = "0.48.0"
rusoto_core = "0.48.0"
aliyun-oss = "0.10.0"
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check` from src-tauri/
Expected: Success (may have warnings about unused imports)

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml
git commit -m "deps: add cloud storage dependencies (rusoto, aliyun-oss)"
```

**Verification:** `cargo check` passes

---

### Task 2: Implement OSS Upload/Download

**Files:**
- Modify: `src-tauri/src/services/cloud_sync.rs`

**Steps:**

- [ ] **Step 1: Add OSS import and implementation**

```rust
use aliyun_oss::Client;

// Add to CloudSyncService impl block
pub async fn upload_to_oss(
    &self,
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    let client = Client::new(endpoint, bucket, access_key, secret_key)
        .map_err(|e| AppError::Internal(format!("OSS 客户端创建失败: {}", e)))?;
    let data = std::fs::read(local_path)?;
    client.put_object(remote_path, data)
        .await
        .map_err(|e| AppError::Internal(format!("OSS 上传失败: {}", e)))?;
    Ok(())
}

pub async fn download_from_oss(
    &self,
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    remote_path: &str,
    local_path: &str,
) -> Result<(), AppError> {
    let client = Client::new(endpoint, bucket, access_key, secret_key)
        .map_err(|e| AppError::Internal(format!("OSS 客户端创建失败: {}", e)))?;
    let data = client.get_object(remote_path)
        .await
        .map_err(|e| AppError::Internal(format!("OSS 下载失败: {}", e)))?;
    std::fs::write(local_path, data)?;
    Ok(())
}
```

- [ ] **Step 2: Update upload_file to support OSS**

```rust
pub async fn upload_file(
    &self,
    config: &CloudConfig,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    match &config.config {
        CloudProviderConfig::WebDAV { url, username, password } => {
            // Existing implementation
        }
        CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
            self.upload_to_oss(endpoint, bucket, access_key, secret_key, local_path, remote_path).await
        }
        CloudProviderConfig::S3 { .. } => {
            Err(AppError::Internal("S3 暂未实现".into()))
        }
    }
}
```

- [ ] **Step 3: Update download_file to support OSS**

```rust
pub async fn download_file(
    &self,
    config: &CloudConfig,
    remote_path: &str,
    local_path: &str,
) -> Result<(), AppError> {
    match &config.config {
        CloudProviderConfig::WebDAV { url, username, password } => {
            // Existing implementation
        }
        CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
            self.download_from_oss(endpoint, bucket, access_key, secret_key, remote_path, local_path).await
        }
        CloudProviderConfig::S3 { .. } => {
            Err(AppError::Internal("S3 暂未实现".into()))
        }
    }
}
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/cloud_sync.rs
git commit -m "feat: implement OSS upload/download in CloudSyncService"
```

**Verification:** `cargo check` passes

---

### Task 3: Implement S3 Upload/Download

**Files:**
- Modify: `src-tauri/src/services/cloud_sync.rs`

**Steps:**

- [ ] **Step 1: Add S3 import and implementation**

```rust
use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest};
use rusoto_core::{Region, credential::StaticProvider};

// Add to CloudSyncService impl block
pub async fn upload_to_s3(
    &self,
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    region: &str,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    let credentials = StaticProvider::new(
        access_key.into(),
        secret_key.into(),
        None,
        None,
    );
    let region = Region::Custom {
        endpoint: endpoint.into(),
        name: region.into(),
    };
    let http_client = rusoto_core::request::HttpClient::new()
        .map_err(|e| AppError::Internal(format!("HTTP 客户端创建失败: {}", e)))?;
    let client = S3Client::new_with_client(http_client, credentials, region);
    
    let data = std::fs::read(local_path)?;
    let req = PutObjectRequest {
        bucket: bucket.into(),
        key: remote_path.into(),
        body: Some(data.into()),
        ..Default::default()
    };
    
    client.put_object(req)
        .await
        .map_err(|e| AppError::Internal(format!("S3 上传失败: {}", e)))?;
    Ok(())
}

pub async fn download_from_s3(
    &self,
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    region: &str,
    remote_path: &str,
    local_path: &str,
) -> Result<(), AppError> {
    let credentials = StaticProvider::new(
        access_key.into(),
        secret_key.into(),
        None,
        None,
    );
    let region = Region::Custom {
        endpoint: endpoint.into(),
        name: region.into(),
    };
    let http_client = rusoto_core::request::HttpClient::new()
        .map_err(|e| AppError::Internal(format!("HTTP 客户端创建失败: {}", e)))?;
    let client = S3Client::new_with_client(http_client, credentials, region);
    
    let req = GetObjectRequest {
        bucket: bucket.into(),
        key: remote_path.into(),
        ..Default::default()
    };
    
    let result = client.get_object(req)
        .await
        .map_err(|e| AppError::Internal(format!("S3 下载失败: {}", e)))?;
    
    let bytes = result.body
        .ok_or_else(|| AppError::Internal("S3 响应无内容".into()))?
        .into_bytes()
        .await
        .map_err(|e| AppError::Internal(format!("S3 读取失败: {}", e)))?;
    
    std::fs::write(local_path, bytes)?;
    Ok(())
}
```

- [ ] **Step 2: Update upload_file to support S3**

```rust
pub async fn upload_file(
    &self,
    config: &CloudConfig,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    match &config.config {
        CloudProviderConfig::WebDAV { url, username, password } => {
            // Existing implementation
        }
        CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
            self.upload_to_oss(endpoint, bucket, access_key, secret_key, local_path, remote_path).await
        }
        CloudProviderConfig::S3 { endpoint, bucket, access_key, secret_key, region } => {
            self.upload_to_s3(endpoint, bucket, access_key, secret_key, region, local_path, remote_path).await
        }
    }
}
```

- [ ] **Step 3: Update download_file to support S3**

```rust
pub async fn download_file(
    &self,
    config: &CloudConfig,
    remote_path: &str,
    local_path: &str,
) -> Result<(), AppError> {
    match &config.config {
        CloudProviderConfig::WebDAV { url, username, password } => {
            // Existing implementation
        }
        CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
            self.download_from_oss(endpoint, bucket, access_key, secret_key, remote_path, local_path).await
        }
        CloudProviderConfig::S3 { endpoint, bucket, access_key, secret_key, region } => {
            self.download_from_s3(endpoint, bucket, access_key, secret_key, region, remote_path, local_path).await
        }
    }
}
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/cloud_sync.rs
git commit -m "feat: implement S3 upload/download in CloudSyncService"
```

**Verification:** `cargo check` passes

---

### Task 4: Settings Panel Component

**Files:**
- Create: `src/components/settings/SettingsPanel.vue`

**Steps:**

- [ ] **Step 1: Create SettingsPanel.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "../../stores/settings";

const settingsStore = useSettingsStore();

// Editor settings
const editorFont = ref("JetBrains Mono");
const editorFontSize = ref(14);
const editorLineHeight = ref(1.6);
const autoSaveInterval = ref(30);

// AI settings
const defaultModelId = ref("");
const aiTemperature = ref(0.7);
const aiMaxTokens = ref(2000);
const aiSystemPrompt = ref("你是一个专业的中文小说写作助手。");

// Export settings
const defaultExportFormat = ref("txt");
const exportPath = ref("");
const exportIncludeMetadata = ref(true);

const loading = ref(false);
const saving = ref(false);

onMounted(async () => {
  await settingsStore.fetchSettings();
  loadSettings();
});

const loadSettings = () => {
  editorFont.value = settingsStore.getSetting("editor_font") || "JetBrains Mono";
  editorFontSize.value = parseInt(settingsStore.getSetting("editor_font_size") || "14");
  editorLineHeight.value = parseFloat(settingsStore.getSetting("editor_line_height") || "1.6");
  autoSaveInterval.value = parseInt(settingsStore.getSetting("auto_save_interval") || "30");
  
  defaultModelId.value = settingsStore.getSetting("default_model_id") || "";
  aiTemperature.value = parseFloat(settingsStore.getSetting("ai_temperature") || "0.7");
  aiMaxTokens.value = parseInt(settingsStore.getSetting("ai_max_tokens") || "2000");
  aiSystemPrompt.value = settingsStore.getSetting("ai_system_prompt") || "你是一个专业的中文小说写作助手。";
  
  defaultExportFormat.value = settingsStore.getSetting("default_export_format") || "txt";
  exportPath.value = settingsStore.getSetting("export_path") || "";
  exportIncludeMetadata.value = settingsStore.getSetting("export_include_metadata") !== "false";
};

const saveSettings = async () => {
  saving.value = true;
  try {
    await settingsStore.updateSetting("editor_font", editorFont.value);
    await settingsStore.updateSetting("editor_font_size", editorFontSize.value.toString());
    await settingsStore.updateSetting("editor_line_height", editorLineHeight.value.toString());
    await settingsStore.updateSetting("auto_save_interval", autoSaveInterval.value.toString());
    
    await settingsStore.updateSetting("default_model_id", defaultModelId.value);
    await settingsStore.updateSetting("ai_temperature", aiTemperature.value.toString());
    await settingsStore.updateSetting("ai_max_tokens", aiMaxTokens.value.toString());
    await settingsStore.updateSetting("ai_system_prompt", aiSystemPrompt.value);
    
    await settingsStore.updateSetting("default_export_format", defaultExportFormat.value);
    await settingsStore.updateSetting("export_path", exportPath.value);
    await settingsStore.updateSetting("export_include_metadata", exportIncludeMetadata.value.toString());
  } finally {
    saving.value = false;
  }
};
</script>

<template>
  <div class="settings-panel">
    <div class="panel-header">设置</div>
    
    <div class="settings-section">
      <h3>编辑器设置</h3>
      <div class="setting-item">
        <label>字体</label>
        <select v-model="editorFont">
          <option value="JetBrains Mono">JetBrains Mono</option>
          <option value="Fira Code">Fira Code</option>
          <option value="Source Code Pro">Source Code Pro</option>
          <option value="Monaco">Monaco</option>
        </select>
      </div>
      <div class="setting-item">
        <label>字号 (px)</label>
        <input type="number" v-model="editorFontSize" min="10" max="24" />
      </div>
      <div class="setting-item">
        <label>行高</label>
        <input type="number" v-model="editorLineHeight" min="1" max="2" step="0.1" />
      </div>
      <div class="setting-item">
        <label>自动保存间隔 (秒)</label>
        <input type="number" v-model="autoSaveInterval" min="10" max="300" />
      </div>
    </div>
    
    <div class="settings-section">
      <h3>AI 设置</h3>
      <div class="setting-item">
        <label>默认模型</label>
        <input type="text" v-model="defaultModelId" placeholder="模型 ID" />
      </div>
      <div class="setting-item">
        <label>温度 (0-2)</label>
        <input type="number" v-model="aiTemperature" min="0" max="2" step="0.1" />
      </div>
      <div class="setting-item">
        <label>最大 Token</label>
        <input type="number" v-model="aiMaxTokens" min="100" max="8000" step="100" />
      </div>
      <div class="setting-item">
        <label>系统提示词</label>
        <textarea v-model="aiSystemPrompt" rows="3"></textarea>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>导出设置</h3>
      <div class="setting-item">
        <label>默认格式</label>
        <select v-model="defaultExportFormat">
          <option value="txt">TXT</option>
          <option value="md">Markdown</option>
          <option value="docx">DOCX</option>
          <option value="pdf">PDF</option>
          <option value="epub">EPUB</option>
        </select>
      </div>
      <div class="setting-item">
        <label>导出路径</label>
        <input type="text" v-model="exportPath" placeholder="/path/to/exports" />
      </div>
      <div class="setting-item">
        <label>
          <input type="checkbox" v-model="exportIncludeMetadata" />
          包含元数据
        </label>
      </div>
    </div>
    
    <div class="settings-actions">
      <button @click="saveSettings" :disabled="saving">
        {{ saving ? "保存中..." : "保存设置" }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  padding: var(--spacing-md);
}

.panel-header {
  font-size: var(--font-size-lg);
  font-weight: 600;
  margin-bottom: var(--spacing-lg);
}

.settings-section {
  margin-bottom: var(--spacing-xl);
  padding: var(--spacing-md);
  background: var(--bg-secondary);
  border-radius: 8px;
}

.settings-section h3 {
  font-size: var(--font-size-md);
  margin-bottom: var(--spacing-md);
  color: var(--text-primary);
}

.setting-item {
  display: flex;
  align-items: center;
  margin-bottom: var(--spacing-sm);
}

.setting-item label {
  width: 120px;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.setting-item input,
.setting-item select,
.setting-item textarea {
  flex: 1;
  padding: var(--spacing-sm);
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.setting-item textarea {
  resize: vertical;
  min-height: 60px;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
}

.settings-actions button {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--accent);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.settings-actions button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/settings/SettingsPanel.vue
git commit -m "feat: create SettingsPanel.vue component"
```

**Verification:** Component renders without errors

---

### Task 5: Update Sidebar to Use SettingsPanel

**Files:**
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Import SettingsPanel**

```vue
<script setup lang="ts">
// Add import
import SettingsPanel from "../settings/SettingsPanel.vue";
</script>
```

- [ ] **Step 2: Replace settings tab content**

```vue
<template>
  <!-- Find and replace the settings tab content -->
  <div v-else-if="activeTab === 'settings'" class="tab-panel">
    <SettingsPanel />
  </div>
</template>
```

- [ ] **Step 3: Commit**

```bash
git add src/components/layout/Sidebar.vue
git commit -m "feat: integrate SettingsPanel into sidebar"
```

**Verification:** Settings tab shows full settings panel

---

### Task 6: Delete HyprlandService

**Files:**
- Delete: `src-tauri/src/services/hyprland.rs`
- Modify: `src-tauri/src/services/mod.rs`

**Steps:**

- [ ] **Step 1: Remove hyprland module declaration**

```rust
// In src-tauri/src/services/mod.rs
// Remove this line:
// pub mod hyprland;
```

- [ ] **Step 2: Delete hyprland.rs file**

```bash
rm src-tauri/src/services/hyprland.rs
```

- [ ] **Step 3: Commit**

```bash
git add -A src-tauri/src/services/
git commit -m "refactor: remove unused HyprlandService"
```

**Verification:** `cargo check` passes without warnings

---

### Task 7: Remove optimize_content Method

**Files:**
- Modify: `src-tauri/src/services/performance.rs`

**Steps:**

- [ ] **Step 1: Remove optimize_content method**

```rust
// In src-tauri/src/services/performance.rs
// Remove the entire optimize_content method (lines 50-57)
```

- [ ] **Step 2: Remove #[allow(dead_code)] from PerformanceService**

```rust
// Remove this line:
// #[allow(dead_code)]
pub struct PerformanceService;
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/services/performance.rs
git commit -m "refactor: remove unused optimize_content method"
```

**Verification:** `cargo check` passes without warnings

---

### Task 8: Update Settings Store

**Files:**
- Modify: `src/stores/settings.ts`

**Steps:**

- [ ] **Step 1: Add default settings initialization**

```typescript
const DEFAULT_SETTINGS: Record<string, string> = {
  editor_font: "JetBrains Mono",
  editor_font_size: "14",
  editor_line_height: "1.6",
  auto_save_interval: "30",
  default_model_id: "",
  ai_temperature: "0.7",
  ai_max_tokens: "2000",
  ai_system_prompt: "你是一个专业的中文小说写作助手。",
  default_export_format: "txt",
  export_path: "",
  export_include_metadata: "true",
};

// Add method
const initializeDefaults = async () => {
  for (const [key, value] of Object.entries(DEFAULT_SETTINGS)) {
    if (!settings.value.has(key)) {
      await updateSetting(key, value);
    }
  }
};
```

- [ ] **Step 2: Export initializeDefaults**

```typescript
return {
  settings,
  loading,
  fetchSettings,
  updateSetting,
  getSetting,
  initializeDefaults,
};
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/settings.ts
git commit -m "feat: add default settings initialization"
```

**Verification:** Settings store initializes with defaults

---

### Task 9: Integration Test

**Files:**
- None (manual testing)

**Steps:**

- [ ] **Step 1: Run full test suite**

```bash
# Rust tests
cd src-tauri && cargo test

# Frontend tests
cd .. && bun run test:run
```

- [ ] **Step 2: Verify build**

```bash
bun run build
```

- [ ] **Step 3: Manual testing**

1. Open settings tab - verify all fields display
2. Change settings and save - verify persistence
3. Test cloud sync with WebDAV
4. Test cloud sync with OSS (if credentials available)
5. Verify no dead code warnings

- [ ] **Step 4: Commit final changes**

```bash
git add -A
git commit -m "chore: complete settings, cloud storage, and cleanup implementation"
```

**Verification:** All tests pass, no warnings, features work correctly

---

## Execution Summary

| Task | Description | Est. Time |
|------|-------------|-----------|
| 1 | Rust Dependencies Setup | 5 min |
| 2 | OSS Upload/Download | 15 min |
| 3 | S3 Upload/Download | 20 min |
| 4 | Settings Panel Component | 20 min |
| 5 | Update Sidebar | 5 min |
| 6 | Delete HyprlandService | 5 min |
| 7 | Remove optimize_content | 5 min |
| 8 | Update Settings Store | 10 min |
| 9 | Integration Test | 15 min |
| **Total** | | **~100 min** |

## Success Criteria

- [ ] Settings page displays all configuration fields
- [ ] Settings can be saved and loaded
- [ ] OSS upload/download works
- [ ] S3 upload/download works
- [ ] Dead code removed (HyprlandService, optimize_content)
- [ ] No compilation warnings
- [ ] All tests pass
- [ ] Build succeeds
