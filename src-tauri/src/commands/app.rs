use std::path::PathBuf;

use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_app_diagnostics(state: State<'_, AppState>) -> Result<backend::model::AppDiagnostics, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .diagnostics(
            &state.app_version,
            &state.db_path.display().to_string(),
            std::env::consts::OS,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_database_backup(
    state: State<'_, AppState>,
    destination_path: String,
) -> Result<(), String> {
    let destination = PathBuf::from(destination_path.trim());
    if destination.as_os_str().is_empty() {
        return Err("Backup destination path is required.".to_string());
    }

    if !state.db_path.is_file() {
        return Err("Database file was not found.".to_string());
    }

    if let Some(parent) = destination.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    std::fs::copy(&state.db_path, &destination).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn open_data_folder(state: State<'_, AppState>) -> Result<String, String> {
    let folder = state
        .db_path
        .parent()
        .map(|path| path.display().to_string())
        .ok_or_else(|| "Database folder is unavailable.".to_string())?;

    Ok(folder)
}
