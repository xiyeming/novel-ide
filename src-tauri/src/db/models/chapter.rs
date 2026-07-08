// src-tauri/src/db/models/chapter.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Chapter {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub sort_order: i32,
    pub word_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChapterRequest {
    pub project_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChapterRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub sort_order: Option<i32>,
}

impl Chapter {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateChapterRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let content = req.content.clone().unwrap_or_default();
        let word_count = content.chars().count() as i32;
        let sort_order = req.sort_order.unwrap_or(0);

        let chapter = sqlx::query_as::<_, Self>(
            r#"INSERT INTO chapters (id, project_id, title, content, sort_order, word_count, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.project_id)
        .bind(req.title.as_deref().unwrap_or("未命名章节"))
        .bind(&content)
        .bind(sort_order)
        .bind(word_count)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(chapter)
    }

    pub async fn list_by_project(
        db: &sqlx::SqlitePool,
        project_id: &str,
    ) -> Result<Vec<Self>, crate::error::AppError> {
        let chapters = sqlx::query_as::<_, Self>(
            "SELECT * FROM chapters WHERE project_id = ? ORDER BY sort_order ASC",
        )
        .bind(project_id)
        .fetch_all(db)
        .await?;
        Ok(chapters)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let chapter = sqlx::query_as::<_, Self>("SELECT * FROM chapters WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::InvalidArgument("章节不存在".into()))?;
        Ok(chapter)
    }

    pub async fn update(
        db: &sqlx::SqlitePool,
        id: &str,
        req: &UpdateChapterRequest,
    ) -> Result<Self, crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let existing = Self::find_by_id(db, id).await?;

        let title = req.title.clone().unwrap_or(existing.title);
        let content = req.content.clone().unwrap_or(existing.content);
        let word_count = content.chars().count() as i32;
        let sort_order = req.sort_order.unwrap_or(existing.sort_order);

        let chapter = sqlx::query_as::<_, Self>(
            r#"UPDATE chapters SET title = ?, content = ?, sort_order = ?, word_count = ?, updated_at = ?
            WHERE id = ? RETURNING *"#,
        )
        .bind(&title)
        .bind(&content)
        .bind(sort_order)
        .bind(word_count)
        .bind(&now)
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(chapter)
    }

    pub async fn delete(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM chapters WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::InvalidArgument("章节不存在".into()));
        }
        Ok(())
    }
}
