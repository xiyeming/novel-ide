use std::collections::HashMap;

use crate::error::AppError;
use crate::state::AppState;

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> Result<HashMap<String, String>, AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;

    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM settings")
        .fetch_all(&db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows.into_iter().collect())
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO settings (id, key, value, updated_at) VALUES (?, ?, ?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at"
    )
    .bind(key.clone())
    .bind(&key)
    .bind(&value)
    .bind(&now)
    .execute(&db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}
