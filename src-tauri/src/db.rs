use sqlx::sqlite::SqlitePool;
use tauri::{AppHandle, Manager};

use crate::error::AppError;

pub async fn init_database(app: &AppHandle) -> Result<SqlitePool, AppError> {
    let db_path = app_data_dir(app).join("novel-ide.db");

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::Io(e.to_string()))?;
    }

    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display()))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(pool)
}

fn app_data_dir(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
}
