// src-tauri/src/db/models/knowledge.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KnowledgeDocument {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub doc_type: String,
    pub chunk_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateKnowledgeDocumentRequest {
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub doc_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKnowledgeDocumentRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub file_path: Option<String>,
    pub doc_type: Option<String>,
    pub chunk_count: Option<i32>,
}

impl KnowledgeDocument {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateKnowledgeDocumentRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let doc_type = req.doc_type.clone().unwrap_or_else(|| "text".into());

        let doc = sqlx::query_as::<_, Self>(
            r#"INSERT INTO knowledge_documents (id, project_id, title, content, file_path, doc_type, chunk_count, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.project_id)
        .bind(&req.title)
        .bind(&req.content)
        .bind(&req.file_path)
        .bind(&doc_type)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(doc)
    }

    pub async fn list_by_project(
        db: &sqlx::SqlitePool,
        project_id: &str,
    ) -> Result<Vec<Self>, crate::error::AppError> {
        let docs = sqlx::query_as::<_, Self>(
            "SELECT * FROM knowledge_documents WHERE project_id = ? ORDER BY created_at DESC",
        )
        .bind(project_id)
        .fetch_all(db)
        .await?;
        Ok(docs)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let doc = sqlx::query_as::<_, Self>("SELECT * FROM knowledge_documents WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::InvalidArgument("知识文档不存在".into()))?;
        Ok(doc)
    }

    pub async fn update(
        db: &sqlx::SqlitePool,
        id: &str,
        req: &UpdateKnowledgeDocumentRequest,
    ) -> Result<Self, crate::error::AppError> {
        let now = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let existing = Self::find_by_id(db, id).await?;

        let title = req.title.clone().unwrap_or(existing.title);
        let content = req.content.clone().unwrap_or(existing.content);
        let file_path = req.file_path.clone().or(existing.file_path);
        let doc_type = req.doc_type.clone().unwrap_or(existing.doc_type);
        let chunk_count = req.chunk_count.unwrap_or(existing.chunk_count);

        let doc = sqlx::query_as::<_, Self>(
            r#"UPDATE knowledge_documents SET title = ?, content = ?, file_path = ?, doc_type = ?, chunk_count = ?, updated_at = ?
            WHERE id = ? RETURNING *"#,
        )
        .bind(&title)
        .bind(&content)
        .bind(&file_path)
        .bind(&doc_type)
        .bind(chunk_count)
        .bind(&now)
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(doc)
    }

    pub async fn delete(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM knowledge_documents WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::InvalidArgument(
                "知识文档不存在".into(),
            ));
        }
        Ok(())
    }
}
