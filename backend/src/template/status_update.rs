pub use super::render::render_status_update;

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

    #[test]
    fn transforms_compact_deployment_note() {
        let project = sample_project("Mesencsi", Some("Acme"));
        let draft = render_status_update(&project, COMPACT_DEPLOYMENT_HU);

        assert!(draft.contains("The webshop is ready for deployment"));
        assert!(draft.contains("Barion production key"));
        assert!(draft.contains("Current blocker:"));
    }

    #[test]
    fn structures_narrative_and_files() {
        let project = sample_project("Fejlődés útján", None);
        let draft = render_status_update(&project, NARRATIVE_WITH_FILES);

        assert!(draft.contains("Project:\nFejlődés útján"));
        assert!(draft.contains(
            "Project focus:\nMobile app for home developmental routines."
        ));
        assert!(draft.contains("* app/(tabs)/index.tsx"));
    }

    #[test]
    fn renders_labeled_context_without_single_focus_area() {
        let project = sample_project("Mesencsi webshop", None);
        let draft = render_status_update(&project, GRAF_ID_LABELED_TEXT);

        assert!(!draft.contains("Focus Area:"));
        assert!(draft.contains("Completed work:"));
    }

    #[test]
    fn renders_unstructured_prose_as_summary() {
        let project = sample_project("Mesencsi", None);
        let draft = render_status_update(&project, UNSTRUCTURED_PROSE);
        assert!(draft.contains("Summary:\nQA handoff complete."));
    }
}
