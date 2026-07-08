use crate::db::models::model_provider::{
    CreateProviderRequest, ModelProvider, UpdateProviderRequest,
};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_provider(
    state: State<'_, AppState>,
    name: String,
    provider_type: String,
    api_url: String,
    api_key: Option<String>,
    model_name: String,
    is_default: Option<bool>,
    config: Option<String>,
) -> AppResult<ModelProvider> {
    let db = state.db().await?;
    let req = CreateProviderRequest {
        name,
        provider_type,
        api_url,
        api_key,
        model_name,
        is_default,
        config,
    };
    let provider = ModelProvider::create(&db, &req).await?;
    Ok(provider)
}

#[tauri::command]
pub async fn list_providers(state: State<'_, AppState>) -> AppResult<Vec<ModelProvider>> {
    let db = state.db().await?;
    let providers = ModelProvider::list_all(&db).await?;
    Ok(providers)
}

#[tauri::command]
pub async fn update_provider(
    state: State<'_, AppState>,
    provider_id: String,
    name: Option<String>,
    api_url: Option<String>,
    api_key: Option<String>,
    model_name: Option<String>,
    is_default: Option<bool>,
    config: Option<String>,
) -> AppResult<ModelProvider> {
    let db = state.db().await?;
    let req = UpdateProviderRequest {
        name,
        api_url,
        api_key,
        model_name,
        is_default,
        config,
    };
    let provider = ModelProvider::update(&db, &provider_id, &req).await?;
    Ok(provider)
}

#[tauri::command]
pub async fn delete_provider(
    state: State<'_, AppState>,
    provider_id: String,
) -> AppResult<()> {
    let db = state.db().await?;
    ModelProvider::delete(&db, &provider_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    provider_id: String,
) -> AppResult<String> {
    let db = state.db().await?;
    let provider = ModelProvider::find_by_id(&db, &provider_id).await?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

    let mut request = client.get(&provider.api_url);

    if let Some(ref api_key) = provider.api_key {
        request = request.bearer_auth(api_key);
    }

    let response = request
        .send()
        .await
        .map_err(|e| crate::error::AppError::ConnectionTestFailed(e.to_string()))?;

    let status = response.status();
    if status.is_success() {
        Ok(format!("连接成功 (HTTP {})", status.as_u16()))
    } else {
        Err(crate::error::AppError::ConnectionTestFailed(format!(
            "连接失败 (HTTP {})",
            status.as_u16()
        )))
    }
}
