use serde::Deserialize;
use std::env;
use super::database::{DatabaseConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub password_min_length: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        }
    }
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
            password_min_length: env::var("PASSWORD_MIN_LENGTH")
                .ok()
                .and_then(|l| l.parse().ok())
                .unwrap_or(8),
        }
    }
}

impl LoggingConfig {
    pub fn from_env() -> Self {
        Self {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            format: env::var("LOG_FORMAT").unwrap_or_else(|_| "json".to_string()),
        }
    }
}

impl CorsConfig {
    pub fn from_env() -> Self {
        Self {
            allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .ok()
                .map(|origins| origins.split(',').map(|s| s.to_string()).collect())
                .unwrap_or_else(|| vec!["http://localhost:3000".to_string()]),
            allowed_methods: env::var("CORS_ALLOWED_METHODS")
                .ok()
                .map(|methods| methods.split(',').map(|s| s.to_string()).collect())
                .unwrap_or_else(|| vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]),
            allowed_headers: env::var("CORS_ALLOWED_HEADERS")
                .ok()
                .map(|headers| headers.split(',').map(|s| s.to_string()).collect())
                .unwrap_or_else(|| vec![
                    "authorization".to_string(),
                    "content-type".to_string(),
                    "accept".to_string(),
                    "x-requested-with".to_string(),
                ]),
            allow_credentials: env::var("CORS_ALLOW_CREDENTIALS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig::from_env(),
            database: DatabaseConfig::from_env(),
            auth: AuthConfig::from_env(),
            logging: LoggingConfig::from_env(),
            cors: CorsConfig::from_env(),
        }
    }
}