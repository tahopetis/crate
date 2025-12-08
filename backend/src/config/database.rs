use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub postgres: PostgreSQLConfig,
    pub neo4j: Neo4jConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgreSQLConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Neo4jConfig {
    pub uri: String,
    pub username: String,
    pub password: String,
    pub max_connection_pool_size: u32,
}

impl PostgreSQLConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn from_env() -> Self {
        Self {
            host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("POSTGRES_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            username: env::var("POSTGRES_USER").unwrap_or_else(|_| "crate_user".to_string()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "crate_password".to_string()),
            database: env::var("POSTGRES_DB").unwrap_or_else(|_| "crate_db".to_string()),
            max_connections: env::var("POSTGRES_MAX_CONNECTIONS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(10),
            min_connections: env::var("POSTGRES_MIN_CONNECTIONS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(1),
        }
    }
}

impl Neo4jConfig {
    pub fn from_env() -> Self {
        Self {
            uri: env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string()),
            username: env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string()),
            password: env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            max_connection_pool_size: env::var("NEO4J_MAX_CONNECTIONS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(10),
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            postgres: PostgreSQLConfig::from_env(),
            neo4j: Neo4jConfig::from_env(),
        }
    }
}