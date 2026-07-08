use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChapterVersion {
    pub id: String,
    pub chapter_id: String,
    pub version_number: i32,
    pub content: String,
    pub word_count: i32,
    pub created_at: String,
}

impl ChapterVersion {
    pub async fn save(
        db: &sqlx::SqlitePool,
        chapter_id: &str,
        content: &str,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let word_count = content.chars().count() as i32;

        let max_version: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(version_number) FROM chapter_versions WHERE chapter_id = ?",
        )
        .bind(chapter_id)
        .fetch_optional(db)
        .await?;
        let version_number = max_version.unwrap_or(0) + 1;

        let version = sqlx::query_as::<_, Self>(
            r#"INSERT INTO chapter_versions (id, chapter_id, version_number, content, word_count, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(chapter_id)
        .bind(version_number)
        .bind(content)
        .bind(word_count)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(version)
    }

    pub async fn list_by_chapter(
        db: &sqlx::SqlitePool,
        chapter_id: &str,
    ) -> Result<Vec<Self>, crate::error::AppError> {
        let versions = sqlx::query_as::<_, Self>(
            "SELECT * FROM chapter_versions WHERE chapter_id = ? ORDER BY version_number DESC",
        )
        .bind(chapter_id)
        .fetch_all(db)
        .await?;
        Ok(versions)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let version = sqlx::query_as::<_, Self>("SELECT * FROM chapter_versions WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::InvalidArgument("版本不存在".into()))?;
        Ok(version)
    }
}
