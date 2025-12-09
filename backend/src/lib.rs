pub mod config;
pub mod models;
pub mod handlers;
pub mod services;
pub mod database;
pub mod utils;
pub mod middleware;
pub mod jobs;
pub mod error;

use database::{PgPool, Neo4jPool, CIRepository, LifecycleRepository, RelationshipRepository, GraphRepository};
use middleware::RateLimiter;
use std::sync::Arc;

// Database layer containing repositories
#[derive(Clone)]
pub struct Database {
    pub ci_repository: CIRepository,
    pub lifecycle_repository: LifecycleRepository,
    pub relationship_repository: RelationshipRepository,
    pub graph_repository: Arc<GraphRepository>,
}

impl Database {
    pub fn new(pg_pool: PgPool, neo4j_pool: Neo4jPool) -> Self {
        Self {
            ci_repository: CIRepository::new(pg_pool.clone()),
            lifecycle_repository: LifecycleRepository::new(pg_pool.clone()),
            relationship_repository: RelationshipRepository::new(pg_pool),
            graph_repository: Arc::new(GraphRepository::new(neo4j_pool)),
        }
    }
}

// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    pub pg_pool: PgPool,
    pub neo4j_pool: Neo4jPool,
    pub database: Database,
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
            pg_pool: pg_pool.clone(),
            neo4j_pool: neo4j_pool.clone(),
            database: Database::new(pg_pool, neo4j_pool),
            rate_limiter,
        }
    }
}

pub use error::{AppError, AppResult};