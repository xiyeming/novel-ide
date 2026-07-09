use crate::db::models::model_provider::ModelProvider;
use crate::error::AppError;
use crate::state::AppState;
use reqwest::Client;
use serde_json::json;

pub struct AIFeaturesService<'a> {
    state: &'a AppState,
    client: Client,
}

impl<'a> AIFeaturesService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            client: Client::new(),
        }
    }

    pub async fn continue_writing(
        &self,
        content: &str,
        provider_id: &str,
        style: Option<&str>,
    ) -> Result<String, AppError> {
        let style_prompt = style.unwrap_or("保持原文风格");
        let prompt = format!(
            r#"请继续以下小说内容，{style_prompt}，保持情节连贯，字数约500字：

{content}

请直接输出续写内容，不要添加任何解释或前缀。"#,
            style_prompt = style_prompt,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn rewrite(
        &self,
        content: &str,
        provider_id: &str,
        instruction: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请改写以下内容，要求：{instruction}

原文：
{content}

请直接输出改写后的内容，不要添加任何解释或前缀。"#,
            instruction = instruction,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn expand(
        &self,
        content: &str,
        provider_id: &str,
        target_words: Option<u32>,
    ) -> Result<String, AppError> {
        let words = target_words.unwrap_or(1000);
        let prompt = format!(
            r#"请扩写以下内容，目标字数约{words}字，丰富细节描写、心理活动和环境描写：

{content}

请直接输出扩写后的内容，不要添加任何解释或前缀。"#,
            words = words,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn condense(
        &self,
        content: &str,
        provider_id: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请精简以下内容，保留核心情节和人物，删除冗余描写：

{content}

请直接输出精简后的内容，不要添加任何解释或前缀。"#,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    pub async fn style_transfer(
        &self,
        content: &str,
        provider_id: &str,
        target_style: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            r#"请将以下内容转换为{target_style}风格：

{content}

请直接输出转换后的内容，不要添加任何解释或前缀。"#,
            target_style = target_style,
            content = content
        );
        
        self.call_ai(&prompt, provider_id).await
    }

    async fn call_ai(&self, prompt: &str, provider_id: &str) -> Result<String, AppError> {
        let db = self.state.db().await?;
        let provider = ModelProvider::find_by_id(&db, provider_id).await?;

        let body = json!({
            "model": provider.model_name,
            "messages": [
                {"role": "system", "content": "你是一个专业的中文小说写作助手，擅长各种写作风格。"},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.7,
            "max_tokens": 2000,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", provider.api_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", provider.api_key.as_deref().unwrap_or("")))
            .json(&body)
            .send()
            .await
            .map_err(AppError::Http)?;

        let resp_json: serde_json::Value = response.json().await
            .map_err(AppError::Http)?;

        let result = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::Internal("AI 响应格式错误".into()))?;

        Ok(result.to_string())
    }
}
