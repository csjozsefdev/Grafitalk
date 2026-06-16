use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_project_context(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.get_context(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_project_context(
    state: State<'_, AppState>,
    project_id: String,
    context_text: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .save_context(&project_id, context_text)
        .map_err(|e| e.to_string())
}
