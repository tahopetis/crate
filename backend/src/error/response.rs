use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some("Resource created successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn updated(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some("Resource updated successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn deleted() -> Self {
        Self {
            success: true,
            data: None,
            message: Some("Resource deleted successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    pub message: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(
        data: Vec<T>,
        page: u64,
        per_page: u64,
        total: u64,
    ) -> Self {
        let total_pages = (total + per_page - 1) / per_page;

        Self {
            success: true,
            data,
            pagination: PaginationInfo {
                page,
                per_page,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_message(
        data: Vec<T>,
        page: u64,
        per_page: u64,
        total: u64,
        message: String,
    ) -> Self {
        let total_pages = (total + per_page - 1) / per_page;

        Self {
            success: true,
            data,
            pagination: PaginationInfo {
                page,
                per_page,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
            message: Some(message),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl<T> IntoResponse for PaginatedResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetails,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<Value>,
}

impl ErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        Self {
            success: false,
            error: ErrorDetails {
                code,
                message,
                details: None,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_details(code: String, message: String, details: Value) -> Self {
        Self {
            success: false,
            error: ErrorDetails {
                code,
                message,
                details: Some(details),
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.error.code.as_str() {
            "validation_failed" => StatusCode::BAD_REQUEST,
            "not_found" => StatusCode::NOT_FOUND,
            "conflict" => StatusCode::CONFLICT,
            "unauthorized" => StatusCode::UNAUTHORIZED,
            "forbidden" => StatusCode::FORBIDDEN,
            "bad_request" => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}