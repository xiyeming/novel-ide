use crate::db::models::version::ChapterVersion;
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn save_version(
    state: State<'_, AppState>,
    chapter_id: String,
    content: String,
) -> AppResult<ChapterVersion> {
    let db = state.db().await?;
    let version = ChapterVersion::save(&db, &chapter_id, &content).await?;
    Ok(version)
}

#[tauri::command]
pub async fn list_versions(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<Vec<ChapterVersion>> {
    let db = state.db().await?;
    let versions = ChapterVersion::list_by_chapter(&db, &chapter_id).await?;
    Ok(versions)
}

#[tauri::command]
pub async fn restore_version(
    state: State<'_, AppState>,
    version_id: String,
) -> AppResult<String> {
    let db = state.db().await?;
    let version = ChapterVersion::find_by_id(&db, &version_id).await?;
    sqlx::query("UPDATE chapters SET content = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&version.content)
        .bind(&version.chapter_id)
        .execute(&db)
        .await?;
    Ok(version.content)
}
