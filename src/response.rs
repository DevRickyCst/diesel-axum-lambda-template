use axum::{
    Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum_diesel_api::{AppResponse as ApiResponse, StatusCode as ApiStatusCode};
use serde::Serialize;

/// Backend wrapper for axum-diesel-api's AppResponse with Axum integration
///
/// This wrapper adds Axum's IntoResponse trait and header support to the
/// WASM-compatible AppResponse from the API crate.
///
/// # Examples
///
/// ```rust
/// use crate::response::AppResponse;
///
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
    inner: ApiResponse<T>,
    headers: Option<HeaderMap>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    /// Creates a new response wrapping an API response
    pub fn new(inner: ApiResponse<T>) -> Self {
        Self {
            inner,
            headers: None,
        }
    }

    /// Adds headers to the response
    #[allow(dead_code)]
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    // === Convenience constructors ===

    /// 200 OK with data
    pub fn ok(data: T) -> Self {
        Self::new(ApiResponse::ok(data))
    }

    /// 201 Created with data
    pub fn created(data: T) -> Self {
        Self::new(ApiResponse::created(data))
    }

    /// 202 Accepted with data
    #[allow(dead_code)]
    pub fn accepted(data: T) -> Self {
        Self::new(ApiResponse::accepted(data))
    }
}

impl AppResponse<()> {
    /// 204 No Content
    pub fn no_content() -> Self {
        Self::new(ApiResponse::no_content())
    }
}

/// Converts API StatusCode to Axum's StatusCode
fn convert_status(api_status: ApiStatusCode) -> StatusCode {
    match api_status {
        ApiStatusCode::Ok => StatusCode::OK,
        ApiStatusCode::Created => StatusCode::CREATED,
        ApiStatusCode::Accepted => StatusCode::ACCEPTED,
        ApiStatusCode::NoContent => StatusCode::NO_CONTENT,
        ApiStatusCode::BadRequest => StatusCode::BAD_REQUEST,
        ApiStatusCode::Unauthorized => StatusCode::UNAUTHORIZED,
        ApiStatusCode::Forbidden => StatusCode::FORBIDDEN,
        ApiStatusCode::NotFound => StatusCode::NOT_FOUND,
        ApiStatusCode::Conflict => StatusCode::CONFLICT,
        ApiStatusCode::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
        ApiStatusCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Implements Axum's IntoResponse trait
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = convert_status(self.inner.status);
        let mut response = match self.inner.data {
            Some(data) => (status, Json(data)).into_response(),
            None => status.into_response(),
        };

        if let Some(headers) = self.headers {
            response.headers_mut().extend(headers);
        }

        response
    }
}
