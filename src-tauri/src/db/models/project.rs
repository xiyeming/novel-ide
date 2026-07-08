// src-tauri/src/db/models/project.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub genre: Option<String>,
    pub sub_genre: Option<String>,
    pub target_readers: Option<String>,
    pub total_chapters: Option<i32>,
    pub words_per_chapter: Option<i32>,
    pub narrative_pov: Option<String>,
    pub story_structure: Option<String>,
    pub core_outline: Option<String>,
    pub world_settings: Option<String>,
    pub character_profiles: Option<String>,
    pub golden_finger: Option<String>,
    pub writing_constraints: Option<String>,
    pub style_constraints: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub path: String,
    pub genre: Option<String>,
    pub sub_genre: Option<String>,
    pub target_readers: Option<String>,
    pub total_chapters: Option<i32>,
    pub words_per_chapter: Option<i32>,
    pub narrative_pov: Option<String>,
    pub story_structure: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub genre: Option<String>,
    pub sub_genre: Option<String>,
    pub target_readers: Option<String>,
    pub total_chapters: Option<i32>,
    pub words_per_chapter: Option<i32>,
    pub narrative_pov: Option<String>,
    pub story_structure: Option<String>,
    pub core_outline: Option<String>,
    pub world_settings: Option<String>,
    pub character_profiles: Option<String>,
    pub golden_finger: Option<String>,
    pub writing_constraints: Option<String>,
    pub style_constraints: Option<String>,
}

impl Project {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateProjectRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let project = sqlx::query_as::<_, Self>(
            r#"INSERT INTO projects (id, name, path, genre, sub_genre, target_readers,
                total_chapters, words_per_chapter, narrative_pov, story_structure,
                created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&req.path)
        .bind(&req.genre)
        .bind(&req.sub_genre)
        .bind(&req.target_readers)
        .bind(req.total_chapters)
        .bind(req.words_per_chapter.unwrap_or(3000))
        .bind(&req.narrative_pov)
        .bind(&req.story_structure)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(project)
    }

    pub async fn list_all(db: &sqlx::SqlitePool) -> Result<Vec<Self>, crate::error::AppError> {
        let projects = sqlx::query_as::<_, Self>("SELECT * FROM projects ORDER BY updated_at DESC")
            .fetch_all(db)
            .await?;
        Ok(projects)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let project = sqlx::query_as::<_, Self>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::ProjectNotFound(id.to_string()))?;
        Ok(project)
    }

    pub async fn delete(db: &sqlx::SqlitePool, id: &str) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::ProjectNotFound(id.to_string()));
        }
        Ok(())
    }

    pub async fn update(
        db: &sqlx::SqlitePool,
        id: &str,
        req: &UpdateProjectRequest,
    ) -> Result<Self, crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        sqlx::query_as::<_, Self>(
            r#"UPDATE projects SET
                genre = COALESCE(?, genre),
                sub_genre = COALESCE(?, sub_genre),
                target_readers = COALESCE(?, target_readers),
                total_chapters = COALESCE(?, total_chapters),
                words_per_chapter = COALESCE(?, words_per_chapter),
                narrative_pov = COALESCE(?, narrative_pov),
                story_structure = COALESCE(?, story_structure),
                core_outline = COALESCE(?, core_outline),
                world_settings = COALESCE(?, world_settings),
                character_profiles = COALESCE(?, character_profiles),
                golden_finger = COALESCE(?, golden_finger),
                writing_constraints = COALESCE(?, writing_constraints),
                style_constraints = COALESCE(?, style_constraints),
                updated_at = ?
            WHERE id = ?
            RETURNING *"#,
        )
        .bind(&req.genre)
        .bind(&req.sub_genre)
        .bind(&req.target_readers)
        .bind(req.total_chapters)
        .bind(req.words_per_chapter)
        .bind(&req.narrative_pov)
        .bind(&req.story_structure)
        .bind(&req.core_outline)
        .bind(&req.world_settings)
        .bind(&req.character_profiles)
        .bind(&req.golden_finger)
        .bind(&req.writing_constraints)
        .bind(&req.style_constraints)
        .bind(&now)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| crate::error::AppError::ProjectNotFound(id.to_string()))
    }
}
