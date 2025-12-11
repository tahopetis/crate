use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,
    pub is_bidirectional: bool,
    pub reverse_name: Option<String>,
    pub attributes_schema: Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTypeWithDetails {
    #[serde(flatten)]
    pub relationship_type: RelationshipType,
    pub from_ci_type_name: Option<String>,
    pub to_ci_type_name: Option<String>,
    pub created_by_name: String,
    pub relationship_count: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelationshipTypeRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,

    #[serde(default)]
    pub is_bidirectional: bool,

    #[validate(length(max = 255))]
    pub reverse_name: Option<String>,

    pub attributes_schema: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRelationshipTypeRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,

    #[serde(default)]
    pub is_bidirectional: Option<bool>,

    #[validate(length(max = 255))]
    pub reverse_name: Option<String>,

    pub attributes_schema: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTypeFilter {
    pub search: Option<String>,
    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,
    pub is_bidirectional: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,
    pub is_bidirectional: bool,
    pub reverse_name: Option<String>,
    pub attributes_schema: Value,
    pub created_by: Uuid,
    pub created_by_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub relationship_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTypeSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_bidirectional: bool,
    pub reverse_name: Option<String>,
    pub from_ci_type_name: Option<String>,
    pub to_ci_type_name: Option<String>,
    pub relationship_count: i64,
}

// Relationship Instance Models (Phase 3.1)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub relationship_type_id: Uuid,
    pub from_ci_asset_id: Uuid,
    pub to_ci_asset_id: Uuid,
    pub attributes: Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipWithDetails {
    pub id: Uuid,
    pub relationship_type_id: Uuid,
    pub relationship_type_name: String,
    pub is_bidirectional: bool,
    pub from_ci_asset_id: Uuid,
    pub from_ci_asset_name: String,
    pub from_ci_type_name: String,
    pub to_ci_asset_id: Uuid,
    pub to_ci_asset_name: String,
    pub to_ci_type_name: String,
    pub attributes: Value,
    pub created_by: Uuid,
    pub created_by_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelationshipRequest {
    pub relationship_type_id: Uuid,
    pub from_ci_asset_id: Uuid,
    pub to_ci_asset_id: Uuid,
    pub attributes: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRelationshipRequest {
    pub attributes: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipFilter {
    pub relationship_type_id: Option<Uuid>,
    pub ci_asset_id: Option<Uuid>, // Find all relationships for this asset (from or to)
    pub from_ci_asset_id: Option<Uuid>,
    pub to_ci_asset_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipResponse {
    pub id: Uuid,
    pub relationship_type_id: Uuid,
    pub relationship_type_name: String,
    pub is_bidirectional: bool,
    pub from_ci_asset_id: Uuid,
    pub from_ci_asset_name: String,
    pub from_ci_type_name: String,
    pub to_ci_asset_id: Uuid,
    pub to_ci_asset_name: String,
    pub to_ci_type_name: String,
    pub attributes: Value,
    pub created_by: Uuid,
    pub created_by_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}