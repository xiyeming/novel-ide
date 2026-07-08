use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub async fn create_project(
    state: tauri::State<'_, AppState>,
    request: CreateProjectRequest,
) -> Result<Project, AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO projects (id, name, path, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&request.name)
        .bind(&request.path)
        .bind(&now)
        .bind(&now)
        .execute(&db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Project {
        id,
        name: request.name,
        path: request.path,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_projects(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Project>, AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;

    let projects = sqlx::query_as::<_, Project>("SELECT id, name, path, created_at, updated_at FROM projects ORDER BY updated_at DESC")
        .fetch_all(&db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(projects)
}

#[tauri::command]
pub async fn open_project(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Project, AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;

    let project = sqlx::query_as::<_, Project>("SELECT id, name, path, created_at, updated_at FROM projects WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;

    Ok(project)
}

#[tauri::command]
pub async fn delete_project(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.get_db().await.ok_or_else(|| AppError::Database("Database not initialized".into()))?;

    sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}
