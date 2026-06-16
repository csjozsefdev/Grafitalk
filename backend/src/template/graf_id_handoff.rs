use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const SCHEMA_VERSION: &str = "0.1";
pub const SOURCE_GRAF_ID: &str = "graf-id";

const LEGACY_SPEC_VERSION_WARNING: &str =
    "Using legacy Graf-ID spec_version alias; prefer schema_version.";
const LEGACY_VERSION_WARNING: &str = "Using legacy Graf-ID version alias; prefer schema_version.";
const LEGACY_MISSING_SOURCE_WARNING: &str =
    "Legacy Graf-Id handoff missing source; normalized to graf-id.";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GrafIdHandoff {
    pub source: String,
    pub schema_version: String,
    pub project_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(default, alias = "completed_work")]
    pub changes: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub next_steps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffParseError {
    InvalidJson(String),
    MissingSchemaVersion,
    ConflictingSchemaVersion { details: String },
    UnsupportedSchemaVersion { found: String, expected: String },
    MissingSource,
    InvalidSource { found: String, expected: String },
    MissingProjectName,
    FullExportMissingContent,
}

impl std::fmt::Display for HandoffParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(message) => write!(f, "Invalid Graf-ID handoff JSON: {message}"),
            Self::MissingSchemaVersion => write!(
                f,
                "Graf-ID handoff JSON is missing a version field (expected schema_version, or legacy alias spec_version or version)"
            ),
            Self::ConflictingSchemaVersion { details } => write!(
                f,
                "Graf-ID handoff JSON has conflicting version fields: {details}"
            ),
            Self::UnsupportedSchemaVersion { found, expected } => write!(
                f,
                "Unsupported Graf-ID handoff schema version {found} (expected {expected})"
            ),
            Self::MissingSource => write!(
                f,
                "Graf-ID handoff JSON is missing required field source (expected \"{expected}\") and does not contain recognizable Graf-ID context fields",
                expected = SOURCE_GRAF_ID
            ),
            Self::InvalidSource { found, expected } => write!(
                f,
                "Invalid Graf-ID handoff source {found} (expected {expected})"
            ),
            Self::MissingProjectName => write!(f, "Graf-ID handoff is missing project_name"),
            Self::FullExportMissingContent => write!(
                f,
                "Graf-ID full project export does not contain enough resume summary content to import"
            ),
        }
    }
}

impl std::error::Error for HandoffParseError {}

fn normalize_version_value(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_u64().map(|value| value as i64))
            .map(|value| value.to_string()),
        _ => None,
    }
}

fn resolve_schema_version(object: &Map<String, Value>) -> Result<(String, Option<&'static str>), HandoffParseError> {
    let schema = object
        .get("schema_version")
        .and_then(normalize_version_value);
    let spec = object.get("spec_version").and_then(normalize_version_value);
    let version = object.get("version").and_then(normalize_version_value);

    let had_schema = schema.is_some();
    let had_spec = spec.is_some();
    let had_version = version.is_some();

    let mut present: Vec<(&str, String)> = Vec::new();
    if let Some(value) = schema {
        present.push(("schema_version", value));
    }
    if let Some(value) = spec {
        present.push(("spec_version", value));
    }
    if let Some(value) = version {
        present.push(("version", value));
    }

    if present.is_empty() {
        return Err(HandoffParseError::MissingSchemaVersion);
    }

    let normalized = present[0].1.clone();
    for (_field, value) in present.iter().skip(1) {
        if *value != normalized {
            return Err(HandoffParseError::ConflictingSchemaVersion {
                details: format!(
                    "schema_version={:?}, spec_version={:?}, version={:?}",
                    object.get("schema_version").and_then(normalize_version_value),
                    object.get("spec_version").and_then(normalize_version_value),
                    object.get("version").and_then(normalize_version_value),
                ),
            });
        }
    }

    let legacy_warning = if !had_schema && had_spec {
        Some(LEGACY_SPEC_VERSION_WARNING)
    } else if !had_schema && had_version {
        Some(LEGACY_VERSION_WARNING)
    } else {
        None
    };

    Ok((normalized, legacy_warning))
}

