use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum RepositoryError {
    NotFound,
    Duplicate,
    Database(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Resource not found"),
            RepositoryError::Duplicate => write!(f, "Resource already exists"),
            RepositoryError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Maps Diesel errors to RepositoryError
pub fn map_diesel_error(err: DieselError) -> RepositoryError {
    match err {
        DieselError::NotFound => RepositoryError::NotFound,
        DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
            RepositoryError::Duplicate
        }
        _ => RepositoryError::Database(err.to_string()),
    }
}

impl From<DieselError> for RepositoryError {
    fn from(err: DieselError) -> Self {
        map_diesel_error(err)
    }
}

impl From<RepositoryError> for crate::error::AppError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => crate::error::AppError::not_found("Resource not found"),
            RepositoryError::Duplicate => {
                crate::error::AppError::duplicate("Resource already exists")
            }
            RepositoryError::Database(msg) => crate::error::AppError::database(msg),
        }
    }
}
