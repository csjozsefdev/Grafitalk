use chrono::Utc;
use uuid::Uuid;

use crate::db::Database;
use crate::export::{prepare_export, ExportFormat, PreparedExport};
use crate::import::{detect_format, import_to_context_text, parse_import, ImportFormat};
use crate::model::Project;
use crate::repo::ProjectRepository;
use crate::template::{render_draft_from_context_text, TemplateKind};

use super::ServiceError;

pub const MAX_PROJECT_NAME_LEN: usize = 120;

fn normalize_project_name(name: impl Into<String>) -> Result<String, ServiceError> {
    let name = name.into().trim().to_string();
    if name.is_empty() {
        return Err(ServiceError::Validation(
            "Project name is required".to_string(),
        ));
    }
    if name.chars().count() > MAX_PROJECT_NAME_LEN {
        return Err(ServiceError::Validation(format!(
            "Project name must be at most {MAX_PROJECT_NAME_LEN} characters"
        )));
    }
    Ok(name)
}

pub struct ProjectService {
    db: Database,
}

impl ProjectService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn list_active(&self) -> Result<Vec<Project>, ServiceError> {
        Ok(ProjectRepository::list_active(self.db.connection())?)
    }

    pub fn touch_last_used(&self, project_id: &str) -> Result<(), ServiceError> {
        let last_used_at = Utc::now().to_rfc3339();
        ProjectRepository::touch_last_used(self.db.connection(), project_id, &last_used_at)?;
        Ok(())
    }

    pub fn get_context(&self, project_id: &str) -> Result<String, ServiceError> {
        Ok(ProjectRepository::get_context_text(
            self.db.connection(),
            project_id,
        )?)
    }

    pub fn get_draft(&self, project_id: &str) -> Result<String, ServiceError> {
        Ok(ProjectRepository::get_draft_text(
            self.db.connection(),
            project_id,
        )?)
    }

    pub fn save_draft(
        &self,
        project_id: &str,
        draft_text: impl Into<String>,
    ) -> Result<(), ServiceError> {
        let draft_text = draft_text.into();
        let updated_at = Utc::now().to_rfc3339();

        ProjectRepository::save_draft_text(
            self.db.connection(),
            project_id,
            &draft_text,
            &updated_at,
        )?;

        Ok(())
    }

    pub fn generate_status_draft(&self, project_id: &str) -> Result<String, ServiceError> {
        self.generate_draft(project_id, TemplateKind::DEFAULT.as_str())
    }

    pub fn generate_draft(
        &self,
        project_id: &str,
        template_kind: &str,
    ) -> Result<String, ServiceError> {
        let kind = TemplateKind::parse(template_kind)
            .map_err(|message| ServiceError::Validation(message))?;

        let conn = self.db.connection();
        let project = ProjectRepository::get_by_id(conn, project_id)?;
        let context_text = ProjectRepository::get_context_text(conn, project_id)?;

        if context_text.trim().is_empty() {
            return Err(ServiceError::Validation(
                "Add context before generating a draft.".to_string(),
            ));
        }

        let draft = render_draft_from_context_text(kind, &project, &context_text);
        let updated_at = Utc::now().to_rfc3339();

        ProjectRepository::save_draft_text(conn, project_id, &draft, &updated_at)?;
        ProjectRepository::save_last_template_kind(conn, project_id, kind.as_str(), &updated_at)?;
        ProjectRepository::save_last_generated_at(conn, project_id, &updated_at, &updated_at)?;

        Ok(draft)
    }

    pub fn get_review_metadata(
        &self,
        project_id: &str,
    ) -> Result<crate::model::ProjectReviewMetadata, ServiceError> {
        Ok(ProjectRepository::get_review_metadata(
            self.db.connection(),
            project_id,
        )?)
    }

    pub fn migrate_last_generated_at_if_empty(
        &self,
        project_id: &str,
        legacy_timestamp: &str,
    ) -> Result<bool, ServiceError> {
        ProjectRepository::get_by_id(self.db.connection(), project_id)?;
        let updated_at = Utc::now().to_rfc3339();
        Ok(ProjectRepository::migrate_last_generated_at_if_empty(
            self.db.connection(),
            project_id,
            legacy_timestamp,
            &updated_at,
        )?)
    }

    pub fn record_draft_export(
        &self,
        project_id: &str,
        export_format: &str,
    ) -> Result<(), ServiceError> {
        crate::export::ExportFormat::parse(export_format)
            .map_err(|err| ServiceError::Validation(err.to_string()))?;

        ProjectRepository::get_by_id(self.db.connection(), project_id)?;
        let exported_at = Utc::now().to_rfc3339();
        let updated_at = exported_at.clone();

        ProjectRepository::record_export_metadata(
            self.db.connection(),
            project_id,
            &exported_at,
            export_format,
            &updated_at,
        )?;

        Ok(())
    }

    pub fn get_template_kind(&self, project_id: &str) -> Result<String, ServiceError> {
        Ok(ProjectRepository::get_last_template_kind(
            self.db.connection(),
            project_id,
        )?)
    }

    pub fn save_template_kind(
        &self,
        project_id: &str,
        template_kind: &str,
    ) -> Result<(), ServiceError> {
        let kind = TemplateKind::parse(template_kind)
            .map_err(|message| ServiceError::Validation(message))?;

        ProjectRepository::get_by_id(self.db.connection(), project_id)?;
        let updated_at = Utc::now().to_rfc3339();

        ProjectRepository::save_last_template_kind(
            self.db.connection(),
            project_id,
            kind.as_str(),
            &updated_at,
        )?;

        Ok(())
    }

    pub fn save_context(
        &self,
        project_id: &str,
        context_text: impl Into<String>,
    ) -> Result<(), ServiceError> {
        let context_text = context_text.into();
        let updated_at = Utc::now().to_rfc3339();

        ProjectRepository::save_context_text(
            self.db.connection(),
            project_id,
            &context_text,
            &updated_at,
        )?;

        Ok(())
    }

    pub fn import_context_handoff(
        &self,
        project_id: &str,
        content: &str,
        format_hint: Option<&str>,
        path_hint: Option<&str>,
    ) -> Result<String, ServiceError> {
        ProjectRepository::get_by_id(self.db.connection(), project_id)?;

        let format = format_hint
            .and_then(ImportFormat::from_hint)
            .unwrap_or_else(|| detect_format(content, path_hint));

        let parsed = parse_import(content, format)
            .map_err(|err| ServiceError::Validation(err.to_string()))?;
        let context_text = import_to_context_text(&parsed)
            .map_err(|err| ServiceError::Validation(err.to_string()))?;

        self.save_context(project_id, context_text.clone())?;

        Ok(context_text)
    }

    pub fn prepare_draft_export(
        &self,
        project_id: &str,
        template_kind: &str,
        draft_text: &str,
        format_hint: Option<&str>,
        path_hint: Option<&str>,
    ) -> Result<PreparedExport, ServiceError> {
        TemplateKind::parse(template_kind)
            .map_err(|message| ServiceError::Validation(message))?;

        let project = ProjectRepository::get_by_id(self.db.connection(), project_id)?;

        let format = if let Some(path) = path_hint {
            ExportFormat::from_path_hint(path).map_err(|err| ServiceError::Validation(err.to_string()))?
        } else if let Some(hint) = format_hint {
            ExportFormat::parse(hint).map_err(|err| ServiceError::Validation(err.to_string()))?
        } else {
            return Err(ServiceError::Validation(
                "Choose a .txt, .md, .json, or .pdf file.".to_string(),
            ));
        };

        prepare_export(&project, template_kind, draft_text, format)
            .map_err(|err| ServiceError::Validation(err.to_string()))
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        client_label: Option<String>,
    ) -> Result<Project, ServiceError> {
        let name = normalize_project_name(name)?;

        let client_label = client_label
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let now = Utc::now().to_rfc3339();
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name,
            client_label,
            status: "active".to_string(),
            created_at: now.clone(),
            updated_at: now,
            last_used_at: None,
            archived_at: None,
        };

        ProjectRepository::insert(self.db.connection(), &project)?;
        Ok(project)
    }

    pub fn archive_project(&self, project_id: &str) -> Result<(), ServiceError> {
        ProjectRepository::get_by_id(self.db.connection(), project_id)?;
        let now = Utc::now().to_rfc3339();
        ProjectRepository::archive_project(
            self.db.connection(),
            project_id,
            &now,
            &now,
        )?;
        Ok(())
    }

    pub fn list_archived(&self) -> Result<Vec<Project>, ServiceError> {
        Ok(ProjectRepository::list_archived(self.db.connection())?)
    }

    pub fn archived_project_count(&self) -> Result<i64, ServiceError> {
        Ok(ProjectRepository::count_archived(self.db.connection())?)
    }

    pub fn restore_project(&self, project_id: &str) -> Result<(), ServiceError> {
        ProjectRepository::get_by_id(self.db.connection(), project_id)?;
        let now = Utc::now().to_rfc3339();
        ProjectRepository::restore_project(self.db.connection(), project_id, &now)?;
        Ok(())
    }

    pub fn rename_project(
        &self,
        project_id: &str,
        name: impl Into<String>,
    ) -> Result<Project, ServiceError> {
        let name = normalize_project_name(name)?;
        let now = Utc::now().to_rfc3339();
        ProjectRepository::update_project_name(self.db.connection(), project_id, &name, &now)?;
        ProjectRepository::get_by_id(self.db.connection(), project_id)
            .map_err(ServiceError::from)
    }

    pub fn active_project_count(&self) -> Result<i64, ServiceError> {
        Ok(ProjectRepository::count_active(self.db.connection())?)
    }

    pub fn diagnostics(
        &self,
        app_version: &str,
        db_path: &str,
        platform: &str,
    ) -> Result<crate::model::AppDiagnostics, ServiceError> {
        let health = self.db.health()?;
        let project_count = ProjectRepository::count_active(self.db.connection())?;

        Ok(crate::model::AppDiagnostics {
            app_version: app_version.to_string(),
            db_path: db_path.to_string(),
            project_count,
            latest_migration: health.latest_migration,
            platform: platform.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("grafitalk_m2_service_{label}_{nanos}.db"))
    }

    fn service_at(path: &PathBuf) -> ProjectService {
        let db = Database::open(path).expect("open");
        ProjectService::new(db)
    }

    #[test]
    fn create_rejects_empty_name() {
        let path = temp_db_path("empty");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let err = service.create("   ", None).unwrap_err();
        assert!(err.to_string().contains("Project name is required"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn create_trims_name_and_sets_fields() {
        let path = temp_db_path("trim");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service
            .create("  My Project  ", Some("  Acme  ".to_string()))
            .expect("create");

        assert_eq!(project.name, "My Project");
        assert_eq!(project.client_label.as_deref(), Some("Acme"));
        assert!(!project.id.is_empty());
        assert!(!project.created_at.is_empty());
        assert_eq!(project.status, "active");

        let list = service.list_active().expect("list");
        assert_eq!(list.len(), 1);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn touch_last_used_sets_timestamp() {
        let path = temp_db_path("touch");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Touch Project", None).expect("create");
        assert!(project.last_used_at.is_none());

        service.touch_last_used(&project.id).expect("touch");
        let list = service.list_active().expect("list");
        assert!(list[0].last_used_at.is_some());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn generate_rejects_empty_context() {
        let path = temp_db_path("draft-empty");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Empty Context", None).expect("create");
        let err = service
            .generate_status_draft(&project.id)
            .expect_err("empty context");

        assert!(err.to_string().contains("Add context before generating a draft."));
        assert_eq!(service.get_draft(&project.id).expect("get draft"), "");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn projects_keep_isolated_context_and_draft() {
        let path = temp_db_path("isolation");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project_a = service.create("Project A", None).expect("create a");
        let project_b = service.create("Project B", None).expect("create b");

        service
            .save_context(&project_a.id, "Context for project A.")
            .expect("save context a");
        service
            .save_context(&project_b.id, "Context for project B.")
            .expect("save context b");

        let draft_a = service
            .generate_status_draft(&project_a.id)
            .expect("generate a");
        service
            .save_draft(&project_b.id, "Manual draft for project B.")
            .expect("save draft b");

        assert_eq!(
            service.get_context(&project_a.id).expect("get context a"),
            "Context for project A."
        );
        assert_eq!(
            service.get_context(&project_b.id).expect("get context b"),
            "Context for project B."
        );
        assert!(draft_a.contains("Summary:\nContext for project A."));
        assert_eq!(
            service.get_draft(&project_a.id).expect("get draft a"),
            draft_a
        );
        assert_eq!(
            service.get_draft(&project_b.id).expect("get draft b"),
            "Manual draft for project B."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn generate_status_draft_uses_project_and_context() {
        let path = temp_db_path("draft");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service
            .create("Mesencsi", Some("Acme".to_string()))
            .expect("create");
        service
            .save_context(&project.id, "QA handoff complete.")
            .expect("save context");

        let draft = service
            .generate_status_draft(&project.id)
            .expect("generate");

        assert!(draft.contains("Status update"));
        assert!(draft.contains("Summary:\nQA handoff complete."));
        assert!(!draft.contains("Hi team,"));
        assert!(!draft.contains("Please review before sending."));
        assert!(!draft.contains("Best regards"));
        assert_eq!(service.get_draft(&project.id).expect("get draft"), draft);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn draft_and_context_survive_database_reopen() {
        let path = temp_db_path("reopen");
        let _ = fs::remove_file(&path);

        let project_id = {
            let service = service_at(&path);
            let project = service.create("Persist Project", None).expect("create");
            service
                .save_context(&project.id, "Persisted context notes.")
                .expect("save context");
            service
                .generate_status_draft(&project.id)
                .expect("generate");
            project.id
        };

        let service = service_at(&path);
        assert_eq!(
            service.get_context(&project_id).expect("get context"),
            "Persisted context notes."
        );
        let draft = service.get_draft(&project_id).expect("get draft");
        assert!(draft.contains("Summary:\nPersisted context notes."));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_and_get_draft() {
        let path = temp_db_path("draft-save");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Draft Project", None).expect("create");
        assert_eq!(service.get_draft(&project.id).expect("get"), "");

        service
            .save_draft(&project.id, "Edited draft body.")
            .expect("save");
        assert_eq!(
            service.get_draft(&project.id).expect("get"),
            "Edited draft body."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_and_get_context() {
        let path = temp_db_path("context");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Context Project", None).expect("create");
        assert_eq!(service.get_context(&project.id).expect("get"), "");

        service
            .save_context(&project.id, "Client prefers async updates.")
            .expect("save");
        assert_eq!(
            service.get_context(&project.id).expect("get"),
            "Client prefers async updates."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn import_context_handoff_persists_canonical_text() {
        use crate::template::fixtures::GRAF_ID_SAMPLE_JSON;

        let path = temp_db_path("import-json");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Import Project", None).expect("create");
        let context = service
            .import_context_handoff(&project.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
            .expect("import");

        assert!(context.contains("Current status:"));
        assert!(context.contains("What changed:"));
        assert_eq!(
            service.get_context(&project.id).expect("get context"),
            context
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn import_failure_does_not_overwrite_existing_context() {
        let path = temp_db_path("import-fail");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Protected Project", None).expect("create");
        service
            .save_context(&project.id, "Existing context.")
            .expect("save context");

        let err = service
            .import_context_handoff(
                &project.id,
                r#"{"source":"graf-id","schema_version":"9.9","project_name":"X"}"#,
                Some("json"),
                None,
            )
            .expect_err("invalid import");

        assert!(err.to_string().contains("Unsupported Graf-ID handoff schema"));
        assert_eq!(
            service.get_context(&project.id).expect("get context"),
            "Existing context."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn import_then_generate_produces_valid_draft() {
        use crate::template::fixtures::GRAF_ID_SAMPLE_JSON;

        let path = temp_db_path("import-generate");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Mesencsi webshop", None).expect("create");
        service
            .import_context_handoff(&project.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
            .expect("import");

        let draft = service
            .generate_status_draft(&project.id)
            .expect("generate");

        assert!(draft.contains("Completed work:"));
        assert!(draft.contains("Barion sandbox flow tested."));
        assert!(!draft.contains("Focus Area:"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn imported_context_survives_database_reopen() {
        use crate::template::fixtures::GRAF_ID_LABELED_TEXT;

        let path = temp_db_path("import-reopen");
        let _ = fs::remove_file(&path);

        let project_id = {
            let service = service_at(&path);
            let project = service.create("Reopen Import", None).expect("create");
            service
                .import_context_handoff(
                    &project.id,
                    GRAF_ID_LABELED_TEXT,
                    Some("text"),
                    Some("handoff.txt"),
                )
                .expect("import");
            project.id
        };

        let service = service_at(&path);
        let context = service.get_context(&project_id).expect("get context");
        assert!(context.contains("Current status:"));
        assert!(context.contains("Current blocker:"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn import_is_isolated_per_project() {
        use crate::template::fixtures::GRAF_ID_SAMPLE_JSON;

        let path = temp_db_path("import-isolation");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project_a = service.create("Project A", None).expect("create a");
        let project_b = service.create("Project B", None).expect("create b");

        service
            .save_context(&project_b.id, "Context for project B.")
            .expect("save b");

        service
            .import_context_handoff(&project_a.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
            .expect("import a");

        assert_ne!(
            service.get_context(&project_a.id).expect("get a"),
            service.get_context(&project_b.id).expect("get b")
        );
        assert_eq!(
            service.get_context(&project_b.id).expect("get b"),
            "Context for project B."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn generate_draft_uses_selected_template() {
        use crate::template::fixtures::GRAF_ID_SAMPLE_JSON;

        let path = temp_db_path("generate-template");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Mesencsi webshop", None).expect("create");
        service
            .import_context_handoff(&project.id, GRAF_ID_SAMPLE_JSON, Some("json"), None)
            .expect("import");

        let draft = service
            .generate_draft(&project.id, "client_update")
            .expect("generate");

        assert!(draft.starts_with("Client update"));
        assert!(draft.contains("Where things stand:"));
        assert!(!draft.contains("Issue / blocker:"));
        assert_eq!(
            service.get_template_kind(&project.id).expect("kind"),
            "client_update"
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn template_kind_survives_database_reopen() {
        let path = temp_db_path("template-reopen");
        let _ = fs::remove_file(&path);

        let project_id = {
            let service = service_at(&path);
            let project = service.create("Template Persist", None).expect("create");
            service
                .save_template_kind(&project.id, "handover")
                .expect("save kind");
            project.id
        };

        let service = service_at(&path);
        assert_eq!(
            service.get_template_kind(&project_id).expect("kind"),
            "handover"
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_template_kind_rejects_unknown_value() {
        let path = temp_db_path("template-invalid");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Invalid Template", None).expect("create");
        let err = service
            .save_template_kind(&project.id, "not_a_template")
            .expect_err("invalid");

        assert!(err.to_string().contains("Unknown template kind"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prepare_draft_export_does_not_modify_stored_draft() {
        let path = temp_db_path("export-no-mutate");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Export Project", None).expect("create");
        service
            .save_draft(&project.id, "Original draft body.")
            .expect("save draft");

        service
            .prepare_draft_export(
                &project.id,
                "status_update",
                "Preview draft for export.",
                Some("json"),
                None,
            )
            .expect("prepare export");

        assert_eq!(
            service.get_draft(&project.id).expect("get draft"),
            "Original draft body."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prepare_draft_export_rejects_empty_preview() {
        let path = temp_db_path("export-empty");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Empty Export", None).expect("create");
        let err = service
            .prepare_draft_export(
                &project.id,
                "status_update",
                "   ",
                Some("txt"),
                None,
            )
            .expect_err("empty export");

        assert!(err.to_string().contains("Nothing to export yet."));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prepare_draft_export_returns_pdf_bytes() {
        let path = temp_db_path("export-pdf");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("PDF Export", None).expect("create");
        let prepared = service
            .prepare_draft_export(
                &project.id,
                "client_update",
                "Client update\n\nProject:\nPDF Export\n\nCurrent status:\nReady.",
                None,
                Some("draft.pdf"),
            )
            .expect("prepare pdf");

        assert!(prepared.bytes.unwrap().starts_with(b"%PDF"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn generate_draft_records_last_generated_at() {
        let path = temp_db_path("review-generate");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Review Generate", None).expect("create");
        service
            .save_context(&project.id, "Context for review metadata.")
            .expect("save context");

        service
            .generate_draft(&project.id, "handover")
            .expect("generate");

        let metadata = service.get_review_metadata(&project.id).expect("metadata");
        assert!(metadata.last_generated_at.is_some());
        assert_eq!(metadata.last_template_kind, "handover");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn record_draft_export_persists_export_metadata() {
        let path = temp_db_path("review-export");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Review Export", None).expect("create");
        service
            .record_draft_export(&project.id, "json")
            .expect("record export");

        let metadata = service.get_review_metadata(&project.id).expect("metadata");
        assert!(metadata.last_exported_at.is_some());
        assert_eq!(metadata.last_export_format.as_deref(), Some("json"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prepare_draft_export_does_not_record_export_metadata() {
        let path = temp_db_path("review-export-no-record");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Review Export Skip", None).expect("create");
        service
            .prepare_draft_export(
                &project.id,
                "status_update",
                "Draft body",
                Some("txt"),
                None,
            )
            .expect("prepare");

        let metadata = service.get_review_metadata(&project.id).expect("metadata");
        assert!(metadata.last_exported_at.is_none());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn migrate_last_generated_at_if_empty_only_once() {
        let path = temp_db_path("review-migrate");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Review Migrate", None).expect("create");
        assert!(service
            .migrate_last_generated_at_if_empty(&project.id, "2026-01-01T00:00:00Z")
            .expect("migrate first"));

        assert!(!service
            .migrate_last_generated_at_if_empty(&project.id, "2026-06-01T00:00:00Z")
            .expect("migrate second"));

        let metadata = service.get_review_metadata(&project.id).expect("metadata");
        assert_eq!(
            metadata.last_generated_at.as_deref(),
            Some("2026-01-01T00:00:00Z")
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn review_metadata_survives_database_reopen() {
        let path = temp_db_path("review-reopen");
        let _ = fs::remove_file(&path);

        let project_id = {
            let service = service_at(&path);
            let project = service.create("Review Reopen", None).expect("create");
            service
                .save_context(&project.id, "Persisted context.")
                .expect("save context");
            service
                .generate_draft(&project.id, "weekly_summary")
                .expect("generate");
            service
                .record_draft_export(&project.id, "md")
                .expect("export");
            project.id
        };

        let service = service_at(&path);
        let metadata = service.get_review_metadata(&project_id).expect("metadata");
        assert!(metadata.last_generated_at.is_some());
        assert_eq!(metadata.last_template_kind, "weekly_summary");
        assert_eq!(metadata.last_export_format.as_deref(), Some("md"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn archive_project_removes_from_active_list() {
        let path = temp_db_path("archive-service");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Archive Me", None).expect("create");
        service.archive_project(&project.id).expect("archive");

        let list = service.list_active().expect("list");
        assert!(list.is_empty());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn rename_project_preserves_context_and_draft() {
        let path = temp_db_path("rename-service");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Original", None).expect("create");
        service
            .save_context(&project.id, "Context body")
            .expect("save context");
        service
            .save_draft(&project.id, "Draft body")
            .expect("save draft");
        service
            .save_template_kind(&project.id, "handover")
            .expect("save template");

        let renamed = service
            .rename_project(&project.id, "Renamed Client")
            .expect("rename");
        assert_eq!(renamed.name, "Renamed Client");
        assert_eq!(renamed.id, project.id);

        assert_eq!(
            service.get_context(&project.id).expect("context"),
            "Context body"
        );
        assert_eq!(
            service.get_draft(&project.id).expect("draft"),
            "Draft body"
        );
        assert_eq!(
            service.get_template_kind(&project.id).expect("template"),
            "handover"
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn rename_project_rejects_empty_name() {
        let path = temp_db_path("rename-empty");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);
        let project = service.create("Keep", None).expect("create");

        let err = service.rename_project(&project.id, "   ").unwrap_err();
        assert!(matches!(err, ServiceError::Validation(_)));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn restore_project_returns_to_active_list_without_mutating_content() {
        let path = temp_db_path("restore-service");
        let _ = fs::remove_file(&path);
        let service = service_at(&path);

        let project = service.create("Restore Me", None).expect("create");
        service
            .save_context(&project.id, "Archived context")
            .expect("save context");
        service.archive_project(&project.id).expect("archive");

        service.restore_project(&project.id).expect("restore");

        let active = service.list_active().expect("list active");
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].name, "Restore Me");
        assert_eq!(
            service.get_context(&project.id).expect("context"),
            "Archived context"
        );

        let _ = fs::remove_file(&path);
    }
}
