use axum::{
    extract::{Request, ConnectInfo},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;
use tracing::{info, warn};
use uuid::Uuid;

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    // Get real IP from headers if behind a proxy
    let real_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .unwrap_or("unknown");

    // Generate request ID for tracing
    let request_id = Uuid::new_v4().to_string();

    // Log request
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        user_agent = %user_agent,
        real_ip = %real_ip,
        "Incoming request"
    );

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    // Log response
    let log_level = match status.as_u16() {
        200..=299 => "info",
        300..=399 => "info",
        400..=499 => "warn",
        500..=599 => "error",
        _ => "info",
    };

    match log_level {
        "info" => info!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status = %status,
            duration_ms = %duration.as_millis(),
            "Request completed"
        ),
        "warn" => warn!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status = %status,
            duration_ms = %duration.as_millis(),
            "Request completed with warning"
        ),
        "error" => warn!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status = %status,
            duration_ms = %duration.as_millis(),
            "Request completed with error"
        ),
        _ => {}
    }

    response
}