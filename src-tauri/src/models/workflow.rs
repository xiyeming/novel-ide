use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<WorkflowStage>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub name: String,
    pub stage_type: String, // outline/draft/proofread/edit/custom
    pub agent_id: Option<String>,
    pub model_provider_id: Option<String>,
    pub system_prompt: Option<String>,
    pub temperature: f64,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    pub chapter_id: String,
    pub status: String, // pending/running/completed/failed
    pub current_stage: usize,
    pub results: std::collections::HashMap<String, String>,
    pub error: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}
