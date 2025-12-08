use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCITypeRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub attributes: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCITypeRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub attributes: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct CITypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String, // User email or name
}

impl From<(CIType, String)> for CITypeResponse {
    fn from((ci_type, created_by_name): (CIType, String)) -> Self {
        Self {
            id: ci_type.id,
            name: ci_type.name,
            description: ci_type.description,
            attributes: ci_type.attributes,
            created_at: ci_type.created_at,
            updated_at: ci_type.updated_at,
            created_by: created_by_name,
        }
    }
}