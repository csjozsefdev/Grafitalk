use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_project_draft(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service.get_draft(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_project_draft(
    state: State<'_, AppState>,
    project_id: String,
    draft_text: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .save_draft(&project_id, draft_text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_status_draft(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .generate_status_draft(&project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_draft(
    state: State<'_, AppState>,
    project_id: String,
    template_kind: String,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .generate_draft(&project_id, &template_kind)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project_template_kind(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .get_template_kind(&project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_project_template_kind(
    state: State<'_, AppState>,
    project_id: String,
    template_kind: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .save_template_kind(&project_id, &template_kind)
        .map_err(|e| e.to_string())
}
