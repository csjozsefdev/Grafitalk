use backend::export::PreparedExport;
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn prepare_draft_export(
    state: State<'_, AppState>,
    project_id: String,
    template_kind: String,
    draft_text: String,
    format_hint: Option<String>,
    path_hint: Option<String>,
) -> Result<PreparedExport, String> {
    let service = state
        .project_service
        .lock()
        .map_err(|_| "Application state lock poisoned".to_string())?;

    service
        .prepare_draft_export(
            &project_id,
            &template_kind,
            &draft_text,
            format_hint.as_deref(),
            path_hint.as_deref(),
        )
        .map_err(|e| e.to_string())
}
