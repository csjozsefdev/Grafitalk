use crate::template::{
    parse_graf_id_json_import, parse_pretty_print, pretty_print_to_context_text, PrettyPrintContext,
};

use super::error::ImportError;
use super::format::ImportFormat;
use super::MAX_IMPORT_BYTES;

pub fn parse_import(content: &str, format: ImportFormat) -> Result<PrettyPrintContext, ImportError> {
    validate_size(content)?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(ImportError::EmptyFile);
    }

    let context = match format {
        ImportFormat::Json => {
            parse_graf_id_json_import(trimmed)
                .map_err(|err| ImportError::InvalidHandoff(err.to_string()))?
        }
        ImportFormat::PlainText => parse_pretty_print(trimmed),
        ImportFormat::Markdown => parse_pretty_print(&normalize_markdown_to_labeled_text(trimmed)),
    };

    if context.is_empty() {
        return Err(ImportError::EmptyContent);
    }

    Ok(context)
}

pub fn import_to_context_text(context: &PrettyPrintContext) -> Result<String, ImportError> {
    if context.is_empty() {
        return Err(ImportError::EmptyContent);
    }

    Ok(pretty_print_to_context_text(context))
}

fn validate_size(content: &str) -> Result<(), ImportError> {
    if content.len() > MAX_IMPORT_BYTES {
        return Err(ImportError::FileTooLarge {
            max_bytes: MAX_IMPORT_BYTES,
        });
    }

    Ok(())
}

