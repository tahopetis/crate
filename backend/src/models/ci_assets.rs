use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIAsset {
    pub id: Uuid,
    pub ci_type_id: Uuid,
    pub name: String,
    pub attributes: Value,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCIAssetRequest {
    pub ci_type_id: Uuid,

    #[validate(length(min = 1, max = 255))]
    pub name: String,

    pub attributes: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCIAssetRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    pub attributes: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct CIAssetResponse {
    pub id: Uuid,
    pub ci_type_id: Uuid,
    pub ci_type_name: String,
    pub name: String,
    pub attributes: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CIAssetFilter {
    pub ci_type_id: Option<Uuid>,
    pub name: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

impl From<(CIAsset, String)> for CIAssetResponse {
    fn from((asset, ci_type_name): (CIAsset, String)) -> Self {
        Self {
            id: asset.id,
            ci_type_id: asset.ci_type_id,
            ci_type_name,
            name: asset.name,
            attributes: asset.attributes,
            created_at: asset.created_at,
            updated_at: asset.updated_at,
            created_by: "User".to_string(), // Would be populated with actual user data
            updated_by: None,               // Would be populated with actual user data
        }
    }
}