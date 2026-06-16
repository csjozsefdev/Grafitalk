use serde_json::{Map, Value};

use super::graf_id_handoff::HandoffParseError;
use super::pretty_print::{looks_like_file_path, PrettyPrintContext};

pub const FULL_PROJECT_EXPORT_WARNING: &str =
    "Legacy Graf-Id full project export; normalized to GrafiTalk context. Prefer v0.1 handoff JSON export.";

fn non_empty_string(value: &Value) -> Option<&str> {
    value.as_str().map(str::trim).filter(|text| !text.is_empty())
}

fn contains_internal_path(text: &str) -> bool {
    text.contains(":\\") || text.contains(":/") || text.contains("\\\\")
}

fn is_internal_metadata_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }

    let lower = trimmed.to_ascii_lowercase();
    lower.starts_with("active session since ")
        || lower.starts_with("confidence:")
        || lower.starts_with("category:")
        || contains_internal_path(trimmed)
}

fn strip_known_label(text: &str) -> String {
    for prefix in [
        "Where you left off:",
        "Project focus:",
        "README excerpt:",
    ] {
        if let Some(rest) = text.strip_prefix(prefix) {
            return rest.trim().to_string();
        }
    }
    text.trim().to_string()
}

fn dedupe_strings(values: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    let mut ordered = Vec::new();
    for value in values {
        let cleaned = value.trim();
        if cleaned.is_empty() || !seen.insert(cleaned.to_string()) {
            continue;
        }
        ordered.push(cleaned.to_string());
    }
    ordered
}

fn string_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.as_str().map(str::trim).filter(|text| !text.is_empty()))
        .map(str::to_string)
        .collect()
}

fn first_useful_summary_line(text: &str) -> Option<String> {
    for line in text.lines() {
        let cleaned = strip_known_label(line);
        if cleaned.is_empty() || is_internal_metadata_line(&cleaned) {
            continue;
        }
        if looks_like_file_path(&cleaned) {
            continue;
        }
        return Some(cleaned);
    }
    None
}

fn sanitize_summary_text(text: &str) -> Option<String> {
    let lines: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !is_internal_metadata_line(line))
        .map(strip_known_label)
        .filter(|line| !line.is_empty())
        .collect();

    let joined = dedupe_strings(lines).join("\n");
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

fn join_attributed_lines(lines: Option<&Value>) -> Option<String> {
    let items: Vec<String> = lines
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("text").and_then(non_empty_string))
        .map(strip_known_label)
        .filter(|text| !text.is_empty() && !is_internal_metadata_line(text))
        .collect();

    let joined = dedupe_strings(items).join("\n");
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

fn pick_current_status(
    summary_preview: Option<&Map<String, Value>>,
    startup_summary: Option<&Map<String, Value>>,
) -> Option<String> {
    if let Some(headline) = summary_preview
        .and_then(|summary| summary.get("headline"))
        .and_then(non_empty_string)
    {
        return Some(headline.to_string());
    }

    if let Some(headline) = startup_summary
        .and_then(|summary| summary.get("headline"))
        .and_then(non_empty_string)
    {
        return Some(headline.to_string());
    }

    summary_preview
        .and_then(|summary| summary.get("summary_text"))
        .and_then(non_empty_string)
        .and_then(|text| first_useful_summary_line(text))
        .or_else(|| {
            startup_summary
                .and_then(|summary| summary.get("summary_text"))
                .and_then(non_empty_string)
                .and_then(|text| first_useful_summary_line(text))
        })
}

fn pick_notes(
    summary_preview: Option<&Map<String, Value>>,
    startup_summary: Option<&Map<String, Value>>,
    resume_panel: Option<&Map<String, Value>>,
) -> Option<String> {
    if let Some(notes) = summary_preview
        .and_then(|summary| summary.get("summary_text"))
        .and_then(non_empty_string)
        .and_then(sanitize_summary_text)
    {
        return Some(notes);
    }

    if let Some(notes) = startup_summary
        .and_then(|summary| summary.get("summary_text"))
        .and_then(non_empty_string)
        .and_then(sanitize_summary_text)
    {
        return Some(notes);
    }

    join_attributed_lines(resume_panel.and_then(|panel| panel.get("attributed_lines"))).or_else(|| {
        join_attributed_lines(startup_summary.and_then(|summary| summary.get("attributed_lines")))
    })
}

