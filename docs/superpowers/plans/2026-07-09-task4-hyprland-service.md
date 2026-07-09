# Task 4: Hyprland Service Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a HyprlandService struct for managing Hyprland window manager configuration on Linux.

**Architecture:** The service provides methods to get Hyprland config path, backup config, append shortcut configurations, and generate/install CLI scripts for handling keyboard shortcuts in Hyprland.

**Tech Stack:** Rust 1.90+, Tauri 2.11, std::fs for file operations

## Global Constraints
- Rust 1.90+, Tauri 2.11
- All UI text in Chinese
- All Rust error messages in Chinese
- Platforms: Windows, macOS, Linux (X11 + Wayland/Hyprland)

---

### Task 4: Hyprland Service

**Files:**
- Create: `src-tauri/src/services/hyprland.rs`
- Modify: `src-tauri/src/services/mod.rs`

**Interfaces:**
- Consumes: `crate::error::AppError`
- Produces: `HyprlandService` struct with methods for Hyprland config management

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

```rust
pub mod hyprland;
```

- [ ] **Step 3: Verify with cargo check**

```bash
cd src-tauri && cargo check
```

Expected: Successful compilation without errors

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/services/hyprland.rs src-tauri/src/services/mod.rs
git commit -m "feat: add Hyprland service"
```

---

## Verification

After implementation, verify:
1. `cargo check` passes from `src-tauri/`
2. HyprlandService struct is properly exported
3. All methods are accessible
4. Error handling works correctly with AppError types
