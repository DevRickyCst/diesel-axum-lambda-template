use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Generic type for handler responses
///
/// # Examples
///
/// ```rust
/// // Simple JSON response
/// AppResponse::ok(user_data)
///
/// // Response with custom status
/// AppResponse::created(new_user)
///
/// // Empty response
/// AppResponse::no_content()
///
/// // Response with headers
/// AppResponse::ok(login_response).with_headers(headers)
/// ```
pub struct AppResponse<T> {
    status: StatusCode,
    headers: Option<HeaderMap>,
    body: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    /// Creates a new response with a status code and data
    pub fn new(status: StatusCode, body: T) -> Self {
        Self {
            status,
            headers: None,
            body: Some(body),
        }
    }

    /// Creates an empty response with a status code
    pub fn empty(status: StatusCode) -> Self {
        Self {
            status,
            headers: None,
            body: None,
        }
    }

    /// Adds headers to the response
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    // === Common status code constructors ===

    /// 200 OK with data
    pub fn ok(body: T) -> Self {
        Self::new(StatusCode::OK, body)
    }

    /// 201 Created with data
    pub fn created(body: T) -> Self {
        Self::new(StatusCode::CREATED, body)
    }

    /// 202 Accepted with data
    #[allow(dead_code)]
    pub fn accepted(body: T) -> Self {
        Self::new(StatusCode::ACCEPTED, body)
    }
}

impl AppResponse<()> {
    /// 204 No Content
    pub fn no_content() -> Self {
        Self::empty(StatusCode::NO_CONTENT)
    }
}

/// Implementation of IntoResponse trait for Axum
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut response = match self.body {
            Some(body) => (self.status, Json(body)).into_response(),
            None => self.status.into_response(),
        };

        if let Some(headers) = self.headers {
            response.headers_mut().extend(headers);
        }

        response
    }
}

/// Type alias for handler results
/// Uses AppResponse for success and AppError for errors
#[allow(dead_code)]
pub type AppResult<T> = Result<AppResponse<T>, crate::error::AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        message: String,
    }

    #[test]
    fn test_ok_response() {
        let data = TestData {
            message: "success".to_string(),
        };
        let response = AppResponse::ok(data);
        assert_eq!(response.status, StatusCode::OK);
        assert!(response.body.is_some());
    }

    #[test]
    fn test_created_response() {
        let data = TestData {
            message: "created".to_string(),
        };
        let response = AppResponse::created(data);
        assert_eq!(response.status, StatusCode::CREATED);
    }

    #[test]
    fn test_no_content_response() {
        let response = AppResponse::no_content();
        assert_eq!(response.status, StatusCode::NO_CONTENT);
        assert!(response.body.is_none());
    }

    #[test]
    fn test_response_with_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Custom-Header", "value".parse().unwrap());

        let data = TestData {
            message: "with headers".to_string(),
        };
        let response = AppResponse::ok(data).with_headers(headers);
        assert!(response.headers.is_some());
    }
}
