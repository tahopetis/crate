use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub old_values: Option<Value>,
    pub new_values: Option<Value>,
    pub performed_by: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAuditLogRequest {
    #[validate(length(min = 1, max = 100))]
    pub entity_type: String,

    pub entity_id: Uuid,

    #[validate(length(min = 1, max = 50))]
    pub action: String,

    pub old_values: Option<Value>,
    pub new_values: Option<Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}