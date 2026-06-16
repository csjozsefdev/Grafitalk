use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("migration not found: {0}")]
    MigrationNotFound(String),

    #[error("migration parse error: {0}")]
    MigrationParse(String),

    #[error("migration apply failed (version {version}): {message}")]
    MigrationApply { version: i32, message: String },

    #[error("{entity} not found: {id}")]
    NotFound { entity: String, id: String },
}
