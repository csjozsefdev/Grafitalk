use backend::model::Project;
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.list_active().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_project(
    state: State<'_, AppState>,
    name: String,
    client_label: Option<String>,
) -> Result<Project, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .create(name, client_label)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn touch_project_used(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .touch_last_used(&project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn archive_project(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.archive_project(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_archived_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.list_archived().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_project(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.restore_project(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_project(
    state: State<'_, AppState>,
    project_id: String,
    name: String,
) -> Result<Project, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .rename_project(&project_id, name)
        .map_err(|e| e.to_string())
}
