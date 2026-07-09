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
