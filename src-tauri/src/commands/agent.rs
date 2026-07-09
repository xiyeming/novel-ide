use crate::error::AppError;
use crate::models::agent::Agent;
use crate::state::AppState;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_agent(
    state: State<'_, AppState>,
    name: String,
    role: String,
    system_prompt: String,
    model_provider_id: Option<String>,
    temperature: f64,
    max_tokens: u32,
    knowledge_base_ids: Vec<String>,
) -> Result<Agent, AppError> {
    let db = state.db().await?;
    let id = Uuid::new_v4().to_string();
    let kb_ids_json = serde_json::to_string(&knowledge_base_ids)?;

    sqlx::query("INSERT INTO agents (id, name, role, system_prompt, model_provider_id, temperature, max_tokens, knowledge_base_ids) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&role)
        .bind(&system_prompt)
        .bind(&model_provider_id)
        .bind(temperature)
        .bind(max_tokens as i64)
        .bind(&kb_ids_json)
        .execute(&db)
        .await?;

    Ok(Agent {
        id,
        name,
        role,
        system_prompt,
        model_provider_id,
        temperature,
        max_tokens,
        knowledge_base_ids,
        is_active: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub async fn list_agents(
    state: State<'_, AppState>,
) -> Result<Vec<Agent>, AppError> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM agents ORDER BY created_at DESC")
        .fetch_all(&db)
        .await?;

    rows.into_iter().map(|row| {
        let kb_ids_json: String = row.get("knowledge_base_ids");
        let knowledge_base_ids: Vec<String> = serde_json::from_str(&kb_ids_json)?;
        Ok(Agent {
            id: row.get("id"),
            name: row.get("name"),
            role: row.get("role"),
            system_prompt: row.get("system_prompt"),
            model_provider_id: row.get("model_provider_id"),
            temperature: row.get("temperature"),
            max_tokens: row.get::<i64, _>("max_tokens") as u32,
            knowledge_base_ids,
            is_active: row.get::<i64, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }).collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn update_agent(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<u32>,
    knowledge_base_ids: Option<Vec<String>>,
) -> Result<Agent, AppError> {
    let db = state.db().await?;

    if let Some(n) = name {
        sqlx::query("UPDATE agents SET name = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&n).bind(&id).execute(&db).await?;
    }
    if let Some(sp) = system_prompt {
        sqlx::query("UPDATE agents SET system_prompt = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&sp).bind(&id).execute(&db).await?;
    }
    if let Some(t) = temperature {
        sqlx::query("UPDATE agents SET temperature = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(t).bind(&id).execute(&db).await?;
    }
    if let Some(mt) = max_tokens {
        sqlx::query("UPDATE agents SET max_tokens = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(mt as i64).bind(&id).execute(&db).await?;
    }
    if let Some(kb) = knowledge_base_ids {
        let json = serde_json::to_string(&kb)?;
        sqlx::query("UPDATE agents SET knowledge_base_ids = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&json).bind(&id).execute(&db).await?;
    }

    // Return updated agent
    let row = sqlx::query("SELECT * FROM agents WHERE id = ?")
        .bind(&id)
        .fetch_one(&db)
        .await?;

    let kb_ids_json: String = row.get("knowledge_base_ids");
    let knowledge_base_ids: Vec<String> = serde_json::from_str(&kb_ids_json)?;

    Ok(Agent {
        id: row.get("id"),
        name: row.get("name"),
        role: row.get("role"),
        system_prompt: row.get("system_prompt"),
        model_provider_id: row.get("model_provider_id"),
        temperature: row.get("temperature"),
        max_tokens: row.get::<i64, _>("max_tokens") as u32,
        knowledge_base_ids,
        is_active: row.get::<i64, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn delete_agent(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let db = state.db().await?;
    sqlx::query("DELETE FROM agents WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await?;
    Ok(())
}
