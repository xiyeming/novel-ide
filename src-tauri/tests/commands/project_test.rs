use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tempfile::TempDir;

use novel_ide_lib::db::models::project::{CreateProjectRequest, Project, UpdateProjectRequest};

async fn setup_test_db() -> SqlitePool {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../../migrations/001_initial.sql"))
        .execute(&pool)
        .await
        .unwrap();

    pool
}

#[tokio::test]
async fn test_create_project() {
    let db = setup_test_db().await;
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project").to_string_lossy().to_string();

    let req = CreateProjectRequest {
        name: "测试项目".to_string(),
        path: project_path,
        genre: Some("玄幻".to_string()),
        sub_genre: None,
        target_readers: Some("男频".to_string()),
        total_chapters: Some(100),
        words_per_chapter: Some(3000),
        narrative_pov: Some("第三人称".to_string()),
        story_structure: Some("三幕式".to_string()),
    };

    let project = Project::create(&db, &req).await.unwrap();

    assert!(!project.id.is_empty());
    assert_eq!(project.name, "测试项目");
    assert_eq!(project.genre.as_deref(), Some("玄幻"));
    assert_eq!(project.target_readers.as_deref(), Some("男频"));
    assert_eq!(project.total_chapters, Some(100));
    assert_eq!(project.words_per_chapter, Some(3000));
}

#[tokio::test]
async fn test_list_projects() {
    let db = setup_test_db().await;
    let temp_dir = TempDir::new().unwrap();

    let req1 = CreateProjectRequest {
        name: "项目一".to_string(),
        path: temp_dir.path().join("p1").to_string_lossy().to_string(),
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };

    let req2 = CreateProjectRequest {
        name: "项目二".to_string(),
        path: temp_dir.path().join("p2").to_string_lossy().to_string(),
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };

    Project::create(&db, &req1).await.unwrap();
    Project::create(&db, &req2).await.unwrap();

    let projects = Project::list_all(&db).await.unwrap();
    assert_eq!(projects.len(), 2);
}

#[tokio::test]
async fn test_find_project_by_id() {
    let db = setup_test_db().await;
    let temp_dir = TempDir::new().unwrap();

    let req = CreateProjectRequest {
        name: "查找测试".to_string(),
        path: temp_dir.path().join("find-me").to_string_lossy().to_string(),
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };

    let created = Project::create(&db, &req).await.unwrap();
    let found = Project::find_by_id(&db, &created.id).await.unwrap();

    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "查找测试");
}

#[tokio::test]
async fn test_find_nonexistent_project() {
    let db = setup_test_db().await;
    let result = Project::find_by_id(&db, "nonexistent-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_project() {
    let db = setup_test_db().await;
    let temp_dir = TempDir::new().unwrap();

    let req = CreateProjectRequest {
        name: "删除测试".to_string(),
        path: temp_dir.path().join("delete-me").to_string_lossy().to_string(),
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };

    let created = Project::create(&db, &req).await.unwrap();
    Project::delete(&db, &created.id).await.unwrap();

    let result = Project::find_by_id(&db, &created.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_project() {
    let db = setup_test_db().await;
    let result = Project::delete(&db, "nonexistent-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_project() {
    let db = setup_test_db().await;
    let temp_dir = TempDir::new().unwrap();

    let req = CreateProjectRequest {
        name: "更新测试".to_string(),
        path: temp_dir.path().join("update-me").to_string_lossy().to_string(),
        genre: Some("都市".to_string()),
        sub_genre: None,
        target_readers: None,
        total_chapters: Some(50),
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
    };

    let created = Project::create(&db, &req).await.unwrap();

    let update_req = UpdateProjectRequest {
        genre: Some("仙侠".to_string()),
        sub_genre: Some("修真".to_string()),
        target_readers: Some("男频".to_string()),
        total_chapters: Some(200),
        words_per_chapter: Some(5000),
        narrative_pov: Some("第三人称".to_string()),
        story_structure: None,
        core_outline: Some("主线大纲".to_string()),
        world_settings: None,
        character_profiles: None,
        golden_finger: None,
        writing_constraints: None,
        style_constraints: None,
    };

    let updated = Project::update(&db, &created.id, &update_req).await.unwrap();

    assert_eq!(updated.genre.as_deref(), Some("仙侠"));
    assert_eq!(updated.sub_genre.as_deref(), Some("修真"));
    assert_eq!(updated.target_readers.as_deref(), Some("男频"));
    assert_eq!(updated.total_chapters, Some(200));
    assert_eq!(updated.words_per_chapter, Some(5000));
    assert_eq!(updated.narrative_pov.as_deref(), Some("第三人称"));
    assert_eq!(updated.core_outline.as_deref(), Some("主线大纲"));
}

#[tokio::test]
async fn test_update_nonexistent_project() {
    let db = setup_test_db().await;
    let update_req = UpdateProjectRequest {
        genre: None,
        sub_genre: None,
        target_readers: None,
        total_chapters: None,
        words_per_chapter: None,
        narrative_pov: None,
        story_structure: None,
        core_outline: None,
        world_settings: None,
        character_profiles: None,
        golden_finger: None,
        writing_constraints: None,
        style_constraints: None,
    };

    let result = Project::update(&db, "nonexistent-id", &update_req).await;
    assert!(result.is_err());
}
