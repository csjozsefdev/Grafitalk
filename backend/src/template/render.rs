use crate::model::Project;

use super::pretty_print::{parse_pretty_print, PrettyPrintContext};
use super::templates::{render_empty, render_template, TemplateKind};

pub fn render_draft(
    kind: TemplateKind,
    project: &Project,
    context: &PrettyPrintContext,
) -> String {
    render_template(kind, project, context)
}

pub fn render_draft_from_context_text(
    kind: TemplateKind,
    project: &Project,
    context_text: &str,
) -> String {
    let trimmed = context_text.trim();
    if trimmed.is_empty() {
        return render_empty(kind, project);
    }

    let pretty = parse_pretty_print(trimmed);
    render_draft(kind, project, &pretty)
}

pub fn render_status_update(project: &Project, context_text: &str) -> String {
    render_draft_from_context_text(TemplateKind::StatusUpdate, project, context_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Project;
    use crate::template::fixtures::{
        COMPACT_DEPLOYMENT_HU, GRAF_ID_LABELED_TEXT, NARRATIVE_WITH_FILES, UNSTRUCTURED_PROSE,
    };

    fn sample_project(name: &str, client_label: Option<&str>) -> Project {
        Project {
            id: "p1".to_string(),
            name: name.to_string(),
            client_label: client_label.map(str::to_string),
            status: "active".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            last_used_at: None,
            archived_at: None,
        }
    }

    fn full_context() -> PrettyPrintContext {
        parse_pretty_print(GRAF_ID_LABELED_TEXT)
    }

    #[test]
    fn status_update_backward_compatible_with_legacy_entrypoint() {
        let project = sample_project("Mesencsi", Some("Acme"));
        let legacy = render_status_update(&project, COMPACT_DEPLOYMENT_HU);
        let routed = render_draft_from_context_text(
            TemplateKind::StatusUpdate,
            &project,
            COMPACT_DEPLOYMENT_HU,
        );
        assert_eq!(legacy, routed);
    }

    #[test]
    fn each_template_omits_empty_sections_for_full_context() {
        let project = sample_project("Mesencsi webshop", None);
        let context = full_context();

        for kind in TemplateKind::all() {
            let draft = render_draft(*kind, &project, &context);
            assert!(draft.starts_with(kind.title()));
            assert!(!draft.contains("Review Notes:"));
            assert!(!draft.contains("should be reviewed before sending"));
        }

        let status = render_draft(TemplateKind::StatusUpdate, &project, &context);
        assert!(status.contains("Current status:"));
        assert!(status.contains("Project focus:"));
        assert!(!status.contains("Files Updated:"));
    }

    #[test]
    fn template_section_emphasis_matches_plan() {
        let project = sample_project("Mesencsi webshop", None);
        let context = full_context();

        let status = render_draft(TemplateKind::StatusUpdate, &project, &context);
        assert!(status.contains("Current status:"));
        assert!(status.contains("Completed work:"));

        let client = render_draft(TemplateKind::ClientUpdate, &project, &context);
        assert!(client.contains("Where things stand:"));
        assert!(client.contains("Progress:"));
        assert!(client.contains("Next milestone:"));
        assert!(!client.contains("Current blocker:"));
        assert!(!client.contains("Investigation notes:"));

        let debug = render_draft(TemplateKind::DebugReport, &project, &context);
        assert!(debug.contains("Issue / blocker:"));
        assert!(debug.contains("Observed state:"));
        assert!(debug.contains("Changes attempted:"));
        assert!(debug.contains("Suggested fix:"));

        let handover = render_draft(TemplateKind::Handover, &project, &context);
        assert!(handover.contains("State at handover:"));
        assert!(handover.contains("Open items for next owner:"));
        assert!(handover.contains("Work completed:"));

        let weekly = render_draft(TemplateKind::WeeklySummary, &project, &context);
        assert!(weekly.contains("Week overview:"));
        assert!(weekly.contains("Highlights:"));
        assert!(weekly.contains("Priorities for next week:"));
    }

    #[test]
    fn each_template_produces_distinct_body_for_same_context() {
        let project = sample_project("Mesencsi webshop", Some("Acme"));
        let context = full_context();
        let drafts: Vec<String> = TemplateKind::all()
            .iter()
            .map(|kind| render_draft(*kind, &project, &context))
            .collect();

        for (index, left) in drafts.iter().enumerate() {
            for right in drafts.iter().skip(index + 1) {
                assert_ne!(
                    left, right,
                    "templates should not produce identical drafts"
                );
            }
        }
    }

    #[test]
    fn table_driven_context_shapes() {
        let project = sample_project("Unicode — β", Some("Acme"));
        let inputs = [
            UNSTRUCTURED_PROSE,
            NARRATIVE_WITH_FILES,
            "Current status:\nPartial only.",
        ];

        for kind in TemplateKind::all() {
            for input in inputs {
                let draft = render_draft_from_context_text(*kind, &project, input);
                assert!(
                    !draft.contains("should be reviewed before sending"),
                    "filler text in {} / {:?}",
                    kind.as_str(),
                    input
                );
                assert!(
                    !draft.contains("Review Notes:"),
                    "review notes in {} / {:?}",
                    kind.as_str(),
                    input
                );
            }
        }

        let status_draft =
            render_draft_from_context_text(TemplateKind::StatusUpdate, &project, UNSTRUCTURED_PROSE);
        assert!(status_draft.contains("Summary:"));

        let handover_draft = render_draft_from_context_text(
            TemplateKind::Handover,
            &project,
            NARRATIVE_WITH_FILES,
        );
        assert!(handover_draft.contains("* app/(tabs)/index.tsx"));
    }

    #[test]
    fn empty_context_renders_review_placeholder() {
        let project = sample_project("Empty", None);
        for kind in TemplateKind::all() {
            let draft = render_draft_from_context_text(*kind, &project, "   ");
            assert!(draft.contains("Add project context before generating a draft."));
            assert!(!draft.contains("should be reviewed before sending"));
            assert!(draft.starts_with(kind.title()));
        }
    }

    #[test]
    fn template_kind_parse_and_roundtrip() {
        for kind in TemplateKind::all() {
            let parsed = TemplateKind::parse(kind.as_str()).expect("parse");
            assert_eq!(parsed, *kind);
        }
        assert!(TemplateKind::parse("unknown").is_err());
    }
}
