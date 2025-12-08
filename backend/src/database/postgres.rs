use crate::config::PostgreSQLConfig;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use anyhow::Result;

pub type PgPool = Pool<Postgres>;

pub async fn get_pg_pool(config: &PostgreSQLConfig) -> Result<PgPool> {
    let options = PgConnectOptions::from_str(&config.connection_string())?
        // .disable_statement_logging(); // Not available in this SQLx version
    ;

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect_with(options)
        .await?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./src/database/migrations/postgres")
        .run(pool)
        .await?;
    Ok(())
}