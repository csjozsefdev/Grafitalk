use thiserror::Error;

use crate::db::DbError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Db(#[from] DbError),

    #[error("{0}")]
    Validation(String),
}
