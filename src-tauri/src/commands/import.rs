use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn import_graf_id_handoff(
    state: State<'_, AppState>,
    project_id: String,
    content: String,
    format_hint: Option<String>,
    path_hint: Option<String>,
) -> Result<String, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .import_context_handoff(
            &project_id,
            &content,
            format_hint.as_deref(),
            path_hint.as_deref(),
        )
        .map_err(|e| e.to_string())
}
