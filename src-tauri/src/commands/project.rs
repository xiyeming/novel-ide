use crate::db::models::{CreateProjectRequest, Project};
use crate::error::AppError;
use crate::state::AppState;

#[tauri::command]
pub async fn create_project(
    state: tauri::State<'_, AppState>,
    request: CreateProjectRequest,
) -> Result<Project, AppError> {
    let db = state.db().await?;
    Project::create(&db, &request).await
}

#[tauri::command]
pub async fn list_projects(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Project>, AppError> {
    let db = state.db().await?;
    Project::list_all(&db).await
}

#[tauri::command]
pub async fn open_project(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Project, AppError> {
    let db = state.db().await?;
    Project::find_by_id(&db, &id).await
}

#[tauri::command]
pub async fn delete_project(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    Project::delete(&db, &id).await
}
