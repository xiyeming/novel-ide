use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModelProvider {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub is_default: bool,
    pub config: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub is_default: Option<bool>,
    pub config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub is_default: Option<bool>,
    pub config: Option<String>,
}

impl ModelProvider {
    pub async fn create(
        db: &sqlx::SqlitePool,
        req: &CreateProviderRequest,
    ) -> Result<Self, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let is_default = req.is_default.unwrap_or(false);

        let provider = sqlx::query_as::<_, Self>(
            r#"INSERT INTO model_providers (id, name, provider_type, api_url, api_key,
                model_name, is_default, config, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&req.provider_type)
        .bind(&req.api_url)
        .bind(&req.api_key)
        .bind(&req.model_name)
        .bind(is_default)
        .bind(&req.config)
        .bind(&now)
        .bind(&now)
        .fetch_one(db)
        .await?;

        Ok(provider)
    }

    pub async fn list_all(db: &sqlx::SqlitePool) -> Result<Vec<Self>, crate::error::AppError> {
        let providers = sqlx::query_as::<_, Self>(
            "SELECT * FROM model_providers ORDER BY is_default DESC, name ASC",
        )
        .fetch_all(db)
        .await?;
        Ok(providers)
    }

    pub async fn find_by_id(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Self, crate::error::AppError> {
        let provider = sqlx::query_as::<_, Self>("SELECT * FROM model_providers WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| crate::error::AppError::ProviderNotFound(id.to_string()))?;
        Ok(provider)
    }

    pub async fn update(
        db: &sqlx::SqlitePool,
        id: &str,
        req: &UpdateProviderRequest,
    ) -> Result<Self, crate::error::AppError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        sqlx::query_as::<_, Self>(
            r#"UPDATE model_providers SET
                name = COALESCE(?, name),
                api_url = COALESCE(?, api_url),
                api_key = COALESCE(?, api_key),
                model_name = COALESCE(?, model_name),
                is_default = COALESCE(?, is_default),
                config = COALESCE(?, config),
                updated_at = ?
            WHERE id = ?
            RETURNING *"#,
        )
        .bind(&req.name)
        .bind(&req.api_url)
        .bind(&req.api_key)
        .bind(&req.model_name)
        .bind(req.is_default)
        .bind(&req.config)
        .bind(&now)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| crate::error::AppError::ProviderNotFound(id.to_string()))
    }

    pub async fn delete(db: &sqlx::SqlitePool, id: &str) -> Result<(), crate::error::AppError> {
        let result = sqlx::query("DELETE FROM model_providers WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::AppError::ProviderNotFound(id.to_string()));
        }
        Ok(())
    }
}
