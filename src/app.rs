// src/app.rs

use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use crate::handlers::health::health;
use crate::db::connection::DbPool;


/// Construit l'application complète
pub fn build_router(db_pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(db_pool)
        .layer(TraceLayer::new_for_http())
}
