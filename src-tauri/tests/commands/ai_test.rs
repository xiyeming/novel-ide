use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

use novel_ide_lib::db::models::model_provider::{
    CreateProviderRequest, ModelProvider, UpdateProviderRequest,
};

async fn setup_test_db() -> SqlitePool {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../../migrations/001_initial.sql"))
        .execute(&pool)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../../migrations/004_versions_models.sql"))
        .execute(&pool)
        .await
        .unwrap();

    pool
}

async fn create_test_provider(db: &SqlitePool) -> ModelProvider {
    let req = CreateProviderRequest {
        name: "测试提供者".to_string(),
        provider_type: "openai".to_string(),
        api_url: "https://api.openai.com".to_string(),
        api_key: Some("test-key".to_string()),
        model_name: "gpt-4".to_string(),
        is_default: Some(true),
        config: None,
    };
    ModelProvider::create(db, &req).await.unwrap()
}

#[tokio::test]
async fn test_create_provider() {
    let db = setup_test_db().await;

    let req = CreateProviderRequest {
        name: "OpenAI".to_string(),
        provider_type: "openai".to_string(),
        api_url: "https://api.openai.com".to_string(),
        api_key: Some("sk-test".to_string()),
        model_name: "gpt-4".to_string(),
        is_default: Some(true),
        config: None,
    };

    let provider = ModelProvider::create(&db, &req).await.unwrap();

    assert!(!provider.id.is_empty());
    assert_eq!(provider.name, "OpenAI");
    assert_eq!(provider.provider_type, "openai");
    assert_eq!(provider.api_url, "https://api.openai.com");
    assert_eq!(provider.api_key.as_deref(), Some("sk-test"));
    assert_eq!(provider.model_name, "gpt-4");
    assert!(provider.is_default);
}

#[tokio::test]
async fn test_list_providers() {
    let db = setup_test_db().await;

    let req1 = CreateProviderRequest {
        name: "OpenAI".to_string(),
        provider_type: "openai".to_string(),
        api_url: "https://api.openai.com".to_string(),
        api_key: None,
        model_name: "gpt-4".to_string(),
        is_default: Some(true),
        config: None,
    };

    let req2 = CreateProviderRequest {
        name: "Claude".to_string(),
        provider_type: "anthropic".to_string(),
        api_url: "https://api.anthropic.com".to_string(),
        api_key: None,
        model_name: "claude-3".to_string(),
        is_default: Some(false),
        config: None,
    };

    ModelProvider::create(&db, &req1).await.unwrap();
    ModelProvider::create(&db, &req2).await.unwrap();

    let providers = ModelProvider::list_all(&db).await.unwrap();
    assert_eq!(providers.len(), 2);
}

#[tokio::test]
async fn test_find_provider_by_id() {
    let db = setup_test_db().await;
    let created = create_test_provider(&db).await;

    let found = ModelProvider::find_by_id(&db, &created.id).await.unwrap();

    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "测试提供者");
}

#[tokio::test]
async fn test_find_nonexistent_provider() {
    let db = setup_test_db().await;
    let result = ModelProvider::find_by_id(&db, "nonexistent-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_provider() {
    let db = setup_test_db().await;
    let created = create_test_provider(&db).await;

    let update_req = UpdateProviderRequest {
        name: Some("更新后的提供者".to_string()),
        api_url: Some("https://api.new-url.com".to_string()),
        api_key: Some("new-key".to_string()),
        model_name: Some("gpt-4-turbo".to_string()),
        is_default: Some(false),
        config: Some("{\"temperature\": 0.8}".to_string()),
    };

    let updated = ModelProvider::update(&db, &created.id, &update_req)
        .await
        .unwrap();

    assert_eq!(updated.name, "更新后的提供者");
    assert_eq!(updated.api_url, "https://api.new-url.com");
    assert_eq!(updated.api_key.as_deref(), Some("new-key"));
    assert_eq!(updated.model_name, "gpt-4-turbo");
    assert!(!updated.is_default);
    assert_eq!(
        updated.config.as_deref(),
        Some("{\"temperature\": 0.8}")
    );
}

#[tokio::test]
async fn test_update_provider_partial() {
    let db = setup_test_db().await;
    let created = create_test_provider(&db).await;

    let update_req = UpdateProviderRequest {
        name: Some("部分更新".to_string()),
        api_url: None,
        api_key: None,
        model_name: None,
        is_default: None,
        config: None,
    };

    let updated = ModelProvider::update(&db, &created.id, &update_req)
        .await
        .unwrap();

    assert_eq!(updated.name, "部分更新");
    assert_eq!(updated.api_url, "https://api.openai.com");
    assert_eq!(updated.api_key.as_deref(), Some("test-key"));
}

#[tokio::test]
async fn test_update_nonexistent_provider() {
    let db = setup_test_db().await;
    let update_req = UpdateProviderRequest {
        name: None,
        api_url: None,
        api_key: None,
        model_name: None,
        is_default: None,
        config: None,
    };

    let result = ModelProvider::update(&db, "nonexistent-id", &update_req).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_provider() {
    let db = setup_test_db().await;
    let created = create_test_provider(&db).await;

    ModelProvider::delete(&db, &created.id).await.unwrap();

    let result = ModelProvider::find_by_id(&db, &created.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_provider() {
    let db = setup_test_db().await;
    let result = ModelProvider::delete(&db, "nonexistent-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_provider_list_ordering() {
    let db = setup_test_db().await;

    let req1 = CreateProviderRequest {
        name: "非默认提供者".to_string(),
        provider_type: "openai".to_string(),
        api_url: "https://api.openai.com".to_string(),
        api_key: None,
        model_name: "gpt-4".to_string(),
        is_default: Some(false),
        config: None,
    };

    let req2 = CreateProviderRequest {
        name: "默认提供者".to_string(),
        provider_type: "anthropic".to_string(),
        api_url: "https://api.anthropic.com".to_string(),
        api_key: None,
        model_name: "claude-3".to_string(),
        is_default: Some(true),
        config: None,
    };

    ModelProvider::create(&db, &req1).await.unwrap();
    ModelProvider::create(&db, &req2).await.unwrap();

    let providers = ModelProvider::list_all(&db).await.unwrap();
    assert_eq!(providers.len(), 2);
    assert!(providers[0].is_default);
    assert!(!providers[1].is_default);
}