fn warn_legacy_compat(message: &str) {
    eprintln!("[grafitalk] {message}");
}

fn non_empty_string(value: &Value) -> Option<&str> {
    value.as_str().map(str::trim).filter(|text| !text.is_empty())
}

fn non_empty_string_array(value: &Value) -> bool {
    value.as_array().is_some_and(|items| {
        items
            .iter()
            .any(|item| item.as_str().is_some_and(|text| !text.trim().is_empty()))
    })
}

fn has_known_graf_id_context_fields(object: &Map<String, Value>) -> bool {
    for field in [
        "project_name",
        "current_status",
        "project_focus",
        "estimated_time",
        "timeline",
        "notes",
    ] {
        if object
            .get(field)
            .is_some_and(|value| non_empty_string(value).is_some())
        {
            return true;
        }
    }

    for field in [
        "changes",
        "completed_work",
        "blockers",
        "next_steps",
        "files",
    ] {
        if object
            .get(field)
            .is_some_and(|value| non_empty_string_array(value))
        {
            return true;
        }
    }

    false
}

fn apply_legacy_field_mapping(object: &mut Map<String, Value>) {
    if object.get("estimated_time").is_none_or(|value| non_empty_string(value).is_none()) {
        if let Some(timeline) = object.get("timeline").and_then(non_empty_string) {
            object.insert(
                "estimated_time".to_string(),
                Value::String(timeline.to_string()),
            );
        }
    }
    object.remove("timeline");

    let Some(focus) = object
        .get("project_focus")
        .and_then(non_empty_string)
        .map(str::to_string)
    else {
        object.remove("project_focus");
        return;
    };

    let project_name_empty = object
        .get("project_name")
        .is_none_or(|value| non_empty_string(value).is_none());
    if project_name_empty {
        let name = focus.lines().next().unwrap_or(&focus).trim();
        let truncated = if name.len() > 120 { &name[..120] } else { name };
        object.insert(
            "project_name".to_string(),
            Value::String(truncated.to_string()),
        );
    }

    let status_empty = object
        .get("current_status")
        .is_none_or(|value| non_empty_string(value).is_none());
    if status_empty {
        object.insert(
            "current_status".to_string(),
            Value::String(focus.clone()),
        );
    } else if object
        .get("notes")
        .is_none_or(|value| non_empty_string(value).is_none())
    {
        object.insert("notes".to_string(), Value::String(focus));
    } else if let Some(Value::String(notes)) = object.get_mut("notes") {
        notes.push('\n');
        notes.push_str(&focus);
    }

    object.remove("project_focus");
}

fn resolve_source(
    object: &mut Map<String, Value>,
    schema_version: &str,
) -> Result<Option<&'static str>, HandoffParseError> {
    match object.get("source").and_then(Value::as_str) {
        Some(source) if source.trim() != SOURCE_GRAF_ID => {
            Err(HandoffParseError::InvalidSource {
                found: source.to_string(),
                expected: SOURCE_GRAF_ID.to_string(),
            })
        }
        Some(_) => Ok(None),
        None => {
            if !has_known_graf_id_context_fields(object) {
                return Err(HandoffParseError::MissingSource);
            }

            if schema_version != SCHEMA_VERSION {
                return Err(HandoffParseError::UnsupportedSchemaVersion {
                    found: schema_version.to_string(),
                    expected: SCHEMA_VERSION.to_string(),
                });
            }

            object.insert(
                "source".to_string(),
                Value::String(SOURCE_GRAF_ID.to_string()),
            );
            Ok(Some(LEGACY_MISSING_SOURCE_WARNING))
        }
    }
}

fn is_handoff_shaped(object: &Map<String, Value>) -> bool {
    if object
        .get("source")
        .and_then(Value::as_str)
        .is_some_and(|source| source.trim() == SOURCE_GRAF_ID)
    {
        return true;
    }

    if object
        .get("project_name")
        .is_some_and(|value| non_empty_string(value).is_some())
    {
        return true;
    }

    object.get("source").is_none() && has_known_graf_id_context_fields(object)
}

