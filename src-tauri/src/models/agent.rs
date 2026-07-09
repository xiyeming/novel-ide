use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: String, // outline/writer/proofreader/editor/custom
    pub system_prompt: String,
    pub model_provider_id: Option<String>,
    pub temperature: f64,
    pub max_tokens: u32,
    pub knowledge_base_ids: Vec<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
