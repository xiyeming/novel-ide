use crate::db::models::project::{CreateProjectRequest, Project, UpdateProjectRequest};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    name: String,
    path: String,
    genre: Option<String>,
    sub_genre: Option<String>,
    target_readers: Option<String>,
    total_chapters: Option<i32>,
    words_per_chapter: Option<i32>,
    narrative_pov: Option<String>,
    story_structure: Option<String>,
) -> AppResult<Project> {
    let db = state.db().await?;

    // Debug: log received parameters
    eprintln!("DEBUG: create_project called with:");
    eprintln!("  name: {:?}", name);
    eprintln!("  path: {:?}", path);
    eprintln!("  genre: {:?}", genre);
    eprintln!("  narrative_pov: {:?}", narrative_pov);
    eprintln!("  total_chapters: {:?}", total_chapters);
    eprintln!("  words_per_chapter: {:?}", words_per_chapter);
    eprintln!("  story_structure: {:?}", story_structure);

    // Validate project name
    if name.is_empty() || name.len() > 50 {
        return Err(crate::error::AppError::InvalidArgument(
            "项目名称必须在 1-50 字之间".into(),
        ));
    }

    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(crate::error::AppError::InvalidArgument(
            "项目名称包含非法字符".into(),
        ));
    }

    // Create project directory
    let project_dir = std::path::Path::new(&path).join(&name);
    
    // Check if project already exists
    if project_dir.exists() {
        return Err(crate::error::AppError::InvalidArgument(
            format!("项目「{}」已存在", name),
        ));
    }
    
    std::fs::create_dir_all(&project_dir)?;

    // Create subdirectories
    for dir in &["chapters", "drafts", "final", "assets", "prompts", "hooks", "skills", "references", "rag", "export", "logs"] {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }

    let req = CreateProjectRequest {
        name,
        path: project_dir.to_string_lossy().to_string(),
        genre,
        sub_genre,
        target_readers,
        total_chapters,
        words_per_chapter,
        narrative_pov,
        story_structure,
    };

    let project = Project::create(&db, &req).await?;
    Ok(project)
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> AppResult<Vec<Project>> {
    let db = state.db().await?;
    let projects = Project::list_all(&db).await?;
    Ok(projects)
}

#[tauri::command]
pub async fn open_project(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Project> {
    let db = state.db().await?;
    let project = Project::find_by_id(&db, &project_id).await?;
    Ok(project)
}

#[tauri::command]
pub async fn delete_project(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<()> {
    let db = state.db().await?;

    // Get project path before deletion
    let project = Project::find_by_id(&db, &project_id).await?;
    let project_path = project.path.clone();

    // Delete from database first
    Project::delete(&db, &project_id).await?;

    // Delete project directory
    let path = std::path::Path::new(&project_path);
    if path.exists() {
        match std::fs::remove_dir_all(path) {
            Ok(_) => {
                eprintln!("Successfully deleted project directory: {}", project_path);
            }
            Err(e) => {
                eprintln!("Failed to delete project directory {}: {}", project_path, e);
                // Don't fail the operation, database record is already deleted
            }
        }
    } else {
        eprintln!("Project directory does not exist: {}", project_path);
    }

    Ok(())
}

#[tauri::command]
pub async fn update_project(
    state: State<'_, AppState>,
    project_id: String,
    genre: Option<String>,
    sub_genre: Option<String>,
    target_readers: Option<String>,
    total_chapters: Option<i32>,
    words_per_chapter: Option<i32>,
    narrative_pov: Option<String>,
    story_structure: Option<String>,
    core_outline: Option<String>,
    world_settings: Option<String>,
    character_profiles: Option<String>,
    golden_finger: Option<String>,
    writing_constraints: Option<String>,
    style_constraints: Option<String>,
) -> AppResult<Project> {
    let db = state.db().await?;
    let req = UpdateProjectRequest {
        genre,
        sub_genre,
        target_readers,
        total_chapters,
        words_per_chapter,
        narrative_pov,
        story_structure,
        core_outline,
        world_settings,
        character_profiles,
        golden_finger,
        writing_constraints,
        style_constraints,
    };
    let project = Project::update(&db, &project_id, &req).await?;
    Ok(project)
}