pub fn parse_graf_id_json_import(
    json: &str,
) -> Result<crate::template::PrettyPrintContext, HandoffParseError> {
    use crate::template::graf_id_full_export::{
        is_full_project_export, parse_full_project_export, FULL_PROJECT_EXPORT_WARNING,
    };
    use crate::template::from_graf_id_handoff;

    let trimmed = json.trim().strip_prefix('\u{FEFF}').unwrap_or(json.trim());
    let value: Value = serde_json::from_str(trimmed)
        .map_err(|err| HandoffParseError::InvalidJson(err.to_string()))?;
    let object = value.as_object().ok_or_else(|| {
        HandoffParseError::InvalidJson("expected JSON object".to_string())
    })?;

    if is_handoff_shaped(object) {
        let handoff = parse_handoff_json(trimmed)?;
        return Ok(from_graf_id_handoff(&handoff));
    }

    if is_full_project_export(object) {
        warn_legacy_compat(FULL_PROJECT_EXPORT_WARNING);
        return Ok(parse_full_project_export(object)?);
    }

    if object.contains_key("project") {
        parse_full_project_export(object)
    } else {
        parse_handoff_json(trimmed).map(|handoff| from_graf_id_handoff(&handoff))
    }
}

pub fn parse_handoff_json(json: &str) -> Result<GrafIdHandoff, HandoffParseError> {
    let trimmed = json.trim().strip_prefix('\u{FEFF}').unwrap_or(json.trim());
    let mut value: Value = serde_json::from_str(trimmed)
        .map_err(|err| HandoffParseError::InvalidJson(err.to_string()))?;

    let object = value
        .as_object_mut()
        .ok_or_else(|| HandoffParseError::InvalidJson("expected JSON object".to_string()))?;

    let (schema_version, legacy_warning) = resolve_schema_version(object)?;
    object.insert(
        "schema_version".to_string(),
        Value::String(schema_version.clone()),
    );
    object.remove("spec_version");
    object.remove("version");

    if let Some(warning) = legacy_warning {
        warn_legacy_compat(warning);
    }

    let legacy_source_warning = resolve_source(object, &schema_version)?;
    if let Some(warning) = legacy_source_warning {
        warn_legacy_compat(warning);
    }

    apply_legacy_field_mapping(object);

    let handoff: GrafIdHandoff = serde_json::from_value(value)
        .map_err(|err| HandoffParseError::InvalidJson(err.to_string()))?;

    validate_handoff(&handoff)?;
    Ok(handoff)
}

pub fn validate_handoff(handoff: &GrafIdHandoff) -> Result<(), HandoffParseError> {
    if handoff.schema_version != SCHEMA_VERSION {
        return Err(HandoffParseError::UnsupportedSchemaVersion {
            found: handoff.schema_version.clone(),
            expected: SCHEMA_VERSION.to_string(),
        });
    }

    if handoff.source.trim() != SOURCE_GRAF_ID {
        return Err(HandoffParseError::InvalidSource {
            found: handoff.source.clone(),
            expected: SOURCE_GRAF_ID.to_string(),
        });
    }

    if handoff.project_name.trim().is_empty() {
        return Err(HandoffParseError::MissingProjectName);
    }

    Ok(())
}

