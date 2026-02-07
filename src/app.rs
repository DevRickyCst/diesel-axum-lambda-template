use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::handlers::health::health;
use crate::handlers::task::{create_task, delete_task, get_task, list_tasks, update_task};

/// Build the complete application router
pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/:id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .layer(TraceLayer::new_for_http())
}
