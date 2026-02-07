use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::OnceLock;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

static POOL: OnceLock<DbPool> = OnceLock::new();

/// Initialize the PostgreSQL connection pool using DATABASE_URL.
/// This should be called once at application startup.
pub fn init_pool() -> Result<()> {
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .context("failed to build r2d2 pool")?;

    POOL.set(pool)
        .map_err(|_| anyhow::anyhow!("Pool already initialized"))?;

    Ok(())
}

/// Get a reference to the initialized pool.
/// Panics if the pool hasn't been initialized with init_pool().
pub fn get_pool() -> &'static DbPool {
    POOL.get()
        .expect("DB pool not initialized. Call init_pool() first.")
}

/// Get a connection from the pool.
pub fn get_connection() -> Result<DbConnection> {
    get_pool()
        .get()
        .context("Failed to get connection from pool")
}

/// Legacy function for compatibility with existing code.
/// Creates a new pool - use init_pool() and get_pool() instead for better performance.
#[deprecated(note = "Use init_pool() at startup and get_pool() to access the pool")]
#[allow(dead_code)]
pub fn create_pool() -> Result<DbPool> {
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .context("failed to build r2d2 pool")
}
