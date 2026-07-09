use crate::error::AppResult;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State};
use tokio_stream::StreamExt;

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub content: String,
    pub finish_reason: Option<String>,
}

#[tauri::command]
pub async fn continue_writing(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    style: Option<String>,
) -> AppResult<String> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.continue_writing(&content, &provider_id, style.as_deref()).await
}

#[tauri::command]
pub async fn rewrite_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    instruction: String,
) -> AppResult<String> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.rewrite(&content, &provider_id, &instruction).await
}

#[tauri::command]
pub async fn expand_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    target_words: Option<u32>,
) -> AppResult<String> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.expand(&content, &provider_id, target_words).await
}

#[tauri::command]
pub async fn condense_content(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
) -> AppResult<String> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.condense(&content, &provider_id).await
}

#[tauri::command]
pub async fn style_transfer(
    state: State<'_, AppState>,
    content: String,
    provider_id: String,
    target_style: String,
) -> AppResult<String> {
    let service = crate::services::ai_features::AIFeaturesService::new(&state);
    service.style_transfer(&content, &provider_id, &target_style).await
}

#[tauri::command]
pub async fn chat_stream(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    provider_id: String,
    messages: Vec<ChatMessage>,
    system_prompt: Option<String>,
) -> AppResult<ChatResponse> {
    let db = state.db().await?;

    let provider =
        crate::db::models::model_provider::ModelProvider::find_by_id(&db, &provider_id).await?;

    let mut api_messages = Vec::new();
    if let Some(sys) = &system_prompt {
        api_messages.push(serde_json::json!({
            "role": "system",
            "content": sys
        }));
    }
    for msg in &messages {
        api_messages.push(serde_json::json!({
            "role": msg.role,
            "content": msg.content
        }));
    }

    let request_body = serde_json::json!({
        "model": provider.model_name,
        "messages": api_messages,
        "stream": true
    });

    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", provider.api_url.trim_end_matches('/'));

    let mut req = client
        .post(&url)
        .header("Content-Type", "application/json");

    if let Some(key) = &provider.api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.json(&request_body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(crate::error::AppError::Internal(format!(
            "API 错误 ({}): {}",
            status, body
        )));
    }

    let mut full_content = String::new();
    let mut buffer = String::new();

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
        buffer.push_str(&String::from_utf8_lossy(&bytes));

        while let Some(line_end) = buffer.find('\n') {
            let line = buffer[..line_end].trim().to_string();
            buffer = buffer[line_end + 1..].to_string();

            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(choices) = json["choices"].as_array() {
                        if let Some(choice) = choices.first() {
                            if let Some(delta) = choice["delta"]["content"].as_str() {
                                full_content.push_str(delta);
                                let _ = app.emit(
                                    "ai:chunk",
                                    serde_json::json!({
                                        "content": delta,
                                        "done": false
                                    }),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = app.emit(
        "ai:chunk",
        serde_json::json!({
            "content": "",
            "done": true
        }),
    );

    Ok(ChatResponse {
        content: full_content,
        finish_reason: Some("stop".into()),
    })
}
