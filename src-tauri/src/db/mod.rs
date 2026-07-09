// src-tauri/src/db/mod.rs
pub mod models;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tauri::{AppHandle, Manager};

pub async fn init_database(app: &AppHandle) -> Result<SqlitePool, crate::error::AppError> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("novel-ide.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run migrations — raw_sql executes all statements in the file
    sqlx::raw_sql(include_str!("../../migrations/001_initial.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/002_chapters.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/003_chapters_fts.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/004_versions_models.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/005_knowledge_base.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/006_knowledge_fts.sql"))
        .execute(&pool)
        .await?;

    sqlx::raw_sql(include_str!("../../migrations/007_workflow_agent.sql"))
        .execute(&pool)
        .await?;

    log::info!("数据库初始化完成: {}", db_path.display());

    Ok(pool)
}
