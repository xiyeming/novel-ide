use crate::error::AppResult;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofreadError {
    pub error_type: String,
    pub original: String,
    pub suggestion: String,
    pub line: usize,
    pub column: usize,
    pub confidence: f64,
}

#[tauri::command]
pub async fn proofread_chapter(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    provider_id: String,
    content: String,
) -> AppResult<Vec<ProofreadError>> {
    let db = state.db().await?;

    let provider =
        crate::db::models::model_provider::ModelProvider::find_by_id(&db, &provider_id).await?;

    let system_prompt = r#"你是一个专业的中文校对编辑。请分析以下文本，找出以下类型的错误：
1. 错别字 (typo) - 同音字、形近字错误
2. 语病 (grammar) - 语法不通、成分残缺
3. 标点错误 (punctuation) - 标点使用不当
4. 用词不当 (word_choice) - 词语搭配或语境不当
5. 逻辑错误 (logic) - 前后矛盾、时间线冲突

请以 JSON 数组格式返回，每个错误包含：
- error_type: 错误类型 (typo/grammar/punctuation/word_choice/logic)
- original: 原文
- suggestion: 建议修改
- line: 行号（从1开始）
- column: 列号（从1开始）
- confidence: 置信度 (0-1)

如果没有错误，返回空数组 []。只返回 JSON，不要添加其他说明文字。"#;

    let api_messages = serde_json::json!([
        {
            "role": "system",
            "content": system_prompt
        },
        {
            "role": "user",
            "content": content
        }
    ]);

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
                                    "proofread:chunk",
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
        "proofread:chunk",
        serde_json::json!({
            "content": "",
            "done": true
        }),
    );

    let errors = parse_proofread_response(&full_content)?;

    Ok(errors)
}

fn parse_proofread_response(response: &str) -> AppResult<Vec<ProofreadError>> {
    let trimmed = response.trim();

    if trimmed.is_empty() || trimmed == "[]" {
        return Ok(Vec::new());
    }

    let start = trimmed.find('[').unwrap_or(0);
    let end = trimmed.rfind(']').unwrap_or(trimmed.len());
    let json_str = &trimmed[start..=end];

    let errors: Vec<ProofreadError> =
        serde_json::from_str(json_str).map_err(|e| {
            crate::error::AppError::Internal(format!("解析校对结果失败: {}", e))
        })?;

    Ok(errors)
}
