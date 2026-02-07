use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_diesel_api::ErrorResponse;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    // === Repository Errors ===
    NotFound(String),
    Duplicate(String),
    DatabaseError(String),

    // === Validation Errors ===
    ValidationError(String),
    InvalidInput(String),

    // === Internal Errors ===
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Duplicate(msg) => write!(f, "Already exists: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message, details) = self.get_error_info();

        let body = Json(ErrorResponse {
            error: error_code.to_string(),
            message,
            details,
        });

        (status, body).into_response()
    }
}

impl AppError {
    /// Retrieves formatted error information for HTTP response
    fn get_error_info(&self) -> (StatusCode, &'static str, String, Option<String>) {
        match self {
            // 404 Not Found
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone(), None),

            // 409 Conflict
            AppError::Duplicate(msg) => {
                (StatusCode::CONFLICT, "DUPLICATE_ENTRY", msg.clone(), None)
            }

            // 400 Bad Request
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                msg.clone(),
                None,
            ),
            AppError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, "INVALID_INPUT", msg.clone(), None)
            }

            // 500 Internal Server Error
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                "An error occurred with the database".to_string(),
                Some(msg.clone()),
            ),
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "An internal server error occurred".to_string(),
                Some(msg.clone()),
            ),
        }
    }

    // === Helper constructors ===
    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError::NotFound(msg.into())
    }

    pub fn duplicate(msg: impl Into<String>) -> Self {
        AppError::Duplicate(msg.into())
    }

    pub fn database(msg: impl Into<String>) -> Self {
        AppError::DatabaseError(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::InternalServerError(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::ValidationError(msg.into())
    }

    pub fn invalid_input(msg: impl Into<String>) -> Self {
        AppError::InvalidInput(msg.into())
    }

    /// Returns the HTTP status code
    #[allow(dead_code)]
    pub fn status_code(&self) -> StatusCode {
        self.get_error_info().0
    }
}

// === Automatic conversions from other error types ===

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::internal(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::invalid_input(format!("JSON error: {}", err))
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::invalid_input(format!("Invalid UUID: {}", err))
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::invalid_input(format!("Invalid date format: {}", err))
    }
}

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(err: axum::extract::rejection::JsonRejection) -> Self {
        AppError::invalid_input(format!("Invalid JSON: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::not_found("Task");
        assert_eq!(err.to_string(), "Not found: Task");
    }

    #[test]
    fn test_not_found_status() {
        assert_eq!(
            AppError::not_found("test").status_code(),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn test_validation_status() {
        assert_eq!(
            AppError::validation("test").status_code(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn test_internal_status() {
        assert_eq!(
            AppError::internal("test").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_response() {
        let err = AppError::not_found("Task");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
