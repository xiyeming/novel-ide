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
            fs::copy(&config_path, &backup_path)?;
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
        let existing_config = fs::read_to_string(&config_path)?;
        
        // Check if shortcuts already added
        if existing_config.contains("# Novel IDE 快捷键配置") {
            return Err(AppError::Internal("快捷键配置已存在，请手动更新".into()));
        }
        
        // Append new config
        let new_config = format!("{}\n\n{}", existing_config, shortcuts_config);
        
        fs::write(&config_path, new_config)?;
        
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
        
        fs::write(&script_path, script)?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)?;
        }
        
        Ok(script_path)
    }
}
