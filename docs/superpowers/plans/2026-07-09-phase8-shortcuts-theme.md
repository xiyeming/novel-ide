# Phase 8: Keyboard Shortcuts + Theme System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a cross-platform keyboard shortcut system with special Hyprland/Wayland support via Lua config generation, plus a theme system (light/dark/custom).

**Architecture:** In-app shortcuts use Vue keyboard events. Global shortcuts use Tauri's global-shortcut plugin on Windows/macOS. On Linux/Hyprland, generate Lua config snippets for users to add to their Hyprland config, with IPC fallback. Theme system uses CSS variables with light/dark/custom themes.

**Tech Stack:** Rust 1.90+ (tauri-plugin-global-shortcut), Vue 3.5.39, TypeScript 5.7+, CSS Variables

## Global Constraints
- Rust 1.90+, Tauri 2.11
- Vue 3.5.39, TypeScript 5.7+, Bun 1.2+
- All UI text in Chinese
- All Rust error messages in Chinese
- Hyprland: Lua config generation for global shortcuts
- Wayland: XDG GlobalShortcuts Portal support (future)
- Platforms: Windows, macOS, Linux (X11 + Wayland/Hyprland)

## File Structure

```
src-tauri/
  src/
    commands/
      shortcuts.rs     — Shortcut management commands
      theme.rs         — Theme management commands
    services/
      hyprland.rs      — Hyprland Lua config generator
src/
  composables/
    useKeyboardShortcuts.ts — In-app keyboard shortcut handler
  stores/
    shortcuts.ts       — Shortcut state + config
    theme.ts           — Theme state
  components/
    settings/
      ShortcutSettings.vue — Shortcut configuration UI
      ThemeSettings.vue    — Theme selection UI
    shortcuts/
      ShortcutHint.vue     — Shortcut hint display
src-tauri/
  capabilities/
    desktop.json       — Desktop capabilities for global shortcuts
```

## Tasks

### Task 1: Shortcut Config Migration + Model

**Files:**
- Create: `src-tauri/migrations/009_shortcuts_theme.sql`
- Create: `src-tauri/src/models/shortcut.rs`

**Steps:**

- [ ] **Step 1: Create migration**

```sql
-- 009_shortcuts_theme.sql
CREATE TABLE IF NOT EXISTS shortcuts (
    id TEXT PRIMARY KEY,
    action TEXT NOT NULL UNIQUE,
    key_binding TEXT NOT NULL,
    platform TEXT NOT NULL DEFAULT 'all', -- all/windows/macos/linux
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS themes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL, -- light/dark/custom
    config TEXT NOT NULL DEFAULT '{}', -- JSON CSS variables
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default shortcuts
INSERT OR IGNORE INTO shortcuts (id, action, key_binding, platform) VALUES
('default-save', '保存', 'CmdOrCtrl+S', 'all'),
('default-new-chapter', '新建章节', 'CmdOrCtrl+N', 'all'),
('default-search', '搜索', 'CmdOrCtrl+F', 'all'),
('default-export', '导出', 'CmdOrCtrl+E', 'all'),
('default-ai-chat', 'AI 聊天', 'CmdOrCtrl+Shift+A', 'all'),
('default-proofread', '校对', 'CmdOrCtrl+Shift+P', 'all'),
('default-toggle-sidebar', '切换侧边栏', 'CmdOrCtrl+B', 'all'),
('default-fullscreen', '全屏', 'F11', 'all');

-- Default themes
INSERT OR IGNORE INTO themes (id, name, type, config, is_active) VALUES
('dark', '深色', 'dark', '{}', 1),
('light', '亮色', 'light', '{}', 0);
```

- [ ] **Step 2: Register in db/mod.rs**

- [ ] **Step 3: Create shortcut.rs model**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: String,
    pub action: String,
    pub key_binding: String,
    pub platform: String,
    pub is_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub theme_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/009_shortcuts_theme.sql src-tauri/src/models/shortcut.rs src-tauri/src/db/mod.rs
git commit -m "feat: add shortcuts + theme migration"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 2: Shortcut Commands

