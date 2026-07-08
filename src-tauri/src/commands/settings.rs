use crate::db::models::settings::GlobalSetting;
use crate::error::AppResult;
use crate::state::AppState;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> AppResult<Vec<GlobalSetting>> {
    let db = state.db().await?;
    let settings = GlobalSetting::get_all(&db).await?;
    Ok(settings)
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    key: String,
    value: Value,
) -> AppResult<()> {
    let db = state.db().await?;
    let value_str = serde_json::to_string(&value)?;
    GlobalSetting::set(&db, &key, &value_str).await?;
    Ok(())
}
