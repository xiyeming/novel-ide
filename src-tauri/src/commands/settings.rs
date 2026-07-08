use std::collections::HashMap;

use crate::db::models::GlobalSetting;
use crate::error::AppError;
use crate::state::AppState;

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> Result<HashMap<String, String>, AppError> {
    let db = state.db().await?;
    let settings = GlobalSetting::get_all(&db).await?;

    Ok(settings
        .into_iter()
        .filter_map(|s| s.value.map(|v| (s.key, v)))
        .collect())
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    GlobalSetting::set(&db, &key, &value).await
}