fn collect_file_candidates(
    project: &Map<String, Value>,
    resume_panel: Option<&Map<String, Value>>,
    startup_summary: Option<&Map<String, Value>>,
    summary_preview: Option<&Map<String, Value>>,
) -> Vec<String> {
    let mut candidates = Vec::new();

    for container in [resume_panel, startup_summary] {
        if let Some(map) = container {
            candidates.extend(string_list(map.get("workflow_files")));
            candidates.extend(string_list(map.get("modified_files")));
        }
    }

    if let Some(map) = summary_preview {
        candidates.extend(string_list(map.get("workflow_files")));
        candidates.extend(string_list(map.get("modified_files")));
    }

    for text in [
        summary_preview.and_then(|summary| summary.get("summary_text")),
        startup_summary.and_then(|summary| summary.get("summary_text")),
        project
            .get("summary_preview")
            .and_then(|summary| summary.get("summary_text")),
    ] {
        if let Some(body) = text.and_then(non_empty_string) {
            for line in body.lines() {
                let cleaned = strip_known_label(line);
                if looks_like_file_path(&cleaned) {
                    candidates.push(cleaned);
                }
            }
        }
    }

    for lines in [
        resume_panel.and_then(|panel| panel.get("attributed_lines")),
        startup_summary.and_then(|summary| summary.get("attributed_lines")),
    ] {
        if let Some(items) = lines.and_then(Value::as_array) {
            for item in items {
                if let Some(source) = item.get("source").and_then(non_empty_string) {
                    if looks_like_file_path(source) || source.contains('.') {
                        candidates.push(source.to_string());
                    }
                }
                if let Some(text) = item.get("text").and_then(non_empty_string) {
                    if looks_like_file_path(text) {
                        candidates.push(strip_known_label(text));
                    }
                }
            }
        }
    }

    dedupe_strings(
        candidates
            .into_iter()
            .filter(|path| !contains_internal_path(path))
            .filter(|path| looks_like_file_path(path) || path.contains('.'))
            .collect(),
    )
}

fn latest_session<'a>(
    project: &'a Map<String, Value>,
    resume_panel: Option<&'a Map<String, Value>>,
) -> Option<&'a Map<String, Value>> {
    project
        .get("latest_session")
        .and_then(Value::as_object)
        .or_else(|| {
            resume_panel
                .and_then(|panel| panel.get("latest_session"))
                .and_then(Value::as_object)
        })
}

pub fn is_full_project_export(object: &Map<String, Value>) -> bool {
    let Some(project) = object.get("project").and_then(Value::as_object) else {
        return false;
    };

    project.get("name").is_some_and(|value| non_empty_string(value).is_some())
        || project.contains_key("summary_preview")
        || object
            .get("resume_panel")
            .and_then(|panel| panel.get("startup_summary"))
            .is_some()
        || project.contains_key("latest_session")
}

pub fn has_useful_full_export_content(object: &Map<String, Value>) -> bool {
    let Some(project) = object.get("project").and_then(Value::as_object) else {
        return false;
    };
    let resume_panel = object.get("resume_panel").and_then(Value::as_object);
    let summary_preview = project
        .get("summary_preview")
        .and_then(Value::as_object);
    let startup_summary = resume_panel
        .and_then(|panel| panel.get("startup_summary"))
        .and_then(Value::as_object);

    pick_current_status(summary_preview, startup_summary).is_some()
        || pick_notes(summary_preview, startup_summary, resume_panel).is_some()
        || latest_session(project, resume_panel)
            .and_then(|session| session.get("next_step"))
            .and_then(non_empty_string)
            .is_some()
        || latest_session(project, resume_panel)
            .and_then(|session| session.get("blocker"))
            .and_then(non_empty_string)
            .is_some()
        || resume_panel
            .and_then(|panel| panel.get("blocker"))
            .and_then(non_empty_string)
            .is_some()
}

