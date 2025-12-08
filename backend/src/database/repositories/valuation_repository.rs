use crate::database::PgPool;
use anyhow::Result;
use sqlx::{postgres::PgRow, Row};
use std::str::FromStr;
use uuid::Uuid;
use rust_decimal::Decimal;
#[derive(Debug)]
pub struct ValuationRepository {
    pool: PgPool,
}

impl ValuationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_valuation_record(
        &self,
        ci_asset_id: Uuid,
        initial_value: Decimal,
        useful_life_years: i32,
        depreciation_method: &str,
        created_by: Uuid,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO valuation_records (
                id, ci_asset_id, initial_value, current_value, useful_life_years,
                depreciation_method, created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $3, $4, $5, $6, NOW(), NOW())
            "#
        )
        .bind(id)
        .bind(ci_asset_id)
        .bind(initial_value.to_string())
        .bind(useful_life_years)
        .bind(depreciation_method)
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn update_valuation(
        &self,
        ci_asset_id: Uuid,
        new_value: Decimal,
        depreciation_amount: Decimal,
        updated_by: Uuid,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE valuation_records
            SET current_value = $1, updated_by = $2, updated_at = NOW()
            WHERE ci_asset_id = $3
            "#
        )
        .bind(new_value.to_string())
        .bind(updated_by)
        .bind(ci_asset_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn create_amortization_entry(
        &self,
        valuation_id: Uuid,
        year: i32,
        opening_value: Decimal,
        depreciation_amount: Decimal,
        closing_value: Decimal,
        created_by: Uuid,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO amortization_entries (
                id, valuation_id, year, opening_value, depreciation_amount,
                closing_value, created_by, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            "#
        )
        .bind(id)
        .bind(valuation_id)
        .bind(year)
        .bind(opening_value.to_string())
        .bind(depreciation_amount.to_string())
        .bind(closing_value.to_string())
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_valuation_by_asset(&self, ci_asset_id: Uuid) -> Result<Option<(Uuid, Decimal, Decimal, i32, String)>> {
        let row = sqlx::query(
            r#"
            SELECT id, initial_value, current_value, useful_life_years, depreciation_method
            FROM valuation_records
            WHERE ci_asset_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(ci_asset_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r: PgRow| {
            let initial_val_str: String = r.get("initial_value");
            let current_val_str: String = r.get("current_value");
            (
                r.get("id"),
                Decimal::from_str(&initial_val_str).unwrap_or_default(),
                Decimal::from_str(&current_val_str).unwrap_or_default(),
                r.get("useful_life_years"),
                r.get("depreciation_method")
            )
        }))
    }

    pub async fn get_amortization_history(&self, valuation_id: Uuid) -> Result<Vec<(i32, Decimal, Decimal, Decimal, chrono::DateTime<chrono::Utc>)>> {
        let rows = sqlx::query(
            r#"
            SELECT year, opening_value, depreciation_amount, closing_value, created_at
            FROM amortization_entries
            WHERE valuation_id = $1
            ORDER BY year
            "#
        )
        .bind(valuation_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter()
            .map(|r: PgRow| {
                let opening_val_str: String = r.get("opening_value");
                let depreciation_str: String = r.get("depreciation_amount");
                let closing_str: String = r.get("closing_value");
                (
                    r.get("year"),
                    Decimal::from_str(&opening_val_str).unwrap_or_default(),
                    Decimal::from_str(&depreciation_str).unwrap_or_default(),
                    Decimal::from_str(&closing_str).unwrap_or_default(),
                    r.get("created_at")
                )
            })
            .collect())
    }

    pub async fn get_all_valuations(&self, limit: i64, offset: i64) -> Result<Vec<(Uuid, Uuid, Decimal, Decimal, i32, String, chrono::DateTime<chrono::Utc>)>> {
        let rows = sqlx::query(
            r#"
            SELECT
                vr.id, vr.ci_asset_id, vr.initial_value, vr.current_value,
                vr.useful_life_years, vr.depreciation_method, vr.created_at
            FROM valuation_records vr
            INNER JOIN ci_assets a ON vr.ci_asset_id = a.id
            WHERE a.deleted_at IS NULL
            ORDER BY vr.created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter()
            .map(|r: PgRow| {
                let initial_val_str: String = r.get("initial_value");
                let current_val_str: String = r.get("current_value");
                (
                    r.get("id"),
                    r.get("ci_asset_id"),
                    Decimal::from_str(&initial_val_str).unwrap_or_default(),
                    Decimal::from_str(&current_val_str).unwrap_or_default(),
                    r.get("useful_life_years"),
                    r.get("depreciation_method"),
                    r.get("created_at")
                )
            })
            .collect())
    }
}