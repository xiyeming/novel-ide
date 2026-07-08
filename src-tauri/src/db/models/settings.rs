// src-tauri/src/db/models/settings.rs
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GlobalSetting {
    pub key: String,
    pub value: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectSetting {
    pub id: String,
    pub project_id: String,
    pub key: String,
    pub value: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl GlobalSetting {
    pub async fn get(
        db: &sqlx::SqlitePool,
        key: &str,
    ) -> Result<Option<String>, crate::error::AppError> {
        let row = sqlx::query_as::<_, Self>("SELECT * FROM global_settings WHERE key = ?")
            .bind(key)
            .fetch_optional(db)
            .await?;
        Ok(row.and_then(|r| r.value))
    }

    pub async fn set(
        db: &sqlx::SqlitePool,
        key: &str,
        value: &str,
    ) -> Result<(), crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            r#"INSERT INTO global_settings (key, value, updated_at) VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at"#,
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_all(db: &sqlx::SqlitePool) -> Result<Vec<Self>, crate::error::AppError> {
        let settings = sqlx::query_as::<_, Self>("SELECT * FROM global_settings")
            .fetch_all(db)
            .await?;
        Ok(settings)
    }
}
