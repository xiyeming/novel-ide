use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub chapter_id: String,
    pub chapter_title: String,
    pub snippet: String,
    pub rank: f64,
}

#[tauri::command]
pub async fn search_chapters(
    state: State<'_, AppState>,
    project_id: String,
    query: String,
    limit: Option<i32>,
) -> AppResult<Vec<SearchResult>> {
    let db = state.db().await?;
    let limit = limit.unwrap_or(20);

    let results = sqlx::query_as::<_, SearchResult>(
        r#"SELECT
            c.id as chapter_id,
            c.title as chapter_title,
            snippet(chapters_fts, 2, '<mark>', '</mark>', '...', 32) as snippet,
            rank
        FROM chapters_fts
        JOIN chapters c ON c.id = chapters_fts.chapter_id
        WHERE chapters_fts MATCH ? AND c.project_id = ?
        ORDER BY rank
        LIMIT ?"#,
    )
    .bind(&query)
    .bind(&project_id)
    .bind(limit)
    .fetch_all(&db)
    .await?;

    Ok(results)
}
