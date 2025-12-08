pub mod config;
pub mod models;
pub mod handlers;
pub mod services;
pub mod database;
pub mod utils;
pub mod middleware;
pub mod jobs;
pub mod error;

use database::{PgPool, Neo4jPool};
use middleware::RateLimiter;

// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    pub pg_pool: PgPool,
    pub neo4j_pool: Neo4jPool,
    pub rate_limiter: RateLimiter,
}

impl AppState {
    pub fn new(
        config: config::AppConfig,
        pg_pool: PgPool,
        neo4j_pool: Neo4jPool,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            config,
            pg_pool,
            neo4j_pool,
            rate_limiter,
        }
    }
}

pub use error::{AppError, AppResult};