use crate::error::{AppError, AppResult};
use crate::models::workflow::{Workflow, WorkflowExecution, WorkflowStage};
use crate::services::workflow_engine::WorkflowEngine;
use crate::state::AppState;
use sqlx::Row;
use tauri::State;

#[tauri::command]
pub async fn create_workflow(
    state: State<'_, AppState>,
    name: String,
    description: Option<String>,
    stages: Vec<WorkflowStage>,
) -> AppResult<Workflow> {
    let db = state.db().await?;
    let id = uuid::Uuid::new_v4().to_string();
    let stages_json = serde_json::to_string(&stages)?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO workflows (id, name, description, stages, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&description)
    .bind(&stages_json)
    .bind(&now)
    .bind(&now)
    .execute(&db)
    .await?;

    Ok(Workflow {
        id,
        name,
        description,
        stages,
        is_active: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_workflows(state: State<'_, AppState>) -> AppResult<Vec<Workflow>> {
    let db = state.db().await?;
    let rows = sqlx::query("SELECT * FROM workflows ORDER BY created_at DESC")
        .fetch_all(&db)
        .await?;

    rows.into_iter()
        .map(|row| {
            let stages_json: String = row.get("stages");
    let stages: Vec<WorkflowStage> = serde_json::from_str(&stages_json)?;
            Ok(Workflow {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                stages,
                is_active: row.get::<i64, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub async fn delete_workflow(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let db = state.db().await?;
    let result = sqlx::query("DELETE FROM workflows WHERE id = ?")
        .bind(&id)
        .execute(&db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::Internal("工作流不存在".into()));
    }
    Ok(())
}

#[tauri::command]
pub async fn execute_workflow(
    state: State<'_, AppState>,
    workflow_id: String,
    chapter_id: String,
) -> AppResult<WorkflowExecution> {
    let db = state.db().await?;
    let id = uuid::Uuid::new_v4().to_string();

    let row = sqlx::query("SELECT * FROM workflows WHERE id = ?")
        .bind(&workflow_id)
        .fetch_optional(&db)
        .await?
        .ok_or_else(|| AppError::Internal("工作流不存在".into()))?;

    let stages_json: String = row.get("stages");
    let stages: Vec<WorkflowStage> = serde_json::from_str(&stages_json)?;

    let results = std::collections::HashMap::new();
    let results_json = serde_json::to_string(&results)?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO workflow_executions (id, workflow_id, chapter_id, status, current_stage, results, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&workflow_id)
    .bind(&chapter_id)
    .bind("pending")
    .bind(stages.len() as i64)
    .bind(&results_json)
    .bind(&now)
    .execute(&db)
    .await?;

    Ok(WorkflowExecution {
        id,
        workflow_id,
        chapter_id,
        status: "pending".into(),
        current_stage: 0,
        results,
        error: None,
        started_at: None,
        completed_at: None,
        created_at: now,
    })
}

#[tauri::command]
pub async fn get_workflow_execution(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<WorkflowExecution> {
    let db = state.db().await?;
    let row = sqlx::query("SELECT * FROM workflow_executions WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db)
        .await?
        .ok_or_else(|| AppError::Internal("执行记录不存在".into()))?;

    let results_json: String = row.get("results");
    let results: std::collections::HashMap<String, String> = serde_json::from_str(&results_json)?;

    Ok(WorkflowExecution {
        id: row.get("id"),
        workflow_id: row.get("workflow_id"),
        chapter_id: row.get("chapter_id"),
        status: row.get("status"),
        current_stage: row.get::<i64, _>("current_stage") as usize,
        results,
        error: row.get("error"),
        started_at: row.get("started_at"),
        completed_at: row.get("completed_at"),
        created_at: row.get("created_at"),
    })
}

#[tauri::command]
pub async fn run_workflow_stage(
    state: State<'_, AppState>,
    execution_id: String,
    stage_index: usize,
    content: String,
) -> AppResult<String> {
    let engine = WorkflowEngine::new(&state);

    let db = state.db().await?;
    let execution = sqlx::query("SELECT * FROM workflow_executions WHERE id = ?")
        .bind(&execution_id)
        .fetch_optional(&db)
        .await?
        .ok_or_else(|| AppError::Internal("执行记录不存在".into()))?;

    let workflow_id: String = execution.get("workflow_id");
    let row = sqlx::query("SELECT * FROM workflows WHERE id = ?")
        .bind(&workflow_id)
        .fetch_one(&db)
        .await?;

    let stages_json: String = row.get("stages");
    let stages: Vec<WorkflowStage> = serde_json::from_str(&stages_json)?;

    let stage = stages
        .get(stage_index)
        .ok_or_else(|| AppError::Internal("阶段索引越界".into()))?;

    let result = engine.run_stage(stage, &content, &execution_id).await?;

    let mut results: std::collections::HashMap<String, String> =
        serde_json::from_str(&execution.get::<String, _>("results")).unwrap_or_default();
    results.insert(stage_index.to_string(), result.clone());

    sqlx::query("UPDATE workflow_executions SET results = ?, current_stage = ? WHERE id = ?")
        .bind(serde_json::to_string(&results)?)
        .bind(stage_index as i64)
        .bind(&execution_id)
        .execute(&db)
        .await?;

    Ok(result)
}
