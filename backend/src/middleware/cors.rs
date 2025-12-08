use axum::http::{HeaderValue, Method, HeaderName};
use tower_http::cors::{Any, CorsLayer};
use crate::config::CorsConfig;

pub fn cors_middleware(config: &CorsConfig) -> CorsLayer {
    let allowed_origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let allowed_methods: Vec<Method> = config
        .allowed_methods
        .iter()
        .filter_map(|method| method.parse().ok())
        .collect();

    let allowed_headers: Vec<HeaderName> = config
        .allowed_headers
        .iter()
        .filter_map(|header| header.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .allow_credentials(config.allow_credentials)
        .expose_headers([header::CONTENT_TYPE])
        .max_age(std::time::Duration::from_secs(3600))
}

use axum::http::header;