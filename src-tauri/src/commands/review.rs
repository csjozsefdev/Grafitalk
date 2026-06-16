use backend::model::ProjectReviewMetadata;
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_project_review_metadata(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<ProjectReviewMetadata, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .get_review_metadata(&project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn migrate_last_generated_at_if_empty(
    state: State<'_, AppState>,
    project_id: String,
    legacy_timestamp: String,
) -> Result<bool, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .migrate_last_generated_at_if_empty(&project_id, &legacy_timestamp)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_draft_export(
    state: State<'_, AppState>,
    project_id: String,
    export_format: String,
) -> Result<(), String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .record_draft_export(&project_id, &export_format)
        .map_err(|e| e.to_string())
}
