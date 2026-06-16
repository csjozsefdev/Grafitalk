use std::path::Path;

use rusqlite::Connection;

use super::migrate::apply_pending_migrations;
use super::DbError;

pub struct Database {
    conn: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbHealth {
    pub db_ok: bool,
    pub latest_migration: Option<i32>,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let conn = Connection::open(path)?;
        apply_pending_migrations(&conn)?;

        Ok(Self { conn })
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn health(&self) -> Result<DbHealth, DbError> {
        let latest_migration: Option<i32> = self
            .conn
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
                row.get::<_, Option<i32>>(0)
            })?;

        Ok(DbHealth {
            db_ok: true,
            latest_migration,
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
        std::env::temp_dir().join(format!("grafitalk_m1_open_{label}_{nanos}.db"))
    }

    #[test]
    fn open_creates_file_and_runs_migrations() {
        let path = temp_db_path("open");
        let _ = fs::remove_file(&path);

        let db = Database::open(&path).expect("open");
        assert!(path.is_file());

        let projects: i64 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'projects'",
                [],
                |row| row.get(0),
            )
            .expect("projects table");
        assert_eq!(projects, 1);

        let health = db.health().expect("health");
        assert!(health.db_ok);
        assert_eq!(health.latest_migration, Some(5));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn open_is_idempotent() {
        let path = temp_db_path("reopen");
        let _ = fs::remove_file(&path);

        Database::open(&path).expect("first open");
        Database::open(&path).expect("second open");

        let _ = fs::remove_file(&path);
    }
}
