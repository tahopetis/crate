use axum::{response::Json, extract::{Path, Query, State}, http::StatusCode};
use crate::error::ApiResponse;
use crate::middleware::AuthContext;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphDataQuery {
    pub ci_type: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
pub struct GraphData {
    pub nodes: Vec<crate::database::repositories::GraphNode>,
    pub edges: Vec<crate::database::repositories::GraphRelationship>,
}

/// Get graph data with optional filtering
pub async fn get_graph_data(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Query(params): Query<GraphDataQuery>,
) -> Result<Json<ApiResponse<GraphData>>, StatusCode> {
    let graph_repo = &app_state.database.graph_repository;

    match graph_repo.get_full_graph(
        params.limit,
        params.ci_type.as_deref(),
    ).await {
        Ok((nodes, edges)) => Ok(Json(ApiResponse {
            success: true,
            data: Some(GraphData { nodes, edges }),
            message: Some(format!("Retrieved {} nodes and {} relationships", nodes.len(), edges.len())),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(e) => {
            tracing::error!("Failed to get graph data: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Error retrieving graph data: {}", e)),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}

/// Get neighbors of a specific node
pub async fn get_node_neighbors(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, StatusCode> {
    let graph_repo = &app_state.database.graph_repository;

    match graph_repo.get_related_nodes(id).await {
        Ok(neighbors) => {
            let neighbor_data: Vec<serde_json::Value> = neighbors.into_iter().map(|(id, name, ci_type, rel_type, attributes)| {
                serde_json::json!({
                    "id": id,
                    "name": name,
                    "ci_type": ci_type,
                    "relationship_type": rel_type,
                    "attributes": attributes,
                })
            }).collect();

            Ok(Json(ApiResponse {
                success: true,
                data: Some(neighbor_data),
                message: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get node neighbors for {}: {}", id, e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Error retrieving node neighbors: {}", e)),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}

/// Search for nodes by name or type
pub async fn search_nodes(
    State(app_state): State<crate::AppState>,
    _auth: AuthContext,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<crate::database::repositories::GraphNode>>>, StatusCode> {
    let graph_repo = &app_state.database.graph_repository;

    match graph_repo.search_assets(&params.q, params.limit).await {
        Ok(results) => Ok(Json(ApiResponse {
            success: true,
            data: Some(results),
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })),
        Err(e) => {
            tracing::error!("Failed to search nodes: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Error searching nodes: {}", e)),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}