/// Converts a Graf-ID handoff into plain text suitable for GrafiTalk context input.
/// This is a future integration adapter only; no live import is wired in the MVP UI.
pub fn handoff_to_context_text(handoff: &GrafIdHandoff) -> String {
    let mut sections = Vec::new();

    sections.push(format!("Project:\n{}", handoff.project_name.trim()));

    if let Some(status) = handoff
        .current_status
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Current status:\n{}", status.trim()));
    }

    if !handoff.changes.is_empty() {
        let mut block = String::from("What changed:\n");
        for item in &handoff.changes {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if !handoff.blockers.is_empty() {
        let mut block = String::from("Current blocker:\n");
        for item in &handoff.blockers {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if !handoff.next_steps.is_empty() {
        let mut block = String::from("Next step:\n");
        for item in &handoff.next_steps {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if let Some(estimate) = handoff
        .estimated_time
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Estimated time:\n{}", estimate.trim()));
    }

    if let Some(notes) = handoff
        .notes
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Notes:\n{}", notes.trim()));
    }

    sections.join("\n\n")
}

/// Reference plain-text serializer for the Graf-ID export contract (v0.1).
pub fn handoff_to_plain_text(handoff: &GrafIdHandoff) -> String {
    handoff_to_context_text(handoff)
}

/// Reference Markdown serializer for the Graf-ID export contract (v0.1).
pub fn handoff_to_markdown(handoff: &GrafIdHandoff) -> String {
    let mut sections = vec![format!("# Project\n{}", handoff.project_name.trim())];

    if let Some(status) = handoff
        .current_status
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("## Current status\n{}", status.trim()));
    }

    if !handoff.changes.is_empty() {
        let mut block = String::from("## What changed\n");
        for item in &handoff.changes {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if !handoff.blockers.is_empty() {
        let mut block = String::from("## Current blocker\n");
        for item in &handoff.blockers {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if !handoff.next_steps.is_empty() {
        let mut block = String::from("## Next step\n");
        for item in &handoff.next_steps {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                block.push_str("- ");
                block.push_str(trimmed);
                block.push('\n');
            }
        }
        sections.push(block.trim_end().to_string());
    }

    if let Some(estimate) = handoff
        .estimated_time
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("## Estimated time\n{}", estimate.trim()));
    }

    if let Some(notes) = handoff
        .notes
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("## Notes\n{}", notes.trim()));
    }

    sections.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_handoff_json() {
        let json = include_str!("../../../examples/graf_id_project_context.sample.json");
        let handoff = parse_handoff_json(json).expect("parse sample");
        assert_eq!(handoff.source, SOURCE_GRAF_ID);
        assert_eq!(handoff.project_name, "Mesencsi webshop");
        assert_eq!(handoff.changes.len(), 3);
    }

    #[test]
    fn converts_handoff_to_context_text() {
        let json = include_str!("../../../examples/graf_id_project_context.sample.json");
        let handoff = parse_handoff_json(json).expect("parse sample");
        let context = handoff_to_context_text(&handoff);

        assert!(context.contains("Project:\nMesencsi webshop"));
        assert!(context.contains("Current status:"));
        assert!(context.contains("What changed:"));
        assert!(context.contains("Barion sandbox flow tested."));
        assert!(context.contains("Current blocker:"));
        assert!(context.contains("Estimated time:"));
    }

    #[test]
    fn reference_serializers_match_sample_files() {
        fn normalize_newlines(value: &str) -> String {
            value.replace("\r\n", "\n")
        }

        let json = include_str!("../../../examples/graf_id_project_context.sample.json");
        let handoff = parse_handoff_json(json).expect("parse sample");
        let plain = handoff_to_plain_text(&handoff);
        let markdown = handoff_to_markdown(&handoff);
        let sample_txt =
            include_str!("../../../examples/graf_id_project_context.sample.txt");
        let sample_md =
            include_str!("../../../examples/graf_id_project_context.sample.md");

        assert_eq!(normalize_newlines(plain.trim()), normalize_newlines(sample_txt.trim()));
        assert_eq!(
            normalize_newlines(markdown.trim()),
            normalize_newlines(sample_md.trim())
        );
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "9.9",
            "project_name": "Example"
        }"#;
        let err = parse_handoff_json(json).expect_err("unsupported version");
        assert!(matches!(
            err,
            HandoffParseError::UnsupportedSchemaVersion { .. }
        ));
    }

    #[test]
    fn accepts_schema_version_field() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "0.1",
            "project_name": "Example"
        }"#;
        let handoff = parse_handoff_json(json).expect("parse schema_version");
        assert_eq!(handoff.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn accepts_spec_version_legacy_alias() {
        let json = r#"{
            "source": "graf-id",
            "spec_version": "0.1",
            "project_name": "Example"
        }"#;
        let handoff = parse_handoff_json(json).expect("parse spec_version alias");
        assert_eq!(handoff.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn accepts_matching_schema_and_spec_version() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "0.1",
            "spec_version": "0.1",
            "project_name": "Example"
        }"#;
        let handoff = parse_handoff_json(json).expect("parse matching versions");
        assert_eq!(handoff.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn rejects_conflicting_schema_and_spec_version() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "0.1",
            "spec_version": "0.2",
            "project_name": "Example"
        }"#;
        let err = parse_handoff_json(json).expect_err("conflicting versions");
        assert!(matches!(
            err,
            HandoffParseError::ConflictingSchemaVersion { .. }
        ));
    }

    #[test]
    fn rejects_missing_version_field() {
        let json = r#"{
            "source": "graf-id",
            "project_name": "Example"
        }"#;
        let err = parse_handoff_json(json).expect_err("missing version");
        assert!(matches!(err, HandoffParseError::MissingSchemaVersion));
    }

    #[test]
    fn accepts_missing_source_when_handoff_shaped() {
        let json = r#"{
            "schema_version": "0.1",
            "project_name": "Example",
            "current_status": "In progress."
        }"#;
        let handoff = parse_handoff_json(json).expect("parse legacy missing source");
        assert_eq!(handoff.source, SOURCE_GRAF_ID);
        assert_eq!(handoff.project_name, "Example");
    }

    #[test]
    fn accepts_spec_version_without_source_when_handoff_shaped() {
        let json = r#"{
            "spec_version": "0.1",
            "project_name": "Example",
            "changes": ["Updated checkout flow."]
        }"#;
        let handoff = parse_handoff_json(json).expect("parse spec_version without source");
        assert_eq!(handoff.source, SOURCE_GRAF_ID);
        assert_eq!(handoff.changes, vec!["Updated checkout flow."]);
    }

    #[test]
    fn rejects_missing_source_without_known_fields() {
        let json = r#"{
            "schema_version": "0.1",
            "exported_at": "2026-06-08T20:44:53Z"
        }"#;
        let err = parse_handoff_json(json).expect_err("missing source and context");
        assert!(matches!(err, HandoffParseError::MissingSource));
    }

    #[test]
    fn rejects_invalid_source_value() {
        let json = r#"{
            "source": "random-app",
            "schema_version": "0.1",
            "project_name": "Example"
        }"#;
        let err = parse_handoff_json(json).expect_err("invalid source");
        assert!(matches!(err, HandoffParseError::InvalidSource { .. }));
    }

    #[test]
    fn maps_project_focus_to_current_status_without_mixing_files() {
        let json = r#"{
            "spec_version": "0.1",
            "project_name": "Example project",
            "project_focus": "Checkout refactor in progress.",
            "files": ["src/App.tsx"]
        }"#;
        let handoff = parse_handoff_json(json).expect("parse project_focus legacy");
        let context = crate::template::from_graf_id_handoff(&handoff);

        assert_eq!(context.project_focus.as_deref(), Some("Example project"));
        assert_eq!(
            context.current_status.as_deref(),
            Some("Checkout refactor in progress.")
        );
        assert_eq!(context.files, vec!["src/App.tsx".to_string()]);
    }

    #[test]
    fn legacy_missing_source_produces_pretty_print_context() {
        let json = r#"{
            "schema_version": "0.1",
            "project_name": "Example",
            "current_status": "In progress.",
            "changes": ["Updated checkout flow."]
        }"#;
        let handoff = parse_handoff_json(json).expect("parse legacy handoff");
        let context = crate::template::from_graf_id_handoff(&handoff);

        assert_eq!(context.project_focus.as_deref(), Some("Example"));
        assert_eq!(context.current_status.as_deref(), Some("In progress."));
        assert_eq!(context.completed_work, vec!["Updated checkout flow.".to_string()]);
    }

    #[test]
    fn maps_files_without_mixing_project_focus() {
        let json = r#"{
            "source": "graf-id",
            "spec_version": "0.1",
            "project_name": "Example project",
            "current_status": "In progress.",
            "changes": ["Updated checkout flow."],
            "files": ["src/App.tsx", "src/checkout.rs"]
        }"#;
        let handoff = parse_handoff_json(json).expect("parse with files");
        let context = crate::template::from_graf_id_handoff(&handoff);

        assert_eq!(context.project_focus.as_deref(), Some("Example project"));
        assert_eq!(context.completed_work, vec!["Updated checkout flow.".to_string()]);
        assert_eq!(
            context.files,
            vec!["src/App.tsx".to_string(), "src/checkout.rs".to_string()]
        );
        assert!(context.current_status.is_some());
    }
}
