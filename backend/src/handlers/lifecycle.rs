use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{
        LifecycleTypeSummary, LifecycleTypeResponse,
        CreateLifecycleTypeRequest, UpdateLifecycleTypeRequest,
        CreateLifecycleStateRequest, UpdateLifecycleStateRequest,
        CreateCITypeLifecycleRequest,
    },
    services::LifecycleService,
    middleware::AuthContext,
};

// Lifecycle Types Handlers

pub async fn create_lifecycle_type(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Json(request): Json<CreateLifecycleTypeRequest>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_type = lifecycle_service
        .create_lifecycle_type(request, &auth_context)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_type,
        "message": "Lifecycle type created successfully"
    })))
}

pub async fn get_lifecycle_type(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
) -> AppResult<Json<LifecycleTypeResponse>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_type = lifecycle_service
        .get_lifecycle_type(id)
        .await?;

    Ok(Json(lifecycle_type))
}

pub async fn list_lifecycle_types(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Query(params): Query<serde_json::Value>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let include_inactive = params
        .get("include_inactive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let lifecycle_types = lifecycle_service
        .list_lifecycle_types(include_inactive)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_types,
        "count": lifecycle_types.len()
    })))
}

pub async fn update_lifecycle_type(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateLifecycleTypeRequest>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_type = lifecycle_service
        .update_lifecycle_type(id, request)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_type,
        "message": "Lifecycle type updated successfully"
    })))
}

pub async fn delete_lifecycle_type(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    lifecycle_service
        .delete_lifecycle_type(id, &auth_context)
        .await?;

    Ok(Json(json!({
        "success": true,
        "message": "Lifecycle type deleted successfully"
    })))
}

// Lifecycle States Handlers

pub async fn create_lifecycle_state(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Json(request): Json<CreateLifecycleStateRequest>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_state = lifecycle_service
        .create_lifecycle_state(request, &auth_context)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_state,
        "message": "Lifecycle state created successfully"
    })))
}

pub async fn get_lifecycle_state(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_state = lifecycle_service
        .get_lifecycle_state(id)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_state
    })))
}

pub async fn update_lifecycle_state(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateLifecycleStateRequest>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycle_state = lifecycle_service
        .update_lifecycle_state(id, request)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycle_state,
        "message": "Lifecycle state updated successfully"
    })))
}

pub async fn delete_lifecycle_state(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    lifecycle_service
        .delete_lifecycle_state(id, &auth_context)
        .await?;

    Ok(Json(json!({
        "success": true,
        "message": "Lifecycle state deleted successfully"
    })))
}

// CI Type to Lifecycle Type Mapping Handlers

pub async fn create_ci_type_lifecycle_mapping(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Json(request): Json<CreateCITypeLifecycleRequest>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let mapping = lifecycle_service
        .create_ci_type_lifecycle_mapping(request, &auth_context)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": mapping,
        "message": "CI type lifecycle mapping created successfully"
    })))
}

pub async fn get_lifecycles_for_ci_type(
    State(app_state): State<AppState>,
    auth_context: AuthContext,
    Path(ci_type_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let lifecycle_service = LifecycleService::new(
        app_state.database.lifecycle_repository.clone(),
        app_state.database.ci_repository.clone(),
    );

    let lifecycles = lifecycle_service
        .get_lifecycles_for_ci_type(ci_type_id)
        .await?;

    Ok(Json(json!({
        "success": true,
        "data": lifecycles,
        "count": lifecycles.len()
    })))
}

// Helper function to generate color palette for lifecycles
pub async fn get_lifecycle_colors(
    State(_app_state): State<AppState>,
    auth_context: AuthContext,
) -> AppResult<Json<Value>> {
    // Predefined color palette for lifecycle states
    let colors = vec![
        "#10B981", // Emerald (green)
        "#3B82F6", // Blue
        "#F59E0B", // Amber (yellow)
        "#EF4444", // Red
        "#8B5CF6", // Violet (purple)
        "#EC4899", // Pink
        "#14B8A6", // Teal
        "#F97316", // Orange
        "#6366F1", // Indigo
        "#84CC16", // Lime
        "#06B6D4", // Cyan
        "#A855F7", // Purple
        "#FB923C", // Orange
        "#0EA5E9", // Sky
        "#22C55E", // Green
    ];

    Ok(Json(json!({
        "success": true,
        "data": colors,
        "message": "Predefined color palette for lifecycle states"
    })))
}