**Files:**
- Create: `src-tauri/src/commands/shortcuts.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Steps:**

- [ ] **Step 1: Create shortcuts.rs**

```rust
use crate::error::AppError;
use crate::models::shortcut::Shortcut;
use crate::state::AppState;
use sqlx::Row;
use tauri::State;

#[tauri::command]
pub async fn list_shortcuts(
    state: State<'_, AppState>,
) -> Result<Vec<Shortcut>, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM shortcuts ORDER BY action")
        .fetch_all(&db)
        .await?;

    rows.into_iter().map(|row| {
        Ok(Shortcut {
            id: row.get("id"),
            action: row.get("action"),
            key_binding: row.get("key_binding"),
            platform: row.get("platform"),
            is_enabled: row.get::<i64, _>("is_enabled") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn update_shortcut(
    state: State<'_, AppState>,
    id: String,
    key_binding: String,
) -> Result<Shortcut, AppError> {
    let db = state.db().await?;
    sqlx::query("UPDATE shortcuts SET key_binding = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&key_binding)
        .bind(&id)
        .execute(&db)
        .await?;

    let row = sqlx::query("SELECT * FROM shortcuts WHERE id = ?")
        .bind(&id)
        .fetch_one(&db)
        .await?;

    Ok(Shortcut {
        id: row.get("id"),
        action: row.get("action"),
        key_binding: row.get("key_binding"),
        platform: row.get("platform"),
        is_enabled: row.get::<i64, _>("is_enabled") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn toggle_shortcut(
    state: State<'_, AppState>,
    id: String,
    enabled: bool,
) -> Result<(), AppError> {
    let db = state.db().await?;
    sqlx::query("UPDATE shortcuts SET is_enabled = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(enabled as i64)
        .bind(&id)
        .execute(&db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn generate_hyprland_config(
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM shortcuts WHERE is_enabled = 1 AND (platform = 'all' OR platform = 'linux')")
        .fetch_all(&db)
        .await?;

    let mut config = String::from("# Novel IDE 快捷键配置\n# 将以下内容添加到你的 hyprland.conf\n\n");

    for row in rows {
        let action: String = row.get("action");
        let key_binding: String = row.get("key_binding");
        
        // Convert Tauri shortcut format to Hyprland format
        let hyprland_binding = convert_to_hyprland_format(&key_binding);
        
        config.push_str(&format!(
            "# {}\nbind = {}, exec, novel-ide-cli shortcut {}\n\n",
            action, hyprland_binding, action
        ));
    }

    Ok(config)
}

fn convert_to_hyprland_format(tauri_shortcut: &str) -> String {
    let mut result = tauri_shortcut
        .replace("CmdOrCtrl", "SUPER")
        .replace("CommandOrControl", "SUPER")
        .replace("Ctrl", "CTRL")
        .replace("Alt", "ALT")
        .replace("Shift", "SHIFT")
        .replace("+", " + ");
    
    // Handle special keys
    result = result
        .replace("F11", "F11")
        .replace("F12", "F12")
        .replace("Space", "SPACE")
        .replace("Enter", "RETURN")
        .replace("Backspace", "BACKSPACE")
        .replace("Delete", "DELETE")
        .replace("Tab", "TAB");
    
    result
}
```

- [ ] **Step 2: Register in mod.rs and lib.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/shortcuts.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add shortcut commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 3: Theme Commands

**Files:**
- Create: `src-tauri/src/commands/theme.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Steps:**

- [ ] **Step 1: Create theme.rs**

```rust
use crate::error::AppError;
use crate::models::shortcut::Theme;
use crate::state::AppState;
use sqlx::Row;
use tauri::State;

#[tauri::command]
pub async fn list_themes(
    state: State<'_, AppState>,
) -> Result<Vec<Theme>, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM themes ORDER BY name")
        .fetch_all(&db)
        .await?;

    rows.into_iter().map(|row| {
        let config_json: String = row.get("config");
        let config: serde_json::Value = serde_json::from_str(&config_json).unwrap_or_default();
        Ok(Theme {
            id: row.get("id"),
            name: row.get("name"),
            theme_type: row.get("type"),
            config,
            is_active: row.get::<i64, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn set_active_theme(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    
    // Deactivate all themes
    sqlx::query("UPDATE themes SET is_active = 0")
        .execute(&db)
        .await?;
    
    // Activate selected theme
    sqlx::query("UPDATE themes SET is_active = 1, updated_at = datetime('now') WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await?;
    
    Ok(())
}

#[tauri::command]
pub async fn create_custom_theme(
    state: State<'_, AppState>,
    name: String,
    config: serde_json::Value,
) -> Result<Theme, AppError> {
    let db = state.db().await?;
    let id = uuid::Uuid::new_v4().to_string();
    let config_json = serde_json::to_string(&config)?;
    
    sqlx::query("INSERT INTO themes (id, name, type, config) VALUES (?, ?, 'custom', ?)")
        .bind(&id)
        .bind(&name)
        .bind(&config_json)
        .execute(&db)
        .await?;

    Ok(Theme {
        id,
        name,
        theme_type: "custom".into(),
        config,
        is_active: false,
        created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[tauri::command]
pub async fn delete_theme(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    
    // Prevent deleting active theme
    let row = sqlx::query("SELECT is_active FROM themes WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db)
        .await?;
    
    if let Some(r) = row {
        if r.get::<i64, _>("is_active") != 0 {
            return Err(AppError::Internal("不能删除当前使用的主题".into()));
        }
    }
    
    sqlx::query("DELETE FROM themes WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await?;
    
    Ok(())
}
```

- [ ] **Step 2: Register in mod.rs and lib.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/theme.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add theme commands"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 4: Hyprland Service

**Files:**
- Create: `src-tauri/src/services/hyprland.rs`
- Modify: `src-tauri/src/services/mod.rs`

**Steps:**

- [ ] **Step 1: Create hyprland.rs**

```rust
use crate::error::AppError;
use std::fs;
use std::path::PathBuf;

pub struct HyprlandService;

impl HyprlandService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_config_path(&self) -> Result<PathBuf, AppError> {
        let home = std::env::var("HOME")
            .map_err(|_| AppError::Internal("无法获取 HOME 目录".into()))?;
        
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("hypr")
            .join("hyprland.conf");
        
        Ok(config_path)
    }

    pub fn backup_config(&self) -> Result<PathBuf, AppError> {
        let config_path = self.get_config_path()?;
        let backup_path = config_path.with_extension("conf.bak");
        
        if config_path.exists() {
            fs::copy(&config_path, &backup_path)
                .map_err(|e| AppError::Io(format!("备份配置失败: {}", e)))?;
        }
        
        Ok(backup_path)
    }

    pub fn append_shortcuts(&self, shortcuts_config: &str) -> Result<(), AppError> {
        let config_path = self.get_config_path()?;
        
        // Check if config exists
        if !config_path.exists() {
            return Err(AppError::Internal("Hyprland 配置文件不存在".into()));
        }
        
        // Read existing config
        let existing_config = fs::read_to_string(&config_path)
            .map_err(|e| AppError::Io(format!("读取配置失败: {}", e)))?;
        
        // Check if shortcuts already added
        if existing_config.contains("# Novel IDE 快捷键配置") {
            return Err(AppError::Internal("快捷键配置已存在，请手动更新".into()));
        }
        
        // Append new config
        let new_config = format!("{}\n\n{}", existing_config, shortcuts_config);
        
        fs::write(&config_path, new_config)
            .map_err(|e| AppError::Io(format!("写入配置失败: {}", e)))?;
        
        Ok(())
    }

    pub fn generate_cli_script(&self) -> Result<String, AppError> {
        let script = r#"#!/bin/bash
# Novel IDE CLI - Hyprland 快捷键处理

ACTION="$1"

case "$ACTION" in
    "保存")
        notify-send "Novel IDE" "保存功能需要在应用内使用"
        ;;
    "新建章节")
        notify-send "Novel IDE" "新建章节功能需要在应用内使用"
        ;;
    "搜索")
        notify-send "Novel IDE" "搜索功能需要在应用内使用"
        ;;
    "导出")
        notify-send "Novel IDE" "导出功能需要在应用内使用"
        ;;
    "AI 聊天")
        notify-send "Novel IDE" "AI 聊天功能需要在应用内使用"
        ;;
    "校对")
        notify-send "Novel IDE" "校对功能需要在应用内使用"
        ;;
    "切换侧边栏")
        notify-send "Novel IDE" "切换侧边栏功能需要在应用内使用"
        ;;
    "全屏")
        notify-send "Novel IDE" "全屏功能需要在应用内使用"
        ;;
    *)
        notify-send "Novel IDE" "未知操作: $ACTION"
        ;;
esac
"#;
        Ok(script.to_string())
    }

    pub fn install_cli_script(&self) -> Result<PathBuf, AppError> {
        let script = self.generate_cli_script()?;
        let script_path = PathBuf::from("/usr/local/bin/novel-ide-cli");
        
        fs::write(&script_path, script)
            .map_err(|e| AppError::Io(format!("安装 CLI 脚本失败: {}", e)))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)
                .map_err(|e| AppError::Io(e.to_string()))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)
                .map_err(|e| AppError::Io(e.to_string()))?;
        }
        
        Ok(script_path)
    }
}
```

- [ ] **Step 2: Register in mod.rs**

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/services/hyprland.rs src-tauri/src/services/mod.rs
git commit -m "feat: add Hyprland service"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 5: Shortcut Store + Composable

**Files:**
- Create: `src/stores/shortcuts.ts`
- Create: `src/composables/useKeyboardShortcuts.ts`

**Steps:**

- [ ] **Step 1: Create shortcuts.ts**

```typescript
import { ref } from 'vue'
import { useTauriIPC } from '../composables/useTauriIPC'

export interface Shortcut {
  id: string
  action: string
  keyBinding: string
  platform: string
  isEnabled: boolean
}

export function useShortcutStore() {
  const shortcuts = ref<Shortcut[]>([])
  const loading = ref(false)
  const { call } = useTauriIPC()

  async function fetchShortcuts() {
    loading.value = true
    try {
      shortcuts.value = await call<Shortcut[]>('list_shortcuts')
    } finally {
      loading.value = false
    }
  }

  async function updateShortcut(id: string, keyBinding: string) {
    const result = await call<Shortcut>('update_shortcut', { id, keyBinding })
    const idx = shortcuts.value.findIndex(s => s.id === id)
    if (idx >= 0) shortcuts.value[idx] = result
    return result
  }

  async function toggleShortcut(id: string, enabled: boolean) {
    await call('toggle_shortcut', { id, enabled })
    const idx = shortcuts.value.findIndex(s => s.id === id)
    if (idx >= 0) shortcuts.value[idx].isEnabled = enabled
  }

  async function generateHyprlandConfig() {
    return await call<string>('generate_hyprland_config')
  }

  function getShortcutForAction(action: string) {
    const shortcut = shortcuts.value.find(s => s.action === action && s.isEnabled)
    return shortcut?.keyBinding || null
  }

  return {
    shortcuts,
    loading,
    fetchShortcuts,
    updateShortcut,
    toggleShortcut,
    generateHyprlandConfig,
    getShortcutForAction,
  }
}
```

- [ ] **Step 2: Create useKeyboardShortcuts.ts**

```typescript
import { onMounted, onUnmounted } from 'vue'
import { useShortcutStore } from '../stores/shortcuts'

type ShortcutHandler = () => void

export function useKeyboardShortcuts(handlers: Record<string, ShortcutHandler>) {
  const store = useShortcutStore()
  
  function parseKeyBinding(binding: string): { key: string; ctrl: boolean; alt: boolean; shift: boolean; meta: boolean } {
    const parts = binding.split('+')
    const key = parts[parts.length - 1]
    const ctrl = parts.includes('Ctrl') || parts.includes('CmdOrCtrl')
    const alt = parts.includes('Alt')
    const shift = parts.includes('Shift')
    const meta = parts.includes('Cmd') || parts.includes('CmdOrCtrl')
    
    return { key, ctrl, alt, shift, meta }
  }

  function handleKeyDown(event: KeyboardEvent) {
    for (const [action, handler] of Object.entries(handlers)) {
      const binding = store.getShortcutForAction(action)
      if (!binding) continue
      
      const parsed = parseKeyBinding(binding)
      
      const keyMatch = event.key.toLowerCase() === parsed.key.toLowerCase() ||
                       event.code.toLowerCase() === `key${parsed.key.toLowerCase()}`
      const ctrlMatch = event.ctrlKey === parsed.ctrl
      const altMatch = event.altKey === parsed.alt
      const shiftMatch = event.shiftKey === parsed.shift
      const metaMatch = event.metaKey === parsed.meta
      
      if (keyMatch && ctrlMatch && altMatch && shiftMatch && metaMatch) {
        event.preventDefault()
        handler()
        return
      }
    }
  }

  onMounted(() => {
    document.addEventListener('keydown', handleKeyDown)
  })

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeyDown)
  })
}
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/shortcuts.ts src/composables/useKeyboardShortcuts.ts
git commit -m "feat: add shortcut store and composable"
```

**Verification:** `bun run build`

---

### Task 6: Theme Store + CSS Variables

**Files:**
- Create: `src/stores/theme.ts`
- Modify: `src/assets/main.css`

**Steps:**

- [ ] **Step 1: Create theme.ts**

```typescript
import { ref, watchEffect } from 'vue'
import { useTauriIPC } from '../composables/useTauriIPC'

export interface Theme {
  id: string
  name: string
  type: string
  config: Record<string, string>
  isActive: boolean
}

const defaultThemes: Record<string, Record<string, string>> = {
  dark: {
    '--bg-primary': '#1a1a2e',
    '--bg-secondary': '#16213e',
    '--bg-tertiary': '#0f3460',
    '--text-primary': '#e6e6e6',
    '--text-secondary': '#a0a0a0',
    '--accent': '#4a9eff',
    '--accent-hover': '#3a8eef',
    '--border': '#333',
    '--success': '#4caf50',
    '--warning': '#ff9800',
    '--error': '#f44336',
  },
  light: {
    '--bg-primary': '#ffffff',
    '--bg-secondary': '#f5f5f5',
    '--bg-tertiary': '#e0e0e0',
    '--text-primary': '#1a1a1a',
    '--text-secondary': '#666666',
    '--accent': '#1976d2',
    '--accent-hover': '#1565c0',
    '--border': '#ddd',
    '--success': '#4caf50',
    '--warning': '#ff9800',
    '--error': '#f44336',
  },
}

export function useThemeStore() {
  const themes = ref<Theme[]>([])
  const activeTheme = ref<Theme | null>(null)
  const loading = ref(false)
  const { call } = useTauriIPC()

  async function fetchThemes() {
    loading.value = true
    try {
      themes.value = await call<Theme[]>('list_themes')
      activeTheme.value = themes.value.find(t => t.isActive) || null
      applyTheme(activeTheme.value)
    } finally {
      loading.value = false
    }
  }

  async function setActiveTheme(id: string) {
    await call('set_active_theme', { id })
    const theme = themes.value.find(t => t.id === id)
    if (theme) {
      activeTheme.value = theme
      applyTheme(theme)
    }
  }

  async function createCustomTheme(name: string, config: Record<string, string>) {
    const result = await call<Theme>('create_custom_theme', { name, config })
    themes.value.push(result)
    return result
  }

  async function deleteTheme(id: string) {
    await call('delete_theme', { id })
    themes.value = themes.value.filter(t => t.id !== id)
  }

  function applyTheme(theme: Theme | null) {
    const root = document.documentElement
    
    // Reset to default dark theme variables
    const baseConfig = defaultThemes.dark
    
    // Apply theme config (custom themes override defaults)
    const config = theme?.config || {}
    const mergedConfig = { ...baseConfig, ...config }
    
    for (const [key, value] of Object.entries(mergedConfig)) {
      root.style.setProperty(key, value)
    }
  }

  function getThemePreview(theme: Theme) {
    return theme.config || defaultThemes[theme.type] || defaultThemes.dark
  }

  return {
    themes,
    activeTheme,
    loading,
    fetchThemes,
    setActiveTheme,
    createCustomTheme,
    deleteTheme,
    applyTheme,
    getThemePreview,
  }
}
```

- [ ] **Step 2: Update main.css with CSS variables**

```css
:root {
  /* Default dark theme */
  --bg-primary: #1a1a2e;
  --bg-secondary: #16213e;
  --bg-tertiary: #0f3460;
  --text-primary: #e6e6e6;
  --text-secondary: #a0a0a0;
  --accent: #4a9eff;
  --accent-hover: #3a8eef;
  --border: #333;
  --success: #4caf50;
  --warning: #ff9800;
  --error: #f44336;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

/* Apply theme variables */
.theme-dark {
  --bg-primary: #1a1a2e;
  --bg-secondary: #16213e;
  --bg-tertiary: #0f3460;
  --text-primary: #e6e6e6;
  --text-secondary: #a0a0a0;
  --accent: #4a9eff;
}

.theme-light {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --bg-tertiary: #e0e0e0;
  --text-primary: #1a1a1a;
  --text-secondary: #666666;
  --accent: #1976d2;
}
```

- [ ] **Step 3: Commit**

```bash
git add src/stores/theme.ts src/assets/main.css
git commit -m "feat: add theme store and CSS variables"
```

**Verification:** `bun run build`

---

### Task 7: Shortcut Settings UI

**Files:**
- Create: `src/components/settings/ShortcutSettings.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Create ShortcutSettings.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useShortcutStore } from '../../stores/shortcuts'

const store = useShortcutStore()
const editingId = ref<string | null>(null)
const newKeyBinding = ref('')
const hyprlandConfig = ref('')
const showHyprlandDialog = ref(false)

onMounted(() => {
  store.fetchShortcuts()
})

function startEdit(id: string, currentBinding: string) {
  editingId.value = id
  newKeyBinding.value = currentBinding
}

function captureKey(event: KeyboardEvent) {
  event.preventDefault()
  const parts: string[] = []
  if (event.ctrlKey) parts.push('Ctrl')
  if (event.altKey) parts.push('Alt')
  if (event.shiftKey) parts.push('Shift')
  if (event.metaKey) parts.push('Cmd')
  
  const key = event.key.length === 1 ? event.key.toUpperCase() : event.key
  if (!['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
    parts.push(key)
    newKeyBinding.value = parts.join('+')
  }
}

async function saveEdit() {
  if (!editingId.value || !newKeyBinding.value) return
  await store.updateShortcut(editingId.value, newKeyBinding.value)
  editingId.value = null
  newKeyBinding.value = ''
}

async function generateHyprland() {
  hyprlandConfig.value = await store.generateHyprlandConfig()
  showHyprlandDialog.value = true
}

function copyToClipboard() {
  navigator.clipboard.writeText(hyprlandConfig.value)
}
</script>

<template>
  <div class="shortcut-settings">
    <div class="settings-header">
      <h3>⌨️ 快捷键设置</h3>
      <button class="btn-sm" @click="generateHyprland">导出 Hyprland 配置</button>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>
    
    <div v-else class="shortcut-list">
      <div v-for="shortcut in store.shortcuts" :key="shortcut.id" class="shortcut-item">
        <div class="shortcut-info">
          <div class="shortcut-action">{{ shortcut.action }}</div>
          <div class="shortcut-binding" v-if="editingId !== shortcut.id">
            {{ shortcut.keyBinding }}
          </div>
          <input 
            v-else
            v-model="newKeyBinding"
            @keydown="captureKey"
            class="key-input"
            readonly
            placeholder="按下快捷键..."
          />
        </div>
        <div class="shortcut-actions">
          <button 
            v-if="editingId === shortcut.id"
            class="btn-sm primary"
            @click="saveEdit"
          >
            保存
          </button>
          <button 
            v-else
            class="btn-sm"
            @click="startEdit(shortcut.id, shortcut.keyBinding)"
          >
            编辑
          </button>
          <label class="toggle">
            <input 
              type="checkbox" 
              :checked="shortcut.isEnabled"
              @change="store.toggleShortcut(shortcut.id, ($event.target as HTMLInputElement).checked)"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- Hyprland 配置对话框 -->
    <div v-if="showHyprlandDialog" class="dialog-overlay" @click="showHyprlandDialog = false">
      <div class="dialog" @click.stop>
        <h3>Hyprland 快捷键配置</h3>
        <p class="dialog-hint">将以下内容添加到 ~/.config/hypr/hyprland.conf</p>
        <textarea :value="hyprlandConfig" readonly class="config-textarea" rows="15" />
        <div class="dialog-actions">
          <button class="btn-sm" @click="copyToClipboard">复制到剪贴板</button>
          <button class="btn-sm" @click="showHyprlandDialog = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.shortcut-settings {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.settings-header h3 {
  margin: 0;
  font-size: 16px;
}

.btn-sm {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.btn-sm:hover {
  background: var(--bg-tertiary);
}

.btn-sm.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border);
}

.shortcut-info {
  flex: 1;
}

.shortcut-action {
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.shortcut-binding {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: monospace;
}

.key-input {
  width: 200px;
  padding: 6px 8px;
  border: 2px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: monospace;
  font-size: 12px;
}

.shortcut-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--bg-tertiary);
  transition: 0.3s;
  border-radius: 22px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: var(--text-secondary);
  transition: 0.3s;
  border-radius: 50%;
}

.toggle input:checked + .toggle-slider {
  background-color: var(--accent);
}

.toggle input:checked + .toggle-slider:before {
  transform: translateX(18px);
  background-color: white;
}

.loading {
  text-align: center;
  color: var(--text-secondary);
  padding: 20px;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 24px;
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.dialog h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
}

.dialog-hint {
  color: var(--text-secondary);
  font-size: 13px;
  margin-bottom: 16px;
}

.config-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: monospace;
  font-size: 12px;
  resize: vertical;
  margin-bottom: 16px;
  box-sizing: border-box;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>
```

- [ ] **Step 2: Add shortcut settings to Sidebar.vue**

Add a new tab for shortcuts (⌨️ 快捷键) and import ShortcutSettings.

- [ ] **Step 3: Commit**

```bash
git add src/components/settings/ShortcutSettings.vue src/components/layout/Sidebar.vue
git commit -m "feat: add shortcut settings UI"
```

**Verification:** `bun run build`

---

### Task 8: Theme Settings UI

**Files:**
- Create: `src/components/settings/ThemeSettings.vue`
- Modify: `src/components/layout/Sidebar.vue`

**Steps:**

- [ ] **Step 1: Create ThemeSettings.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useThemeStore } from '../../stores/theme'

const store = useThemeStore()
const showCreateDialog = ref(false)
const newThemeName = ref('')
const newThemeConfig = ref<Record<string, string>>({
  '--bg-primary': '#1a1a2e',
  '--bg-secondary': '#16213e',
  '--text-primary': '#e6e6e6',
  '--accent': '#4a9eff',
})

onMounted(() => {
  store.fetchThemes()
})

async function createTheme() {
  if (!newThemeName.value) return
  await store.createCustomTheme(newThemeName.value, newThemeConfig.value)
  showCreateDialog.value = false
  newThemeName.value = ''
}

function getThemePreviewColors(theme: any) {
  const config = store.getThemePreview(theme)
  return {
    bg: config['--bg-primary'] || '#1a1a2e',
    text: config['--text-primary'] || '#e6e6e6',
    accent: config['--accent'] || '#4a9eff',
  }
}
</script>

<template>
  <div class="theme-settings">
    <div class="settings-header">
      <h3>🎨 主题设置</h3>
      <button class="btn-sm" @click="showCreateDialog = true">+ 自定义主题</button>
    </div>

    <div v-if="store.loading" class="loading">加载中...</div>
    
    <div v-else class="theme-grid">
      <div 
        v-for="theme in store.themes" 
        :key="theme.id" 
        class="theme-card"
        :class="{ active: theme.isActive }"
        @click="store.setActiveTheme(theme.id)"
      >
        <div class="theme-preview" :style="{ 
          background: getThemePreviewColors(theme).bg,
          color: getThemePreviewColors(theme).text
        }">
          <div class="preview-accent" :style="{ background: getThemePreviewColors(theme).accent }"></div>
          <div class="preview-text">Aa</div>
        </div>
        <div class="theme-name">{{ theme.name }}</div>
        <div class="theme-type">{{ theme.type }}</div>
        <button 
          v-if="theme.type === 'custom'"
          class="btn-delete"
          @click.stop="store.deleteTheme(theme.id)"
        >
          删除
        </button>
      </div>
    </div>

    <!-- 创建自定义主题对话框 -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click="showCreateDialog = false">
      <div class="dialog" @click.stop>
        <h3>创建自定义主题</h3>
        <input v-model="newThemeName" placeholder="主题名称" class="input" />
        
        <div class="color-inputs">
          <div class="color-input">
            <label>背景色</label>
            <input type="color" v-model="newThemeConfig['--bg-primary']" />
          </div>
          <div class="color-input">
            <label>文字色</label>
            <input type="color" v-model="newThemeConfig['--text-primary']" />
          </div>
          <div class="color-input">
            <label>强调色</label>
            <input type="color" v-model="newThemeConfig['--accent']" />
          </div>
        </div>

        <div class="dialog-actions">
          <button class="btn-sm" @click="showCreateDialog = false">取消</button>
          <button class="btn-sm primary" @click="createTheme">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.theme-settings {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.settings-header h3 {
  margin: 0;
  font-size: 16px;
}

.btn-sm {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.btn-sm:hover {
  background: var(--bg-tertiary);
}

.btn-sm.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
}

.theme-card {
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.theme-card:hover {
  border-color: var(--accent);
}

.theme-card.active {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent);
}

.theme-preview {
  width: 100%;
  height: 60px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
  position: relative;
  overflow: hidden;
}

.preview-accent {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
}

.preview-text {
  font-size: 18px;
  font-weight: bold;
}

.theme-name {
  font-size: 13px;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.theme-type {
  font-size: 11px;
  color: var(--text-secondary);
}

.btn-delete {
  margin-top: 8px;
  padding: 4px 8px;
  border: none;
  border-radius: 4px;
  background: var(--error);
  color: white;
  cursor: pointer;
  font-size: 11px;
}

.loading {
  text-align: center;
  color: var(--text-secondary);
  padding: 20px;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 24px;
  max-width: 400px;
  width: 90%;
}

.dialog h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
}

.input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  margin-bottom: 16px;
  box-sizing: border-box;
}

.color-inputs {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.color-input {
  flex: 1;
  text-align: center;
}

.color-input label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.color-input input[type="color"] {
  width: 100%;
  height: 40px;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>
```

- [ ] **Step 2: Add theme settings to Sidebar.vue**

Add a new tab for themes (🎨 主题) and import ThemeSettings.

- [ ] **Step 3: Commit**

```bash
git add src/components/settings/ThemeSettings.vue src/components/layout/Sidebar.vue
git commit -m "feat: add theme settings UI"
```

**Verification:** `bun run build`

---

### Task 9: Global Shortcut Plugin Integration

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`
- Create: `src-tauri/capabilities/desktop.json`

**Steps:**

- [ ] **Step 1: Add global-shortcut plugin to Cargo.toml**

```toml
tauri-plugin-global-shortcut = "2"
```

- [ ] **Step 2: Update lib.rs to register global shortcuts**

```rust
// In lib.rs setup
.plugin(tauri_plugin_global_shortcut::Builder::new().build())
```

- [ ] **Step 3: Create desktop.json capability**

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "desktop-capability",
  "windows": ["main"],
  "platforms": ["linux", "macOS", "windows"],
  "permissions": [
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-is-registered"
  ]
}
```

- [ ] **Step 4: Add platform-specific shortcut registration**

In lib.rs setup, register shortcuts based on platform:
- Windows/macOS: Use Tauri's global-shortcut plugin
- Linux/Hyprland: Generate Lua config and show instructions

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs src-tauri/capabilities/
git commit -m "feat: add global shortcut plugin integration"
```

**Verification:** `cargo check` from src-tauri/

---

### Task 10: Integration Verification

**Files:**
- None (verification only)

**Steps:**

- [ ] **Step 1: Verify all commands in lib.rs**

Count should now be ~55+.

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
Task 1: feat: add shortcuts + theme migration
Task 2: feat: add shortcut commands
Task 3: feat: add theme commands
Task 4: feat: add Hyprland service
Task 5: feat: add shortcut store and composable
Task 6: feat: add theme store and CSS variables
Task 7: feat: add shortcut settings UI
Task 8: feat: add theme settings UI
Task 9: feat: add global shortcut plugin integration
Task 10: (verification only)
```
