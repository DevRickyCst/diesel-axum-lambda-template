use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Create a PostgreSQL connection pool using DATABASE_URL.
pub fn create_pool() -> Result<DbPool> {
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .context("failed to build r2d2 pool")
}
