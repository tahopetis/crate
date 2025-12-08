use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tower::ServiceExt;

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }

    pub fn is_allowed(&self, key: &str) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();

        let entry = entries.entry(key.to_string()).or_insert_with(|| RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) >= self.window_duration {
            entry.count = 0;
            entry.window_start = now;
        }

        if entry.count >= self.max_requests {
            false
        } else {
            entry.count += 1;
            true
        }
    }

    pub fn cleanup_expired_entries(&self) {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();

        entries.retain(|_, entry| {
            now.duration_since(entry.window_start) < self.window_duration * 2
        });
    }
}

pub async fn rate_limit_middleware(
    State(rate_limiter): State<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Use IP address as the rate limiting key
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .unwrap_or_else(|| {
            // Fallback to remote address
            "unknown"
        });

    let key = format!("rate_limit:{}", client_ip);

    if rate_limiter.is_allowed(&key) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}

// Background task to cleanup expired entries
pub async fn cleanup_task(rate_limiter: RateLimiter) {
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Cleanup every minute

    loop {
        interval.tick().await;
        rate_limiter.cleanup_expired_entries();
    }
}