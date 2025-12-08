use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CILifecycle {
    pub id: Uuid,
    pub ci_asset_id: Uuid,
    pub status: String,
    pub status_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLifecycleRequest {
    pub ci_asset_id: Uuid,

    #[validate(length(min = 1, max = 50))]
    pub status: String,

    pub notes: Option<String>,
}