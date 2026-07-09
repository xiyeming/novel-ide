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
