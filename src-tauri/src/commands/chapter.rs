use crate::db::models::chapter::{Chapter, CreateChapterRequest, UpdateChapterRequest};
use crate::error::AppResult;
use crate::services::performance::PerformanceService;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_chapter(
    state: State<'_, AppState>,
    project_id: String,
    title: Option<String>,
    content: Option<String>,
    sort_order: Option<i32>,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let req = CreateChapterRequest {
        project_id,
        title,
        content,
        sort_order,
    };
    let chapter = Chapter::create(&db, &req).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn list_chapters(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<Chapter>> {
    let db = state.db().await?;
    let chapters = Chapter::list_by_project(&db, &project_id).await?;
    Ok(chapters)
}

#[tauri::command]
pub async fn get_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let chapter = Chapter::find_by_id(&db, &chapter_id).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn update_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
    title: Option<String>,
    content: Option<String>,
    sort_order: Option<i32>,
) -> AppResult<Chapter> {
    let db = state.db().await?;
    let req = UpdateChapterRequest {
        title,
        content,
        sort_order,
    };
    let chapter = Chapter::update(&db, &chapter_id, &req).await?;
    Ok(chapter)
}

#[tauri::command]
pub async fn delete_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
) -> AppResult<()> {
    let db = state.db().await?;
    Chapter::delete(&db, &chapter_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn read_chapter_chunk(
    file_path: String,
    start_line: usize,
    max_lines: usize,
) -> AppResult<String> {
    let service = PerformanceService::new();
    let content = service.read_file_chunk(&file_path, start_line, max_lines)?;
    Ok(content)
}

#[tauri::command]
pub async fn get_chapter_file_info(file_path: String) -> AppResult<(usize, u64)> {
    let service = PerformanceService::new();
    let line_count = service.count_lines(&file_path)?;
    let file_size = service.get_file_size(&file_path)?;
    Ok((line_count, file_size))
}
