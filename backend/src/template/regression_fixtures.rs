//! Regression tests tied to shared fixtures (PR 0 baseline + PR 1 pipeline).

#[cfg(test)]
mod tests {
    use crate::model::Project;
    use crate::template::fixtures::{
        COMPACT_DEPLOYMENT_HU, FEJLESZTESI_NAPLO, GRAF_ID_LABELED_TEXT, NARRATIVE_WITH_FILES,
        UNSTRUCTURED_PROSE,
    };
    use crate::template::{parse_pretty_print, render_status_update};

    fn sample_project(name: &str) -> Project {
        Project {
            id: "fixture-project".to_string(),
            name: name.to_string(),
            client_label: None,
            status: "active".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            last_used_at: None,
            archived_at: None,
        }
    }

    #[test]
    fn fixture_unstructured_prose_renders_summary() {
        let draft = render_status_update(&sample_project("Mesencsi"), UNSTRUCTURED_PROSE);
        assert!(draft.contains("Summary:\nQA handoff complete."));
    }

    #[test]
    fn fixture_narrative_with_files_renders_files_section() {
        let draft = render_status_update(&sample_project("Fejlődés útján"), NARRATIVE_WITH_FILES);
        assert!(draft.contains("Project focus:\nMobile app for home developmental routines."));
        assert!(draft.contains("* app/(tabs)/index.tsx"));
        assert!(!draft.contains("should be reviewed before sending"));
    }

    #[test]
    fn fixture_compact_deployment_note_renders_structured_sections() {
        let draft = render_status_update(&sample_project("Mesencsi"), COMPACT_DEPLOYMENT_HU);
        assert!(draft.contains("The webshop is ready for deployment"));
        assert!(draft.contains("Current blocker:"));
    }

    #[test]
    fn fixture_graf_id_labeled_text_parses_all_sections() {
        let context = parse_pretty_print(GRAF_ID_LABELED_TEXT);
        assert!(context.current_status.is_some());
        assert!(!context.completed_work.is_empty());
        assert!(!context.blockers.is_empty());
        assert!(!context.next_steps.is_empty());
        assert!(context.raw_fallback.is_none());
    }

    #[test]
    fn fixture_graf_id_labeled_text_draft_uses_structured_sections() {
        let draft = render_status_update(&sample_project("Mesencsi webshop"), GRAF_ID_LABELED_TEXT);
        assert!(!draft.contains("Focus Area:"));
        assert!(draft.contains("Project focus:\nMesencsi webshop"));
        assert!(draft.contains("Completed work:"));
        assert!(draft.contains("Barion sandbox flow tested."));
        assert!(!draft.contains("Review Notes:"));
    }

    #[test]
    fn fixture_fejlesztesi_naplo_separates_files_from_prose() {
        let context = parse_pretty_print(FEJLESZTESI_NAPLO);
        assert_eq!(context.project_focus.as_deref(), Some("Mesencsi webshop"));
        assert_eq!(context.completed_work.len(), 3);
        assert_eq!(context.files.len(), 2);
        assert!(context
            .completed_work
            .iter()
            .any(|item| item.contains("Checkout flow refaktorálva.")));

        let draft = render_status_update(&sample_project("Mesencsi webshop"), FEJLESZTESI_NAPLO);
        assert!(draft.contains("Project focus:"));
        assert!(draft.contains("Files Updated:"));
        assert!(draft.contains("* src/components/Checkout.tsx"));
        assert!(draft.contains("Current blocker:"));
        assert!(draft.contains("* Barion production key / merchant approval."));
    }
}
