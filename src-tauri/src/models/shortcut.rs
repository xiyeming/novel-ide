use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: String,
    pub action: String,
    pub key_binding: String,
    pub platform: String,
    pub is_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub theme_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
