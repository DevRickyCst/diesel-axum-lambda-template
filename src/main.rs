use std::env;

mod api;
mod app;
mod db;
mod error;
mod handlers;
mod services;

use app::build_router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Setup logging for all execution modes
fn setup_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,{{project_name}}=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging early for all modes
    setup_logging();

    // Initialize database connection pool
    db::connection::init_pool().expect("Failed to initialize database pool");
    tracing::info!("Database connection pool initialized");

    let app = build_router();

    if env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        // Lambda execution mode
        tracing::info!("Starting in Lambda mode");
        lambda_http::run(app).await?;
    } else {
        // Local HTTP server mode
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
        let addr = format!("{}:{}", host, port);

        tracing::info!("Starting in local HTTP server mode");
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        tracing::info!("Server listening on http://{}", addr);

        axum::serve(listener, app).await?;
    }

    Ok(())
}
