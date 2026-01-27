use std::env;
mod app;
mod db;
mod handlers;

use tracing;
use app::build_router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    // Initialize logging early for local runs.
    setup_logging().await;

    // Create DB pool.
    let pool = db::connection::create_pool().expect("failed to create DB pool");

    let app = build_router(pool);

    if env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        // Lambda execution
        lambda_http::run(app).await
    } else {
        let addr = "0.0.0.0:3000";
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("🚀 Server running at http://{}", addr);
        let app = app.layer(TraceLayer::new_for_http());
        axum::serve(listener, app).await?;
        Ok(())
    }
}
