use crate::db::models::model_provider::ModelProvider;
use crate::error::AppError;
use crate::models::workflow::WorkflowStage;
use crate::state::AppState;
use reqwest::Client;
use serde_json::json;

pub struct WorkflowEngine<'a> {
    state: &'a AppState,
    client: Client,
}

impl<'a> WorkflowEngine<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            client: Client::new(),
        }
    }

    pub async fn run_stage(
        &self,
        stage: &WorkflowStage,
        content: &str,
        _execution_id: &str,
    ) -> Result<String, AppError> {
        let provider_id = stage
            .model_provider_id
            .as_ref()
            .ok_or_else(|| AppError::Internal("阶段未配置模型".into()))?;

        let provider = self.get_provider(provider_id).await?;

        let system_prompt = stage
            .system_prompt
            .as_deref()
            .unwrap_or("你是一个专业的中文小说写作助手。");

        let body = json!({
            "model": provider.model_name,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": content}
            ],
            "temperature": stage.temperature,
            "max_tokens": stage.max_tokens,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", provider.api_url);
        let response = self
            .client
            .post(&url)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    provider.api_key.as_deref().unwrap_or("")
                ),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Http(e))?;

        let resp_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Http(e))?;

        let result = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::Internal("AI 响应格式错误".into()))?;

        Ok(result.to_string())
    }

    async fn get_provider(&self, id: &str) -> Result<ModelProvider, AppError> {
        let db = self.state.db().await?;
        ModelProvider::find_by_id(&db, id).await
    }
}
