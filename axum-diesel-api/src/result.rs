use serde::{Deserialize, Serialize};

/// HTTP status codes as enum (WASM-compatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    UnprocessableEntity = 422,
    InternalServerError = 500,
}

/// Generic API response wrapper (WASM-compatible)
///
/// # Examples
///
/// ```rust
/// use axum_diesel_api::AppResponse;
///
/// // Simple response
/// let response = AppResponse::ok("success");
///
/// // Empty response
/// let response = AppResponse::no_content();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub status: StatusCode,
}

impl<T> AppResponse<T> {
    /// Creates a new response with a status code and data
    pub fn new(status: StatusCode, data: T) -> Self {
        Self {
            status,
            data: Some(data),
        }
    }

    /// Creates an empty response with a status code
    pub fn empty(status: StatusCode) -> Self {
        Self { status, data: None }
    }

    // === Common status code constructors ===

    /// 200 OK with data
    pub fn ok(data: T) -> Self {
        Self::new(StatusCode::Ok, data)
    }

    /// 201 Created with data
    pub fn created(data: T) -> Self {
        Self::new(StatusCode::Created, data)
    }

    /// 202 Accepted with data
    pub fn accepted(data: T) -> Self {
        Self::new(StatusCode::Accepted, data)
    }
}

impl AppResponse<()> {
    /// 204 No Content
    pub fn no_content() -> Self {
        Self::empty(StatusCode::NoContent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TestData {
        message: String,
    }

    #[test]
    fn test_ok_response() {
        let data = TestData {
            message: "success".to_string(),
        };
        let response = AppResponse::ok(data);
        assert_eq!(response.status, StatusCode::Ok);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_created_response() {
        let data = TestData {
            message: "created".to_string(),
        };
        let response = AppResponse::created(data);
        assert_eq!(response.status, StatusCode::Created);
    }

    #[test]
    fn test_no_content_response() {
        let response = AppResponse::no_content();
        assert_eq!(response.status, StatusCode::NoContent);
        assert!(response.data.is_none());
    }
}
