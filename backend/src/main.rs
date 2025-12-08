use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware::{self},
    extract::State,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate_backend::{
    AppState,
    config::AppConfig,
    database::{get_pg_pool, get_neo4j_pool, run_migrations, run_initializations, PgPool, Neo4jPool},
    middleware::{auth_middleware, logging_middleware, cors_middleware, rate_limit_middleware, RateLimiter},
    handlers::{
        auth::{login, register, get_current_user, logout},
        dashboard::get_dashboard_stats,
        ci_management::{
            create_ci_type, list_ci_types, create_ci_asset, list_ci_assets,
            get_ci_asset, update_ci_asset, delete_ci_asset
        },
        graph::{get_graph_data, get_node_neighbors, search_nodes},
        audit::get_audit_logs,
        amortization::{get_valuation_records, get_amortization_schedule},
        import_export::{import_ci_assets, export_ci_assets},
    },
    jobs::start_background_jobs,
    error::AppError,
};


#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load configuration
    let config = AppConfig::from_env();

    // Initialize logging
    init_logging(&config);

    // Initialize database connections
    let pg_pool = get_pg_pool(&config.database.postgres).await?;
    let neo4j_pool = get_neo4j_pool(&config.database.neo4j).await?;

    // Run database migrations
    run_migrations(&pg_pool).await?;
    run_initializations(&neo4j_pool).await?;

    // Start background jobs
    start_background_jobs(pg_pool.clone()).await?;

    // Initialize rate limiter
    let rate_limiter = RateLimiter::new(100, std::time::Duration::from_secs(60)); // 100 requests per minute

    // Create application state
    let app_state = AppState::new(config.clone(), pg_pool.clone(), neo4j_pool.clone(), rate_limiter);

    // Build the application router
    let app = create_app(app_state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::internal(format!("Failed to start server: {}", e)))?;

    Ok(())
}

fn create_app(app_state: AppState) -> Router {
    // Build middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_middleware(&app_state.config.cors))
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn_with_state(
            app_state.rate_limiter.clone(),
            rate_limit_middleware,
        ));

    // Create API routes
    let api_routes = Router::new()
        // Health check (no auth required)
        .route("/health", get(health_check))

        // Authentication routes (no auth required)
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/logout", post(logout))

        // Protected routes
        .route("/auth/me", get(get_current_user))
        .route("/dashboard/stats", get(get_dashboard_stats))
        .route("/ci-types", post(create_ci_type))
        .route("/ci-types", get(list_ci_types))
        .route("/ci-assets", post(create_ci_asset))
        .route("/ci-assets", get(list_ci_assets))
        .route("/ci-assets/:id", get(get_ci_asset))
        .route("/ci-assets/:id", put(update_ci_asset))
        .route("/ci-assets/:id", delete(delete_ci_asset))
        .route("/graph/data", get(get_graph_data))
        .route("/graph/nodes/:id/neighbors", get(get_node_neighbors))
        .route("/graph/search", get(search_nodes))
        .route("/audit/logs", get(get_audit_logs))
        .route("/amortization/records", get(get_valuation_records))
        .route("/amortization/assets/:id/schedule", get(get_amortization_schedule))
        .route("/import/ci-assets", post(import_ci_assets))
        .route("/export/ci-assets", get(export_ci_assets))
        .layer(middleware::from_fn_with_state(
            app_state.config.auth.jwt_secret.clone(),
            auth_middleware,
        ));

    // Combine with middleware and state
    Router::new()
        .nest("/api/v1", api_routes)
        .with_state(app_state)
        .layer(middleware_stack)
}

fn init_logging(config: &AppConfig) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.logging.level));

    if config.logging.format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(filter)
            .init();
    }
}

async fn health_check() -> &'static str {
    "OK"
}