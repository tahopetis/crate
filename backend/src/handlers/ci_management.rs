use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    database::{CIRepository, GraphRepository},
    error::{AppError, AppResult},
    models::{CreateCITypeRequest, UpdateCITypeRequest, CreateCIAssetRequest, CIAssetFilter},
    services::CIService,
    middleware::AuthContext,
};

pub async fn create_ci_type(
    State(app_state): State<crate::AppState>,
    auth_context: crate::middleware::AuthContext,
    Json(request_data): Json<CreateCITypeRequest>,
) -> AppResult<Json<Value>> {

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Create CI type
    let ci_type = ci_service.create_ci_type(request_data, auth_context.user_id).await?;

    Ok(Json(json!({
        "data": ci_type,
        "message": "CI type created successfully"
    })))
}

pub async fn list_ci_types(
    State(app_state): State<crate::AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> AppResult<Json<Value>> {
    // Parse query parameters
    let limit: Option<i64> = params.get("limit")
        .and_then(|s| s.parse().ok());
    let offset: Option<i64> = params.get("offset")
        .and_then(|s| s.parse().ok());

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // List CI types
    let ci_types = ci_service.list_ci_types(limit, offset).await?;

    // Get total count
    let total = ci_service.count_ci_types().await?;

    Ok(Json(json!({
        "data": ci_types,
        "pagination": {
            "total": total,
            "limit": limit.unwrap_or(50),
            "offset": offset.unwrap_or(0)
        },
        "message": "CI types retrieved successfully"
    })))
}

pub async fn create_ci_asset(
    State(app_state): State<crate::AppState>,
    auth_context: AuthContext,
    Json(request_data): Json<CreateCIAssetRequest>,
) -> AppResult<Json<Value>> {

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Create CI asset
    let asset_id = ci_service.create_ci_asset(request_data, auth_context.user_id).await?;

    Ok(Json(json!({
        "data": {
            "id": asset_id.to_string()
        },
        "message": "CI asset created successfully"
    })))
}

pub async fn list_ci_assets(
    State(app_state): State<crate::AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> AppResult<Json<Value>> {
    // Parse query parameters
    let ci_type_id: Option<Uuid> = params.get("ci_type_id")
        .and_then(|s| s.parse().ok());
    let limit: Option<i64> = params.get("limit")
        .and_then(|s| s.parse().ok());
    let offset: Option<i64> = params.get("offset")
        .and_then(|s| s.parse().ok());

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // List CI assets
    let ci_assets = ci_service.list_ci_assets(ci_type_id, limit, offset).await?;

    Ok(Json(json!({
        "data": ci_assets,
        "filters": {
            "ci_type_id": ci_type_id.map(|id| id.to_string()),
            "limit": limit.unwrap_or(50),
            "offset": offset.unwrap_or(0)
        },
        "message": "CI assets retrieved successfully"
    })))
}

pub async fn get_ci_asset(
    State(app_state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Get CI asset
    let ci_asset = ci_service.get_ci_asset(id).await?
        .ok_or_else(|| AppError::not_found(&format!("CI asset with id '{}' not found", id)))?;

    Ok(Json(json!({
        "data": {
            "id": ci_asset.0.to_string(),
            "name": ci_asset.1,
            "attributes": ci_asset.2,
            "ci_type_id": ci_asset.3.to_string(),
            "created_by": ci_asset.4.to_string()
        },
        "message": "CI asset retrieved successfully"
    })))
}

pub async fn update_ci_asset(
    State(app_state): State<crate::AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
    Json(request_data): Json<Value>,
) -> AppResult<Json<Value>> {

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Extract update parameters
    let name = request_data.get("name")
        .and_then(|v| v.as_str());
    let attributes = request_data.get("attributes");

    // Update CI asset
    let updated = ci_service.update_ci_asset(id, name, attributes, auth_context.user_id).await?;

    if !updated {
        return Err(AppError::not_found(&format!("CI asset with id '{}' not found", id)));
    }

    Ok(Json(json!({
        "message": "CI asset updated successfully"
    })))
}

pub async fn delete_ci_asset(
    State(app_state): State<crate::AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {

    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Delete CI asset
    ci_service.delete_ci_asset(id, auth_context.user_id).await?;

    Ok(Json(json!({
        "message": "CI asset deleted successfully"
    })))
}

// Additional CI Type handlers for complete CRUD

pub async fn get_ci_type(
    State(app_state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Get CI type
    let ci_type = ci_service.get_ci_type_by_id(id).await?
        .ok_or_else(|| AppError::not_found(&format!("CI type with id '{}' not found", id)))?;

    Ok(Json(json!({
        "data": ci_type,
        "message": "CI type retrieved successfully"
    })))
}

pub async fn update_ci_type(
    State(app_state): State<crate::AppState>,
    Path(id): Path<Uuid>,
    Json(request_data): Json<UpdateCITypeRequest>,
) -> AppResult<Json<Value>> {
    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Update CI type
    let ci_type = ci_service.update_ci_type(id, request_data).await?;

    Ok(Json(json!({
        "data": ci_type,
        "message": "CI type updated successfully"
    })))
}

pub async fn delete_ci_type(
    State(app_state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    // Initialize repositories
    let ci_repository = CIRepository::new(app_state.pg_pool.clone());
    let graph_repository = GraphRepository::new(app_state.neo4j_pool.clone());

    // Initialize service
    let ci_service = CIService::new(ci_repository, graph_repository);

    // Delete CI type
    ci_service.delete_ci_type(id).await?;

    Ok(Json(json!({
        "message": "CI type deleted successfully"
    })))
}