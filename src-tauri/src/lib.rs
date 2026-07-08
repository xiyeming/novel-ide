mod commands;
mod db;
mod error;
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
