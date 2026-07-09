use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub file_path: String,
    pub file_size: u64,
}

#[tauri::command]
pub async fn export_chapter(
    state: State<'_, AppState>,
    chapter_id: String,
    format: String,
    output_path: String,
) -> AppResult<ExportResult> {
    let db = state.db().await?;

    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;

    let project = crate::db::models::project::Project::find_by_id(&db, &chapter.project_id).await?;

    let content = match format.as_str() {
        "txt" => format!("{}\n\n{}", chapter.title, chapter.content),
        "md" => format!("# {}\n\n{}", chapter.title, chapter.content),
        _ => return Err(crate::error::AppError::InvalidArgument("不支持的导出格式".into())),
    };

    let file_name = format!("{}_{}.{}", project.name, chapter.title, format);
    let file_path = std::path::Path::new(&output_path).join(&file_name);
    std::fs::write(&file_path, content)?;

    let file_size = std::fs::metadata(&file_path)?.len();

    Ok(ExportResult {
        file_path: file_path.to_string_lossy().to_string(),
        file_size,
    })
}

#[tauri::command]
pub async fn export_all_chapters(
    state: State<'_, AppState>,
    project_id: String,
    format: String,
    output_path: String,
) -> AppResult<Vec<ExportResult>> {
    let db = state.db().await?;
    let chapters = crate::db::models::chapter::Chapter::list_by_project(&db, &project_id).await?;
    let project = crate::db::models::project::Project::find_by_id(&db, &project_id).await?;

    let mut results = Vec::new();
    for chapter in &chapters {
        let content = match format.as_str() {
            "txt" => format!("{}\n\n{}", chapter.title, chapter.content),
            "md" => format!("# {}\n\n{}", chapter.title, chapter.content),
            _ => continue,
        };

        let file_name = format!("{}_{}.{}", project.name, chapter.title, format);
        let file_path = std::path::Path::new(&output_path).join(&file_name);
        std::fs::write(&file_path, &content)?;

        let file_size = std::fs::metadata(&file_path)?.len();
        results.push(ExportResult {
            file_path: file_path.to_string_lossy().to_string(),
            file_size,
        });
    }

    Ok(results)
}

#[tauri::command]
pub async fn export_docx(
    state: State<'_, AppState>,
    chapter_id: String,
    output_path: String,
) -> AppResult<()> {
    let db = state.db().await?;

    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;

    crate::services::export_docx::export_docx(&chapter.content, &output_path, &chapter.title)?;

    Ok(())
}

#[tauri::command]
pub async fn export_pdf(
    state: State<'_, AppState>,
    chapter_id: String,
    output_path: String,
) -> AppResult<()> {
    let db = state.db().await?;

    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;

    crate::services::export_pdf::export_pdf(&chapter.content, &output_path, &chapter.title)?;

    Ok(())
}

#[tauri::command]
pub async fn export_epub(
    state: State<'_, AppState>,
    chapter_id: String,
    output_path: String,
    author: Option<String>,
) -> AppResult<()> {
    let db = state.db().await?;

    let chapter = crate::db::models::chapter::Chapter::find_by_id(&db, &chapter_id).await?;

    let author_str = author.unwrap_or_default();

    crate::services::export_epub::export_epub(&chapter.content, &output_path, &chapter.title, &author_str)?;

    Ok(())
}
