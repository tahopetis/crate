use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Neo4j error: {0}")]
    Neo4j(#[from] neo4rs::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Cron error: {0}")]
    Cron(#[from] cron::error::Error),

    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

impl AppError {
    pub fn internal<T: Into<String>>(message: T) -> Self {
        Self::Internal(message.into())
    }

    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found<T: Into<String>>(message: T) -> Self {
        Self::NotFound(message.into())
    }

    pub fn conflict<T: Into<String>>(message: T) -> Self {
        Self::Conflict(message.into())
    }

    pub fn authentication<T: Into<String>>(message: T) -> Self {
        Self::Authentication(message.into())
    }

    pub fn authorization<T: Into<String>>(message: T) -> Self {
        Self::Authorization(message.into())
    }

    pub fn configuration<T: Into<String>>(message: T) -> Self {
        Self::Configuration(message.into())
    }

    pub fn validation<T: Into<String>>(message: T) -> Self {
        Self::Validation(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error")
            }
            AppError::Neo4j(ref err) => {
                tracing::error!("Neo4j error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal graph database error")
            }
            AppError::Authentication(ref message) => {
                (StatusCode::UNAUTHORIZED, message.as_str())
            }
            AppError::Authorization(ref message) => {
                (StatusCode::FORBIDDEN, message.as_str())
            }
            AppError::Validation(ref message) => {
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            AppError::NotFound(ref message) => {
                (StatusCode::NOT_FOUND, message.as_str())
            }
            AppError::Conflict(ref message) => {
                (StatusCode::CONFLICT, message.as_str())
            }
            AppError::BadRequest(ref message) => {
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            AppError::Internal(ref message) => {
                tracing::error!("Internal error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Configuration(ref message) => {
                tracing::error!("Configuration error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error")
            }
            AppError::Jwt(ref err) => {
                tracing::warn!("JWT error: {:?}", err);
                (StatusCode::UNAUTHORIZED, "Invalid authentication token")
            }
            AppError::Bcrypt(ref err) => {
                tracing::error!("Bcrypt error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Password processing error")
            }
            AppError::Io(ref err) => {
                tracing::error!("IO error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "File system error")
            }
            AppError::Csv(ref err) => {
                tracing::error!("CSV error: {:?}", err);
                (StatusCode::BAD_REQUEST, "CSV processing error")
            }
            AppError::Cron(ref err) => {
                tracing::error!("Cron error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Scheduled job error")
            }
            AppError::Serialization(ref err) => {
                tracing::error!("Serialization error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Data serialization error")
            }
            AppError::Uuid(ref err) => {
                tracing::error!("UUID error: {:?}", err);
                (StatusCode::BAD_REQUEST, "Invalid ID format")
            }
            AppError::Generic(ref err) => {
                tracing::error!("Generic error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": true,
            "message": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

        (status, body).into_response()
    }
}