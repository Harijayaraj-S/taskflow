//! State Db

//! Database Manager

use anyhow::{Context, Result};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::info;

#[derive(Clone)]
pub struct DbManager {
    pool: Pool<Postgres>,
}

impl DbManager {
    pub async fn new() -> Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").context("DATABASE_URL env var is not set")?;

        info!("initializing postgres connection pool");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .connect(&database_url)
            .await
            .context("failed to create postgres connection pool")?;

        Ok(Self { pool })
    }

    /// Get shared pool reference
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Health check (used for /health endpoint)
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .context("database health check failed")?;

        Ok(())
    }
}
