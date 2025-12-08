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
    pub attributes_schema: Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelationshipTypeRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub from_ci_type_id: Option<Uuid>,
    pub to_ci_type_id: Option<Uuid>,

    pub attributes_schema: Option<Value>,
}