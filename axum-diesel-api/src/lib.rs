//! # axum-diesel-api
//!
//! Shared API types for the axum-diesel-project.
//! WASM-compatible - can be used in frontend applications.
//!
//! ## Features
//!
//! - Request/Response DTOs
//! - Error response format
//! - Generic response wrapper
//! - Zero server dependencies (no Axum, Diesel, Tokio)
//!
//! ## Usage
//!
//! ```rust
//! use axum_diesel_api::CreateTaskRequest;
//!
//! // Create request
//! let request = CreateTaskRequest {
//!     title: "My Task".to_string(),
//!     description: Some("Description".to_string()),
//!     completed: false,
//! };
//! ```

pub mod error;
pub mod requests;
pub mod responses;
pub mod result;

// Re-exports for convenience
pub use error::ErrorResponse;
pub use requests::{CreateTaskRequest, UpdateTaskRequest};
pub use responses::TaskResponse;
pub use result::{AppResponse, StatusCode};
