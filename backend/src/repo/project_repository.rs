use rusqlite::{params, Connection, Row};

use crate::db::DbError;
use crate::model::{Project, ProjectReviewMetadata};

pub struct ProjectRepository;

impl ProjectRepository {
    pub fn insert(conn: &Connection, project: &Project) -> Result<(), DbError> {
        conn.execute(
            "INSERT INTO projects (
                id, name, client_label, status,
                created_at, updated_at, last_used_at, archived_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                project.id,
                project.name,
                project.client_label,
                project.status,
                project.created_at,
                project.updated_at,
                project.last_used_at,
                project.archived_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, project_id: &str) -> Result<Project, DbError> {
        conn.query_row(
            "SELECT id, name, client_label, status, created_at, updated_at, last_used_at, archived_at
             FROM projects WHERE id = ?1",
            [project_id],
            map_project_row,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            },
            other => DbError::from(other),
        })
    }

    pub fn get_context_text(conn: &Connection, project_id: &str) -> Result<String, DbError> {
        conn.query_row(
            "SELECT context_text FROM projects WHERE id = ?1",
            [project_id],
            |row| row.get::<_, Option<String>>(0).map(|value| value.unwrap_or_default()),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            },
            other => DbError::from(other),
        })
    }

    pub fn save_context_text(
        conn: &Connection,
        project_id: &str,
        context_text: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET context_text = ?1, updated_at = ?2 WHERE id = ?3",
            params![context_text, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn get_draft_text(conn: &Connection, project_id: &str) -> Result<String, DbError> {
        conn.query_row(
            "SELECT draft_text FROM projects WHERE id = ?1",
            [project_id],
            |row| row.get::<_, Option<String>>(0).map(|value| value.unwrap_or_default()),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            },
            other => DbError::from(other),
        })
    }

    pub fn save_draft_text(
        conn: &Connection,
        project_id: &str,
        draft_text: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET draft_text = ?1, updated_at = ?2 WHERE id = ?3",
            params![draft_text, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn get_last_template_kind(conn: &Connection, project_id: &str) -> Result<String, DbError> {
        conn.query_row(
            "SELECT last_template_kind FROM projects WHERE id = ?1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            },
            other => DbError::from(other),
        })
    }

    pub fn save_last_template_kind(
        conn: &Connection,
        project_id: &str,
        template_kind: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET last_template_kind = ?1, updated_at = ?2 WHERE id = ?3",
            params![template_kind, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn get_review_metadata(
        conn: &Connection,
        project_id: &str,
    ) -> Result<ProjectReviewMetadata, DbError> {
        conn.query_row(
            "SELECT last_generated_at, last_template_kind, last_exported_at, last_export_format
             FROM projects WHERE id = ?1",
            [project_id],
            |row| {
                Ok(ProjectReviewMetadata {
                    last_generated_at: row.get(0)?,
                    last_template_kind: row.get(1)?,
                    last_exported_at: row.get(2)?,
                    last_export_format: row.get(3)?,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            },
            other => DbError::from(other),
        })
    }

    pub fn save_last_generated_at(
        conn: &Connection,
        project_id: &str,
        last_generated_at: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET last_generated_at = ?1, updated_at = ?2 WHERE id = ?3",
            params![last_generated_at, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn migrate_last_generated_at_if_empty(
        conn: &Connection,
        project_id: &str,
        last_generated_at: &str,
        updated_at: &str,
    ) -> Result<bool, DbError> {
        let changed = conn.execute(
            "UPDATE projects
             SET last_generated_at = ?1, updated_at = ?2
             WHERE id = ?3 AND last_generated_at IS NULL",
            params![last_generated_at, updated_at, project_id],
        )?;

        if changed == 0 {
            let exists: i64 = conn.query_row(
                "SELECT COUNT(*) FROM projects WHERE id = ?1",
                [project_id],
                |row| row.get(0),
            )?;
            if exists == 0 {
                return Err(DbError::NotFound {
                    entity: "project".to_string(),
                    id: project_id.to_string(),
                });
            }
            return Ok(false);
        }

        Ok(true)
    }

    pub fn record_export_metadata(
        conn: &Connection,
        project_id: &str,
        last_exported_at: &str,
        last_export_format: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects
             SET last_exported_at = ?1, last_export_format = ?2, updated_at = ?3
             WHERE id = ?4",
            params![
                last_exported_at,
                last_export_format,
                updated_at,
                project_id
            ],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn touch_last_used(
        conn: &Connection,
        project_id: &str,
        last_used_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET last_used_at = ?1 WHERE id = ?2",
            params![last_used_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn list_active(conn: &Connection) -> Result<Vec<Project>, DbError> {
        let mut stmt = conn.prepare(
            "SELECT id, name, client_label, status, created_at, updated_at, last_used_at, archived_at
             FROM projects
             WHERE status = 'active'
             ORDER BY COALESCE(last_used_at, updated_at, created_at) DESC",
        )?;

        let projects = stmt
            .query_map([], map_project_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn count_active(conn: &Connection) -> Result<i64, DbError> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE status = 'active'",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn archive_project(
        conn: &Connection,
        project_id: &str,
        archived_at: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET status = 'archived', archived_at = ?1, updated_at = ?2 WHERE id = ?3 AND status = 'active'",
            params![archived_at, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn restore_project(
        conn: &Connection,
        project_id: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET status = 'active', archived_at = NULL, updated_at = ?1 WHERE id = ?2 AND status = 'archived'",
            params![updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn update_project_name(
        conn: &Connection,
        project_id: &str,
        name: &str,
        updated_at: &str,
    ) -> Result<(), DbError> {
        let changed = conn.execute(
            "UPDATE projects SET name = ?1, updated_at = ?2 WHERE id = ?3 AND status = 'active'",
            params![name, updated_at, project_id],
        )?;

        if changed == 0 {
            return Err(DbError::NotFound {
                entity: "project".to_string(),
                id: project_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn list_archived(conn: &Connection) -> Result<Vec<Project>, DbError> {
        let mut stmt = conn.prepare(
            "SELECT id, name, client_label, status, created_at, updated_at, last_used_at, archived_at
             FROM projects
             WHERE status = 'archived'
             ORDER BY COALESCE(archived_at, updated_at, created_at) DESC",
        )?;

        let projects = stmt
            .query_map([], map_project_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn count_archived(conn: &Connection) -> Result<i64, DbError> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE status = 'archived'",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}

fn map_project_row(row: &Row<'_>) -> Result<Project, rusqlite::Error> {
    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        client_label: row.get(2)?,
        status: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
        last_used_at: row.get(6)?,
        archived_at: row.get(7)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("grafitalk_m2_repo_{label}_{nanos}.db"))
    }

    fn sample_project(id: &str, name: &str) -> Project {
        Project {
            id: id.to_string(),
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
    fn create_inserts_row() {
        let path = temp_db_path("insert");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("p1", "Alpha");
        ProjectRepository::insert(conn, &project).expect("insert");

        let list = ProjectRepository::list_active(conn).expect("list");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "Alpha");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn list_active_excludes_archived() {
        let path = temp_db_path("archived");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        ProjectRepository::insert(conn, &sample_project("active-1", "Active")).expect("insert");
        ProjectRepository::insert(
            conn,
            &Project {
                status: "archived".to_string(),
                archived_at: Some("2026-01-02T00:00:00Z".to_string()),
                ..sample_project("archived-1", "Archived")
            },
        )
        .expect("insert archived");

        let list = ProjectRepository::list_active(conn).expect("list");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "active-1");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn context_round_trips() {
        let path = temp_db_path("context");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        ProjectRepository::insert(conn, &sample_project("ctx-1", "Context Test")).expect("insert");

        let empty =
            ProjectRepository::get_context_text(conn, "ctx-1").expect("get empty context");
        assert_eq!(empty, "");

        ProjectRepository::save_context_text(
            conn,
            "ctx-1",
            "Handoff notes for QA.",
            "2026-01-02T00:00:00Z",
        )
        .expect("save context");

        let saved =
            ProjectRepository::get_context_text(conn, "ctx-1").expect("get saved context");
        assert_eq!(saved, "Handoff notes for QA.");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn touch_last_used_updates_timestamp() {
        let path = temp_db_path("touch");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        ProjectRepository::insert(conn, &sample_project("touch-1", "Touch Test")).expect("insert");
        ProjectRepository::touch_last_used(conn, "touch-1", "2026-06-03T12:00:00Z")
            .expect("touch");

        let project = ProjectRepository::get_by_id(conn, "touch-1").expect("get");
        assert_eq!(
            project.last_used_at.as_deref(),
            Some("2026-06-03T12:00:00Z")
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn draft_round_trips() {
        let path = temp_db_path("draft");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        ProjectRepository::insert(conn, &sample_project("draft-1", "Draft Test")).expect("insert");

        let empty = ProjectRepository::get_draft_text(conn, "draft-1").expect("get empty draft");
        assert_eq!(empty, "");

        ProjectRepository::save_draft_text(
            conn,
            "draft-1",
            "Status update\n\nFocus Area:\nDeployment notes for QA.",
            "2026-01-02T00:00:00Z",
        )
        .expect("save draft");

        let saved = ProjectRepository::get_draft_text(conn, "draft-1").expect("get saved draft");
        assert_eq!(
            saved,
            "Status update\n\nFocus Area:\nDeployment notes for QA."
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn template_kind_round_trips_with_default() {
        let path = temp_db_path("template-kind");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("tpl-1", "Template Test");
        ProjectRepository::insert(conn, &project).expect("insert");

        let default =
            ProjectRepository::get_last_template_kind(conn, "tpl-1").expect("get default");
        assert_eq!(default, "status_update");

        ProjectRepository::save_last_template_kind(
            conn,
            "tpl-1",
            "debug_report",
            "2026-01-02T00:00:00Z",
        )
        .expect("save kind");

        let saved =
            ProjectRepository::get_last_template_kind(conn, "tpl-1").expect("get saved kind");
        assert_eq!(saved, "debug_report");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn review_metadata_round_trips() {
        let path = temp_db_path("review-meta");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("review-1", "Review Test");
        ProjectRepository::insert(conn, &project).expect("insert");

        let empty = ProjectRepository::get_review_metadata(conn, "review-1").expect("get");
        assert!(empty.last_generated_at.is_none());
        assert_eq!(empty.last_template_kind, "status_update");

        ProjectRepository::save_last_generated_at(
            conn,
            "review-1",
            "2026-06-07T10:00:00Z",
            "2026-06-07T10:00:00Z",
        )
        .expect("save generated");

        ProjectRepository::record_export_metadata(
            conn,
            "review-1",
            "2026-06-07T11:00:00Z",
            "pdf",
            "2026-06-07T11:00:00Z",
        )
        .expect("save export");

        let saved = ProjectRepository::get_review_metadata(conn, "review-1").expect("get saved");
        assert_eq!(
            saved.last_generated_at.as_deref(),
            Some("2026-06-07T10:00:00Z")
        );
        assert_eq!(saved.last_export_format.as_deref(), Some("pdf"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn archive_project_hides_from_active_list() {
        let path = temp_db_path("archive-action");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("p-archive", "To Archive");
        ProjectRepository::insert(conn, &project).expect("insert");

        ProjectRepository::archive_project(
            conn,
            "p-archive",
            "2026-06-14T00:00:00Z",
            "2026-06-14T00:00:00Z",
        )
        .expect("archive");

        let list = ProjectRepository::list_active(conn).expect("list");
        assert!(list.is_empty());

        let archived = ProjectRepository::get_by_id(conn, "p-archive").expect("get");
        assert_eq!(archived.status, "archived");
        assert!(archived.archived_at.is_some());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn restore_project_returns_to_active_list() {
        let path = temp_db_path("restore-repo");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("p-restore", "Restore Me");
        ProjectRepository::insert(conn, &project).expect("insert");
        ProjectRepository::archive_project(
            conn,
            "p-restore",
            "2026-06-14T00:00:00Z",
            "2026-06-14T00:00:00Z",
        )
        .expect("archive");

        ProjectRepository::restore_project(conn, "p-restore", "2026-06-15T00:00:00Z")
            .expect("restore");

        let active = ProjectRepository::list_active(conn).expect("list");
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].status, "active");
        assert!(active[0].archived_at.is_none());

        let archived = ProjectRepository::list_archived(conn).expect("archived");
        assert!(archived.is_empty());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn update_project_name_changes_active_project_only() {
        let path = temp_db_path("rename-repo");
        let _ = fs::remove_file(&path);
        let db = Database::open(&path).expect("open");
        let conn = db.connection();

        let project = sample_project("p-rename", "Before");
        ProjectRepository::insert(conn, &project).expect("insert");

        ProjectRepository::update_project_name(
            conn,
            "p-rename",
            "After",
            "2026-06-14T00:00:00Z",
        )
        .expect("rename");

        let saved = ProjectRepository::get_by_id(conn, "p-rename").expect("get");
        assert_eq!(saved.name, "After");

        let _ = fs::remove_file(&path);
    }
}
