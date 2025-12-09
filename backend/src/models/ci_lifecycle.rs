use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Legacy asset lifecycle status tracking
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

// === Configurable Lifecycle Types Management ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_color: String,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLifecycleTypeRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub description: Option<String>,

    #[validate(length(min = 7, max = 7))]
    pub default_color: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLifecycleTypeRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub description: Option<String>,

    #[validate(length(min = 7, max = 7))]
    pub default_color: Option<String>,

    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleState {
    pub id: Uuid,
    pub lifecycle_type_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub order_index: i32,
    pub is_initial_state: bool,
    pub is_terminal_state: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLifecycleStateRequest {
    pub lifecycle_type_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub description: Option<String>,

    #[validate(length(min = 7, max = 7))]
    pub color: Option<String>,

    #[validate(range(min = 0))]
    pub order_index: i32,

    pub is_initial_state: Option<bool>,
    pub is_terminal_state: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLifecycleStateRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub description: Option<String>,

    #[validate(length(min = 7, max = 7))]
    pub color: Option<String>,

    #[validate(range(min = 0))]
    pub order_index: Option<i32>,

    pub is_initial_state: Option<bool>,
    pub is_terminal_state: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTransition {
    pub id: Uuid,
    pub lifecycle_type_id: Uuid,
    pub from_state_id: Option<Uuid>,
    pub to_state_id: Uuid,
    pub transition_name: Option<String>,
    pub description: Option<String>,
    pub requires_approval: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLifecycleTransitionRequest {
    pub lifecycle_type_id: Uuid,
    pub from_state_id: Option<Uuid>,
    pub to_state_id: Uuid,
    pub transition_name: Option<String>,
    pub description: Option<String>,
    pub requires_approval: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CITypeLifecycleMapping {
    pub id: Uuid,
    pub ci_type_id: Uuid,
    pub lifecycle_type_id: Uuid,
    pub is_default: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCITypeLifecycleRequest {
    pub ci_type_id: Uuid,
    pub lifecycle_type_id: Uuid,
    pub is_default: Option<bool>,
}

// Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTypeResponse {
    #[serde(flatten)]
    pub lifecycle_type: LifecycleType,
    pub states: Vec<LifecycleState>,
    pub transitions: Vec<LifecycleTransition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTypeSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_color: String,
    pub is_active: bool,
    pub state_count: i64,
    pub ci_type_count: i64,
    pub created_at: DateTime<Utc>,
}