fn normalize_markdown_to_labeled_text(markdown: &str) -> String {
    let mut lines = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }

        if let Some(label) = trimmed
            .strip_prefix("### ")
            .or_else(|| trimmed.strip_prefix("## "))
            .or_else(|| trimmed.strip_prefix("# "))
        {
            lines.push(format!("{}:", label.trim()));
            continue;
        }

        if trimmed.starts_with("**") && trimmed.ends_with("**") && trimmed.len() > 4 {
            let label = trimmed.trim_matches('*').trim();
            lines.push(format!("{}:", label));
            continue;
        }

        lines.push(trimmed.to_string());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::fixtures::{GRAF_ID_LABELED_TEXT, GRAF_ID_SAMPLE_JSON};

    #[test]
    fn parses_json_import() {
        let context = parse_import(GRAF_ID_SAMPLE_JSON, ImportFormat::Json).expect("parse json");
        assert!(context.current_status.is_some());
        assert!(!context.completed_work.is_empty());
    }

    #[test]
    fn parses_plain_text_import() {
        let context =
            parse_import(GRAF_ID_LABELED_TEXT, ImportFormat::PlainText).expect("parse text");
        assert!(context.current_status.is_some());
        assert!(!context.blockers.is_empty());
    }

    #[test]
    fn parses_markdown_import() {
        let markdown = "\
# Project\n\
Mesencsi webshop\n\
\n\
## Current status\n\
Deployment is nearly ready.\n\
\n\
## What changed\n\
- Manual QA completed.\n\
- Barion sandbox flow tested.\n\
\n\
## Current blocker\n\
- Barion production key / merchant approval.\n\
\n\
## Next step\n\
- Deploy once Barion credentials are available.\n\
\n\
## Estimated time\n\
Approximately 1 day after credentials are received.";

        let context = parse_import(markdown, ImportFormat::Markdown).expect("parse md");
        assert_eq!(
            context.current_status.as_deref(),
            Some("Deployment is nearly ready.")
        );
        assert_eq!(context.completed_work.len(), 2);
        assert_eq!(context.blockers.len(), 1);
    }

    #[test]
    fn rejects_empty_file() {
        let err = parse_import("   ", ImportFormat::PlainText).expect_err("empty");
        assert_eq!(err, ImportError::EmptyFile);
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "9.9",
            "project_name": "Example"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("unsupported");
        assert!(matches!(err, ImportError::InvalidHandoff(_)));
    }

    #[test]
    fn rejects_oversized_file() {
        let oversized = "x".repeat(MAX_IMPORT_BYTES + 1);
        let err = parse_import(&oversized, ImportFormat::PlainText).expect_err("oversized");
        assert!(matches!(err, ImportError::FileTooLarge { .. }));
    }

    #[test]
    fn equivalent_formats_produce_equivalent_context() {
        let from_json = parse_import(GRAF_ID_SAMPLE_JSON, ImportFormat::Json).expect("json");
        let from_text =
            parse_import(GRAF_ID_LABELED_TEXT, ImportFormat::PlainText).expect("text");

        assert_eq!(from_json.current_status, from_text.current_status);
        assert_eq!(from_json.completed_work, from_text.completed_work);
        assert_eq!(from_json.blockers, from_text.blockers);
        assert_eq!(from_json.next_steps, from_text.next_steps);
        assert_eq!(from_json.timeline, from_text.timeline);
    }

    #[test]
    fn import_accepts_completed_work_alias() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "0.1",
            "project_name": "Mesencsi webshop",
            "current_status": "Deployment is nearly ready.",
            "completed_work": ["Manual QA completed."]
        }"#;
        let context = parse_import(json, ImportFormat::Json).expect("parse alias json");
        assert_eq!(context.completed_work, vec!["Manual QA completed.".to_string()]);
    }

    #[test]
    fn rejects_invalid_source() {
        let json = r#"{
            "source": "other-tool",
            "schema_version": "0.1",
            "project_name": "Example"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("invalid source");
        assert!(matches!(err, ImportError::InvalidHandoff(_)));
    }

    #[test]
    fn import_accepts_spec_version_legacy_alias() {
        let json = r#"{
            "source": "graf-id",
            "spec_version": "0.1",
            "project_name": "Mesencsi webshop",
            "current_status": "Deployment is nearly ready.",
            "changes": ["Manual QA completed."]
        }"#;
        let context = parse_import(json, ImportFormat::Json).expect("parse spec_version alias");
        assert_eq!(context.project_focus.as_deref(), Some("Mesencsi webshop"));
        assert_eq!(context.completed_work, vec!["Manual QA completed.".to_string()]);
    }

    #[test]
    fn import_rejects_conflicting_version_fields() {
        let json = r#"{
            "source": "graf-id",
            "schema_version": "0.1",
            "spec_version": "0.2",
            "project_name": "Example"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("conflicting versions");
        assert!(matches!(err, ImportError::InvalidHandoff(message) if message.contains("conflicting version")));
    }

    #[test]
    fn import_rejects_missing_version_field() {
        let json = r#"{
            "source": "graf-id",
            "project_name": "Example"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("missing version");
        assert!(matches!(err, ImportError::InvalidHandoff(message) if message.contains("missing a version field")));
    }

    #[test]
    fn import_accepts_legacy_json_missing_source() {
        let json = r#"{
            "schema_version": "0.1",
            "project_name": "Mesencsi webshop",
            "current_status": "Deployment is nearly ready.",
            "changes": ["Manual QA completed."]
        }"#;
        let context = parse_import(json, ImportFormat::Json).expect("parse legacy missing source");
        assert_eq!(context.project_focus.as_deref(), Some("Mesencsi webshop"));
        assert_eq!(context.completed_work, vec!["Manual QA completed.".to_string()]);
    }

    #[test]
    fn import_rejects_missing_source_without_known_fields() {
        let json = r#"{
            "schema_version": "0.1",
            "exported_at": "2026-06-08T20:44:53Z"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("missing source and fields");
        assert!(matches!(err, ImportError::InvalidHandoff(message) if message.contains("missing required field source")));
    }

    #[test]
    fn import_accepts_full_graf_id_project_export() {
        let json = include_str!("../../../examples/graf_id_full_project_export.sample.json");
        let context = parse_import(json, ImportFormat::Json).expect("import full export");
        assert_eq!(context.project_focus.as_deref(), Some("Fejlesztesi naplo"));
        assert!(context.current_status.is_some());
        assert!(context.files.contains(&"README.md".to_string()));
    }

    #[test]
    fn import_rejects_full_export_without_useful_summary() {
        let json = r#"{
            "spec_version": 1,
            "project": { "name": "Example", "path": "C:\\secret\\path" },
            "resume_panel": {}
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("empty full export");
        assert!(matches!(err, ImportError::InvalidHandoff(message) if message.contains("does not contain enough resume summary")));
    }

    #[test]
    fn import_rejects_random_source_value() {
        let json = r#"{
            "source": "random-app",
            "schema_version": "0.1",
            "project_name": "Example"
        }"#;
        let err = parse_import(json, ImportFormat::Json).expect_err("invalid source");
        assert!(matches!(err, ImportError::InvalidHandoff(message) if message.contains("Invalid Graf-ID handoff source")));
    }

    #[test]
    fn import_to_context_text_omits_empty_sections() {
        let context = parse_import(GRAF_ID_SAMPLE_JSON, ImportFormat::Json).expect("parse");
        let text = import_to_context_text(&context).expect("canonical text");
        assert!(text.contains("Current status:"));
        assert!(text.contains("Project:\nMesencsi webshop"));
        assert!(!text.contains("Files updated:"));
    }
}
