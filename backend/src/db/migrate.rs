use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;

use super::DbError;

const MIGRATIONS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");

pub fn apply_pending_migrations(conn: &Connection) -> Result<(), DbError> {
    let applied = applied_versions(conn)?;
    let mut migrations = load_migrations()?;
    migrations.sort_by_key(|m| m.version);

    for migration in migrations {
        if applied.contains(&migration.version) {
            continue;
        }

        let sql = fs::read_to_string(&migration.path)?;
        let tx = conn.unchecked_transaction().map_err(DbError::from)?;

        if let Err(e) = tx.execute_batch(&sql) {
            return Err(DbError::MigrationApply {
                version: migration.version,
                message: e.to_string(),
            });
        }

        if let Err(e) = tx.execute(
            "INSERT INTO schema_migrations (version) VALUES (?1)",
            [migration.version],
        ) {
            return Err(DbError::MigrationApply {
                version: migration.version,
                message: e.to_string(),
            });
        }

        tx.commit().map_err(|e| DbError::MigrationApply {
            version: migration.version,
            message: e.to_string(),
        })?;
    }

    Ok(())
}

struct MigrationFile {
    version: i32,
    path: PathBuf,
}

fn migrations_dir() -> PathBuf {
    PathBuf::from(MIGRATIONS_DIR)
}

fn load_migrations() -> Result<Vec<MigrationFile>, DbError> {
    let dir = migrations_dir();
    if !dir.is_dir() {
        return Err(DbError::MigrationNotFound(dir.display().to_string()));
    }

    let mut migrations = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("sql") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DbError::MigrationParse(path.display().to_string()))?;
        let version = parse_version(file_name)?;
        migrations.push(MigrationFile { version, path });
    }

    if migrations.is_empty() {
        return Err(DbError::MigrationNotFound(dir.display().to_string()));
    }

    Ok(migrations)
}

fn parse_version(file_name: &str) -> Result<i32, DbError> {
    let stem = file_name.strip_suffix(".sql").unwrap_or(file_name);
    let digits: String = stem.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(DbError::MigrationParse(file_name.to_string()));
    }
    digits
        .parse::<i32>()
        .map_err(|_| DbError::MigrationParse(file_name.to_string()))
}

fn applied_versions(conn: &Connection) -> Result<std::collections::HashSet<i32>, DbError> {
    let table_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_migrations'",
        [],
        |row| row.get(0),
    )?;

    if table_exists == 0 {
        return Ok(std::collections::HashSet::new());
    }

    let mut stmt = conn.prepare("SELECT version FROM schema_migrations")?;
    let versions = stmt
        .query_map([], |row| row.get::<_, i32>(0))?
        .collect::<Result<std::collections::HashSet<_>, _>>()?;

    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("grafitalk_m1_{label}_{nanos}.db"))
    }

    fn table_exists(conn: &Connection, name: &str) -> i64 {
        conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
            [name],
            |row| row.get(0),
        )
        .expect("query")
    }

    #[test]
    fn migration_creates_projects_table_on_fresh_db() {
        let path = temp_db_path("fresh");
        let _ = fs::remove_file(&path);

        let conn = Connection::open(&path).expect("open");
        apply_pending_migrations(&conn).expect("migrate");

        assert_eq!(table_exists(&conn, "projects"), 1);
        assert_eq!(table_exists(&conn, "schema_migrations"), 1);

        let version_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM schema_migrations WHERE version = 1",
                [],
                |row| row.get(0),
            )
            .expect("version row");
        assert_eq!(version_count, 1);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn migration_is_idempotent() {
        let path = temp_db_path("idempotent");
        let _ = fs::remove_file(&path);

        let conn = Connection::open(&path).expect("open");
        apply_pending_migrations(&conn).expect("migrate first");
        apply_pending_migrations(&conn).expect("migrate second");

        let version_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM schema_migrations WHERE version = 1",
                [],
                |row| row.get(0),
            )
            .expect("version count");
        assert_eq!(version_count, 1);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn parse_version_reads_leading_digits() {
        assert_eq!(parse_version("001_projects.sql").expect("parse"), 1);
    }

    #[test]
    fn migration_adds_context_and_draft_columns() {
        let path = temp_db_path("columns");
        let _ = fs::remove_file(&path);

        let conn = Connection::open(&path).expect("open");
        apply_pending_migrations(&conn).expect("migrate");

        let context_column: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name = 'context_text'",
                [],
                |row| row.get(0),
            )
            .expect("context_text column");
        let draft_column: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name = 'draft_text'",
                [],
                |row| row.get(0),
            )
            .expect("draft_text column");

        assert_eq!(context_column, 1);
        assert_eq!(draft_column, 1);

        let version_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| row.get(0))
            .expect("version count");
        assert_eq!(version_count, 5);

        let generated_column: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name = 'last_generated_at'",
                [],
                |row| row.get(0),
            )
            .expect("last_generated_at column");
        let exported_column: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name = 'last_exported_at'",
                [],
                |row| row.get(0),
            )
            .expect("last_exported_at column");

        assert_eq!(generated_column, 1);
        assert_eq!(exported_column, 1);

        let _ = fs::remove_file(&path);
    }
}