pub fn parse_full_project_export(object: &Map<String, Value>) -> Result<PrettyPrintContext, HandoffParseError> {
    let project = object
        .get("project")
        .and_then(Value::as_object)
        .ok_or_else(|| HandoffParseError::InvalidJson("expected project object".to_string()))?;

    let resume_panel = object.get("resume_panel").and_then(Value::as_object);
    let summary_preview = project
        .get("summary_preview")
        .and_then(Value::as_object);
    let startup_summary = resume_panel
        .and_then(|panel| panel.get("startup_summary"))
        .and_then(Value::as_object);

    let project_name = project
        .get("name")
        .and_then(non_empty_string)
        .ok_or(HandoffParseError::MissingProjectName)?;

    if !has_useful_full_export_content(object) {
        return Err(HandoffParseError::FullExportMissingContent);
    }

    let current_status = pick_current_status(summary_preview, startup_summary);
    let notes = pick_notes(summary_preview, startup_summary, resume_panel);
    let session = latest_session(project, resume_panel);

    let next_steps = session
        .and_then(|value| value.get("next_step"))
        .and_then(non_empty_string)
        .map(|step| vec![step.to_string()])
        .unwrap_or_default();

    let blockers = session
        .and_then(|value| value.get("blocker"))
        .and_then(non_empty_string)
        .map(|blocker| vec![blocker.to_string()])
        .or_else(|| {
            resume_panel
                .and_then(|panel| panel.get("blocker"))
                .and_then(non_empty_string)
                .map(|blocker| vec![blocker.to_string()])
        })
        .unwrap_or_default();

    let files = collect_file_candidates(project, resume_panel, startup_summary, summary_preview);

    Ok(PrettyPrintContext {
        project_focus: Some(project_name.to_string()),
        current_status,
        completed_work: Vec::new(),
        blockers,
        next_steps,
        timeline: None,
        notes,
        files,
        raw_fallback: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Project;
    use crate::template::render_status_update;

    const FULL_EXPORT_SAMPLE: &str =
        include_str!("../../../examples/graf_id_full_project_export.sample.json");

    #[test]
    fn detects_full_project_export_shape() {
        let value: Value = serde_json::from_str(FULL_EXPORT_SAMPLE).expect("parse sample");
        let object = value.as_object().expect("object");
        assert!(is_full_project_export(object));
    }

    #[test]
    fn imports_full_export_with_summary_preview() {
        let value: Value = serde_json::from_str(FULL_EXPORT_SAMPLE).expect("parse sample");
        let object = value.as_object().expect("object");
        let context = parse_full_project_export(object).expect("import full export");

        assert_eq!(context.project_focus.as_deref(), Some("Fejlesztesi naplo"));
        assert!(context.current_status.is_some());
        assert!(context.notes.is_some());
        assert!(context.files.contains(&"README.md".to_string()));
        assert!(context.completed_work.is_empty());
    }

    #[test]
    fn imports_full_export_with_startup_summary_only() {
        let json = r#"{
            "spec_version": 1,
            "project": { "name": "Example" },
            "resume_panel": {
                "startup_summary": {
                    "headline": "Checkout refactor in progress.",
                    "summary_text": "Session still open (started today)."
                }
            }
        }"#;
        let value: Value = serde_json::from_str(json).expect("parse json");
        let context = parse_full_project_export(value.as_object().unwrap()).expect("import");
        assert_eq!(
            context.current_status.as_deref(),
            Some("Checkout refactor in progress.")
        );
    }

    #[test]
    fn rejects_full_export_without_useful_summary() {
        let json = r#"{
            "spec_version": 1,
            "project": { "name": "Example", "path": "C:\\secret\\path" },
            "resume_panel": {}
        }"#;
        let value: Value = serde_json::from_str(json).expect("parse json");
        let err = parse_full_project_export(value.as_object().unwrap()).expect_err("reject");
        assert!(matches!(err, HandoffParseError::FullExportMissingContent));
    }

    #[test]
    fn full_export_status_update_omits_internal_metadata() {
        let value: Value = serde_json::from_str(FULL_EXPORT_SAMPLE).expect("parse sample");
        let context = parse_full_project_export(value.as_object().unwrap()).expect("import");
        let text = crate::template::pretty_print_to_context_text(&context);
        let project = Project {
            id: "p1".to_string(),
            name: "Fejlesztesi naplo".to_string(),
            client_label: None,
            status: "active".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            last_used_at: None,
            archived_at: None,
        };
        let draft = render_status_update(&project, &text);

        assert!(!draft.contains("C:\\Projektek"));
        assert!(!draft.contains("Personal Projects"));
        assert!(!draft.contains("session_id"));
        assert!(draft.contains("Fejlesztesi naplo"));
    }
}
