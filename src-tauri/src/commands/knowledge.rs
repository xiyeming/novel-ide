use crate::db::models::knowledge::{CreateKnowledgeDocumentRequest, KnowledgeDocument};
use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct KnowledgeSearchResult {
    pub doc_id: String,
    pub title: String,
    pub snippet: String,
    pub source_type: String,
    pub rank: f64,
}

#[tauri::command]
pub async fn import_knowledge(
    state: State<'_, AppState>,
    project_id: String,
    title: String,
    content: String,
    file_path: Option<String>,
) -> AppResult<KnowledgeDocument> {
    let db = state.db().await?;
    let req = CreateKnowledgeDocumentRequest {
        project_id,
        title,
        content,
        file_path,
        doc_type: None,
    };
    let doc = KnowledgeDocument::create(&db, &req).await?;
    Ok(doc)
}

#[tauri::command]
pub async fn list_knowledge(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<KnowledgeDocument>> {
    let db = state.db().await?;
    let docs = KnowledgeDocument::list_by_project(&db, &project_id).await?;
    Ok(docs)
}

#[tauri::command]
pub async fn delete_knowledge(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<()> {
    let db = state.db().await?;
    KnowledgeDocument::delete(&db, &id).await?;
    Ok(())
}

#[tauri::command]
pub async fn search_knowledge(
    state: State<'_, AppState>,
    project_id: String,
    query: String,
    limit: Option<i32>,
) -> AppResult<Vec<KnowledgeSearchResult>> {
    let db = state.db().await?;
    let limit = limit.unwrap_or(20);

    let results = sqlx::query_as::<_, KnowledgeSearchResult>(
        r#"SELECT
            k.id as doc_id,
            k.title,
            snippet(knowledge_documents_fts, 1, '<mark>', '</mark>', '...', 32) as snippet,
            'knowledge' as source_type,
            rank
        FROM knowledge_documents_fts
        JOIN knowledge_documents k ON k.rowid = knowledge_documents_fts.rowid
        WHERE knowledge_documents_fts MATCH ? AND k.project_id = ?
        UNION ALL
        SELECT
            c.id as doc_id,
            c.title,
            snippet(chapters_fts, 2, '<mark>', '</mark>', '...', 32) as snippet,
            'chapter' as source_type,
            rank
        FROM chapters_fts
        JOIN chapters c ON c.id = chapters_fts.chapter_id
        WHERE chapters_fts MATCH ? AND c.project_id = ?
        ORDER BY rank
        LIMIT ?"#,
    )
    .bind(&query)
    .bind(&project_id)
    .bind(&query)
    .bind(&project_id)
    .bind(limit)
    .fetch_all(&db)
    .await?;

    Ok(results)
}
