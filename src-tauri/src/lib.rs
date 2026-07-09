mod commands;
mod db;
mod error;
mod models;
mod services;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState::default())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db = db::init_database(&handle).await?;
                let state = app.state::<AppState>();
                state.set_db(db).await;
                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            commands::ai::chat_stream,
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::open_project,
            commands::project::delete_project,
            commands::project::update_project,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::chapter::create_chapter,
            commands::chapter::list_chapters,
            commands::chapter::get_chapter,
            commands::chapter::update_chapter,
            commands::chapter::delete_chapter,
            commands::search::search_chapters,
            commands::version::save_version,
            commands::version::list_versions,
            commands::version::restore_version,
            commands::model::create_provider,
            commands::model::list_providers,
            commands::model::update_provider,
            commands::model::delete_provider,
            commands::model::test_connection,
            commands::export::export_chapter,
            commands::export::export_all_chapters,
            commands::export::export_docx,
            commands::export::export_pdf,
            commands::export::export_epub,
            commands::knowledge::import_knowledge,
            commands::knowledge::list_knowledge,
            commands::knowledge::delete_knowledge,
            commands::knowledge::search_knowledge,
            commands::proofread::proofread_chapter,
            commands::workflow::create_workflow,
            commands::workflow::list_workflows,
            commands::workflow::delete_workflow,
            commands::workflow::execute_workflow,
            commands::workflow::get_workflow_execution,
            commands::workflow::run_workflow_stage,
            commands::agent::create_agent,
            commands::agent::list_agents,
            commands::agent::update_agent,
            commands::agent::delete_agent,
            commands::cloud::create_cloud_config,
            commands::cloud::list_cloud_configs,
            commands::cloud::delete_cloud_config,
            commands::cloud::upload_to_cloud,
            commands::cloud::download_from_cloud,
            commands::shortcuts::list_shortcuts,
            commands::shortcuts::update_shortcut,
            commands::shortcuts::toggle_shortcut,
            commands::shortcuts::generate_hyprland_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
