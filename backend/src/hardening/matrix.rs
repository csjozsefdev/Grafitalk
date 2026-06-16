use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::Database;
use crate::export::{ExportFormat, PreparedExport};
use crate::import::{parse_import, ImportFormat, MAX_IMPORT_BYTES};
use crate::service::ProjectService;
use crate::template::fixtures::{
    GRAF_ID_LABELED_TEXT, GRAF_ID_SAMPLE_JSON, NARRATIVE_WITH_FILES, UNSTRUCTURED_PROSE,
};
use crate::template::{render_draft_from_context_text, TemplateKind};

fn temp_db_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!("grafitalk_hardening_{label}_{nanos}.db"))
}

fn service_at(path: &PathBuf) -> ProjectService {
    let db = Database::open(path).expect("open");
    ProjectService::new(db)
}

fn sample_project(name: &str) -> crate::model::Project {
    crate::model::Project {
        id: "fixture".to_string(),
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
fn import_matrix_valid_json_txt_md_are_equivalent() {
    let json = parse_import(GRAF_ID_SAMPLE_JSON, ImportFormat::Json).expect("json");
    let text = parse_import(GRAF_ID_LABELED_TEXT, ImportFormat::PlainText).expect("text");
    let md = parse_import(
        include_str!("../../../examples/graf_id_project_context.sample.md"),
        ImportFormat::Markdown,
    )
    .expect("md");

    assert_eq!(json.current_status, text.current_status);
    assert_eq!(json.completed_work, text.completed_work);
    assert_eq!(json.blockers, md.blockers);
}

#[test]
fn import_matrix_rejects_invalid_json_empty_and_oversized() {
    assert!(parse_import("not-json", ImportFormat::Json).is_err());
    assert!(parse_import("   ", ImportFormat::PlainText).is_err());

    let oversized = "x".repeat(MAX_IMPORT_BYTES + 1);
    assert!(parse_import(&oversized, ImportFormat::PlainText).is_err());

    let unsupported = r#"{"source":"graf-id","schema_version":"9.9","project_name":"X"}"#;
    assert!(parse_import(unsupported, ImportFormat::Json).is_err());
}

#[test]
fn template_matrix_all_kinds_render_without_filler() {
    let project = sample_project("Unicode — β");
    let contexts = [
        UNSTRUCTURED_PROSE,
        NARRATIVE_WITH_FILES,
        GRAF_ID_LABELED_TEXT,
        "Current status:\nPartial.\n\nNext step:\n- Finish.",
    ];

    for kind in TemplateKind::all() {
        for context in contexts {
            let draft = render_draft_from_context_text(*kind, &project, context);
            assert!(
                !draft.contains("Review Notes:"),
                "{} contains review notes for {:?}",
                kind.as_str(),
                context
            );
            assert!(
                !draft.contains("should be reviewed before sending"),
                "{} contains filler for {:?}",
                kind.as_str(),
                context
            );
            assert!(
                draft.starts_with(kind.title()),
                "{} wrong heading",
                kind.as_str()
            );
        }
    }
}

#[test]
fn export_matrix_all_formats_support_unicode_and_long_lines() {
    let project = sample_project("Export Matrix");
    let body = format!(
        "Status update\n\nNotes:\nUnicode — β\n\n{}",
        "Long export line ".repeat(50)
    );

    for format in [
        ExportFormat::Txt,
        ExportFormat::Md,
        ExportFormat::Json,
        ExportFormat::Pdf,
    ] {
        let prepared: PreparedExport = crate::export::prepare_export(
            &project,
            "status_update",
            &body,
            format,
        )
        .expect("prepare");

        match format {
            ExportFormat::Pdf => {
                assert!(prepared.bytes.unwrap().starts_with(b"%PDF"));
            }
            _ => assert!(prepared.text.unwrap().contains("Unicode — β")),
        }
    }
}

#[test]
fn export_matrix_rejects_empty_draft() {
    let project = sample_project("Empty Export");
    let err = crate::export::prepare_export(
        &project,
        "status_update",
        "   ",
        ExportFormat::Txt,
    )
    .expect_err("empty");
    assert!(err.to_string().contains("Nothing to export yet."));
}

#[test]
fn workflow_import_generate_export_persists_after_reopen() {
    let path = temp_db_path("workflow");
    let _ = fs::remove_file(&path);

    let project_id = {
        let service = service_at(&path);
        let project = service.create("Workflow Project", None).expect("create");
        service
            .import_context_handoff(&project.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
            .expect("import");
        service
            .generate_draft(&project.id, "debug_report")
            .expect("generate");
        service
            .record_draft_export(&project.id, "json")
            .expect("export");
        project.id
    };

    let service = service_at(&path);
    assert!(service
        .get_context(&project_id)
        .expect("context")
        .contains("Current status:"));
    assert!(service
        .get_draft(&project_id)
        .expect("draft")
        .contains("Debug report"));
    let metadata = service
        .get_review_metadata(&project_id)
        .expect("metadata");
    assert_eq!(metadata.last_template_kind, "debug_report");
    assert!(metadata.last_generated_at.is_some());
    assert_eq!(metadata.last_export_format.as_deref(), Some("json"));

    let _ = fs::remove_file(&path);
}

#[test]
fn workflow_template_kind_persists_across_reopen() {
    let path = temp_db_path("template-persist");
    let _ = fs::remove_file(&path);

    let project_id = {
        let service = service_at(&path);
        let project = service.create("Template Persist", None).expect("create");
        service
            .save_template_kind(&project.id, "weekly_summary")
            .expect("save kind");
        project.id
    };

    let service = service_at(&path);
    assert_eq!(
        service.get_template_kind(&project_id).expect("kind"),
        "weekly_summary"
    );

    let _ = fs::remove_file(&path);
}

#[test]
fn isolation_matrix_project_actions_do_not_mutate_peer() {
    let path = temp_db_path("isolation-matrix");
    let _ = fs::remove_file(&path);
    let service = service_at(&path);

    let a = service.create("Project A", None).expect("a");
    let b = service.create("Project B", None).expect("b");

    service
        .save_context(&a.id, "Context A")
        .expect("context a");
    service
        .import_context_handoff(&b.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
        .expect("import b");
    service
        .generate_draft(&b.id, "handover")
        .expect("generate b");

    assert_eq!(service.get_context(&a.id).expect("get a"), "Context A");
    assert_ne!(
        service.get_context(&b.id).expect("get b"),
        service.get_context(&a.id).expect("get a again")
    );

    let _ = fs::remove_file(&path);
}

#[test]
fn migration_matrix_upgrades_fresh_database_to_latest() {
    let path = temp_db_path("migration-matrix");
    let _ = fs::remove_file(&path);

    let db = Database::open(&path).expect("open");
    let conn = db.connection();

    let migration_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| row.get(0))
        .expect("count");
    assert_eq!(migration_count, 5);

    let review_column: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name = 'last_export_format'",
            [],
            |row| row.get(0),
        )
        .expect("column");
    assert_eq!(review_column, 1);

    let _ = fs::remove_file(&path);
}

#[test]
fn ipc_alias_generate_status_draft_matches_status_update_kind() {
    let path = temp_db_path("alias");
    let _ = fs::remove_file(&path);
    let service = service_at(&path);

    let project = service.create("Alias Project", None).expect("create");
    service
        .save_context(&project.id, UNSTRUCTURED_PROSE)
        .expect("save context");

    let alias = service
        .generate_status_draft(&project.id)
        .expect("alias");
    service
        .save_context(&project.id, UNSTRUCTURED_PROSE)
        .expect("reset context");
    let explicit = service
        .generate_draft(&project.id, "status_update")
        .expect("explicit");

    assert_eq!(alias, explicit);

    let _ = fs::remove_file(&path);
}

#[test]
fn failed_export_prepare_does_not_record_metadata() {
    let path = temp_db_path("failed-export");
    let _ = fs::remove_file(&path);
    let service = service_at(&path);

    let project = service.create("Failed Export", None).expect("create");
    let _ = service
        .prepare_draft_export(&project.id, "status_update", "   ", Some("txt"), None)
        .expect_err("empty prepare");

    let metadata = service.get_review_metadata(&project.id).expect("metadata");
    assert!(metadata.last_exported_at.is_none());

    let _ = fs::remove_file(&path);
}
