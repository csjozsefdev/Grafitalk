mod commands;
mod state;

use std::sync::Mutex;

use backend::db::Database;
use backend::service::ProjectService;
use tauri::Manager;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let path = app.path().app_data_dir()?.join("grafitalk.db");
            let db = Database::open(&path).map_err(|e| e.to_string())?;
            let service = ProjectService::new(db);
            let app_version = app
                .config()
                .version
                .clone()
                .unwrap_or_else(|| "0.1.0".to_string());

            app.manage(AppState {
                project_service: Mutex::new(service),
                db_path: path,
                app_version,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::project::list_projects,
            commands::project::create_project,
            commands::project::touch_project_used,
            commands::project::archive_project,
            commands::project::list_archived_projects,
            commands::project::restore_project,
            commands::project::rename_project,
            commands::context::get_project_context,
            commands::context::save_project_context,
            commands::draft::get_project_draft,
            commands::draft::save_project_draft,
            commands::draft::generate_status_draft,
            commands::draft::generate_draft,
            commands::draft::get_project_template_kind,
            commands::draft::save_project_template_kind,
            commands::export::prepare_draft_export,
            commands::import::import_graf_id_handoff,
            commands::review::get_project_review_metadata,
            commands::review::migrate_last_generated_at_if_empty,
            commands::review::record_draft_export,
            commands::app::get_app_diagnostics,
            commands::app::export_database_backup,
            commands::app::open_data_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
