use crate::error::AppError;
use crate::services::cloud_sync::{CloudConfig, CloudProviderConfig, CloudSyncService};
use crate::state::AppState;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_cloud_config(
    state: State<'_, AppState>,
    name: String,
    provider_type: String,
    config: CloudProviderConfig,
) -> Result<CloudConfig, AppError> {
    let db = state.db().await?;
    let id = Uuid::new_v4().to_string();
    let config_json = serde_json::to_string(&config)?;

    sqlx::query("INSERT INTO cloud_configs (id, name, provider_type, config) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&provider_type)
        .bind(&config_json)
        .execute(&db)
        .await?;

    Ok(CloudConfig {
        id,
        name,
        provider_type,
        config,
        is_active: false,
    })
}

#[tauri::command]
pub async fn list_cloud_configs(
    state: State<'_, AppState>,
) -> Result<Vec<CloudConfig>, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM cloud_configs ORDER BY created_at DESC")
        .fetch_all(&db)
        .await?;

    rows.into_iter().map(|row| {
        let config_json: String = row.get("config");
        let config: CloudProviderConfig = serde_json::from_str(&config_json)?;
        Ok(CloudConfig {
            id: row.get("id"),
            name: row.get("name"),
            provider_type: row.get("provider_type"),
            config,
            is_active: row.get::<i64, _>("is_active") != 0,
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn delete_cloud_config(
    state: State<'_, AppState>,
    config_id: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    sqlx::query("DELETE FROM cloud_configs WHERE id = ?")
        .bind(&config_id)
        .execute(&db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn upload_to_cloud(
    state: State<'_, AppState>,
    config_id: String,
    local_path: String,
    remote_path: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM cloud_configs WHERE id = ?")
        .bind(&config_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Internal("云配置不存在".into()))?;

    let config_json: String = row.get("config");
    let config: CloudProviderConfig = serde_json::from_str(&config_json)?;

    let cloud_config = CloudConfig {
        id: row.get("id"),
        name: row.get("name"),
        provider_type: row.get("provider_type"),
        config,
        is_active: row.get::<i64, _>("is_active") != 0,
    };

    let service = CloudSyncService::new();
    service.upload_file(&cloud_config, &local_path, &remote_path).await
}

#[tauri::command]
pub async fn download_from_cloud(
    state: State<'_, AppState>,
    config_id: String,
    remote_path: String,
    local_path: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM cloud_configs WHERE id = ?")
        .bind(&config_id)
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Internal("云配置不存在".into()))?;

    let config_json: String = row.get("config");
    let config: CloudProviderConfig = serde_json::from_str(&config_json)?;

    let cloud_config = CloudConfig {
        id: row.get("id"),
        name: row.get("name"),
        provider_type: row.get("provider_type"),
        config,
        is_active: row.get::<i64, _>("is_active") != 0,
    };

    let service = CloudSyncService::new();
    service.download_file(&cloud_config, &remote_path, &local_path).await
}
