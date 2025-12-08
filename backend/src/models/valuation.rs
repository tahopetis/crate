use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

pub struct CreateValuationRequest {
    pub ci_asset_id: Uuid,

    pub initial_value: Decimal,

    pub useful_life_years: i32,

    pub depreciation_method: String,

    pub purchase_date: Option<chrono::NaiveDate>,
}

impl CreateValuationRequest {
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        // Placeholder validation - TODO: fix later
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationRecord {
    pub id: Uuid,
    pub ci_asset_id: Uuid,
    pub initial_value: Decimal,
    pub current_value: Decimal,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    pub purchase_date: Option<chrono::NaiveDate>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmortizationEntry {
    pub id: Uuid,
    pub valuation_id: Uuid,
    pub year: i32,
    pub opening_value: Decimal,
    pub depreciation_amount: Decimal,
    pub closing_value: Decimal,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

