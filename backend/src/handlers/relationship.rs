use crate::services::RelationshipService;
use crate::models::{
    CreateRelationshipTypeRequest, UpdateRelationshipTypeRequest, RelationshipTypeFilter
};
use crate::middleware::AuthContext;
use crate::error::ApiResponse;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

pub fn relationship_routes() -> Router<crate::AppState> {
    Router::new()
        .route("/relationship-types", get(list_relationship_types).post(create_relationship_type))
        .route("/relationship-types/:id", get(get_relationship_type).put(update_relationship_type).delete(delete_relationship_type))
}

#[derive(Debug, Deserialize)]
pub struct ListRelationshipTypesQuery {
    pub search: Option<String>,
    pub from_ci_type_id: Option<String>,
    pub to_ci_type_id: Option<String>,
    pub is_bidirectional: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_relationship_types(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Query(query): Query<ListRelationshipTypesQuery>,
) -> Result<Json<ApiResponse<Vec<crate::models::RelationshipTypeSummary>>>, StatusCode> {
    let filter = RelationshipTypeFilter {
        search: query.search,
        from_ci_type_id: query.from_ci_type_id.and_then(|s| s.parse().ok()),
        to_ci_type_id: query.to_ci_type_id.and_then(|s| s.parse().ok()),
        is_bidirectional: query.is_bidirectional,
        limit: query.limit,
        offset: query.offset,
    };

    let relationship_service = RelationshipService::new(
        app_state.database.relationship_repository.clone(),
        app_state.database.ci_repository.clone(),
        app_state.database.graph_repository.clone(),
    );

    match relationship_service.list_relationship_types(filter).await {
        Ok(relationship_types) => Ok(Json(ApiResponse::success(relationship_types))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_relationship_type(
    State(app_state): State<crate::AppState>,
    auth: AuthContext,
    Json(request): Json<CreateRelationshipTypeRequest>,
) -> Result<Json<ApiResponse<crate::models::RelationshipType>>, StatusCode> {
    let relationship_service = RelationshipService::new(
        app_state.database.relationship_repository.clone(),
        app_state.database.ci_repository.clone(),
        app_state.database.graph_repository.clone(),
    );

    match relationship_service.create_relationship_type(request, auth.user_id).await {
        Ok(relationship_type) => Ok(Json(ApiResponse {
            success: true,
            data: Some(relationship_type),
            message: Some("Relationship type created successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Error: {}", e)),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
    }
}

pub async fn get_relationship_type(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::RelationshipType>>, StatusCode> {
    let relationship_service = RelationshipService::new(
        app_state.database.relationship_repository.clone(),
        app_state.database.ci_repository.clone(),
        app_state.database.graph_repository.clone(),
    );

    let id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match relationship_service.get_relationship_type(id).await {
        Ok(Some(relationship_type)) => Ok(Json(ApiResponse {
            success: true,
            data: Some(relationship_type),
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Ok(None) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Relationship type not found".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_relationship_type(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateRelationshipTypeRequest>,
) -> Result<Json<ApiResponse<crate::models::RelationshipType>>, StatusCode> {
    let relationship_service = RelationshipService::new(
        app_state.database.relationship_repository.clone(),
        app_state.database.ci_repository.clone(),
        app_state.database.graph_repository.clone(),
    );

    let id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match relationship_service.update_relationship_type(id, request).await {
        Ok(relationship_type) => Ok(Json(ApiResponse {
            success: true,
            data: Some(relationship_type),
            message: Some("Relationship type updated successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Error: {}", e)),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
    }
}

pub async fn delete_relationship_type(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let relationship_service = RelationshipService::new(
        app_state.database.relationship_repository.clone(),
        app_state.database.ci_repository.clone(),
        app_state.database.graph_repository.clone(),
    );

    let id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match relationship_service.delete_relationship_type(id).await {
        Ok(()) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Relationship type deleted successfully".to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Error: {}", e)),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
    }
}