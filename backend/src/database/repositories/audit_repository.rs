use crate::database::PgPool;
use anyhow::Result;
use serde_json::Value;
use sqlx::{postgres::PgRow, Row};
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuditRepository {
    pool: PgPool,
}

impl AuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_audit_log(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        action: &str,
        old_values: Option<&Value>,
        new_values: Option<&Value>,
        performed_by: Uuid,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();

        // Convert ip_address to a format compatible with INET type
        let ip_addr = ip_address
            .and_then(|addr| addr.parse::<IpAddr>().ok());

        sqlx::query(
            r#"
            INSERT INTO audit_log (
                id, entity_type, entity_id, action, old_values, new_values,
                performed_by, ip_address, user_agent, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
            "#
        )
        .bind(id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(action)
        .bind(old_values)
        .bind(new_values)
        .bind(performed_by)
        .bind(ip_addr as Option<IpAddr>)
        .bind(user_agent)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_audit_logs(
        &self,
        entity_type: Option<&str>,
        entity_id: Option<Uuid>,
        performed_by: Option<Uuid>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(
        Uuid,
        String,
        Uuid,
        String,
        Option<Value>,
        Option<Value>,
        Uuid,
        Option<IpAddr>,
        Option<String>,
        chrono::DateTime<chrono::Utc>
    )>> {
        // Simplified query - for production, you'd want proper parameterized queries
        let query = r#"
            SELECT
                id, entity_type, entity_id, action, old_values, new_values,
                performed_by, ip_address, user_agent, created_at
            FROM audit_log
            WHERE ($1::text IS NULL OR entity_type = $1)
            AND ($2::uuid IS NULL OR entity_id = $2)
            AND ($3::uuid IS NULL OR performed_by = $3)
            AND ($4::timestamp IS NULL OR created_at >= $4)
            AND ($5::timestamp IS NULL OR created_at <= $5)
            ORDER BY created_at DESC LIMIT $6 OFFSET $7
        "#;

        let rows = sqlx::query(query)
        .bind(entity_type)
        .bind(entity_id)
        .bind(performed_by)
        .bind(from_date)
        .bind(to_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter()
            .map(|r: PgRow| (
                r.get("id"),
                r.get("entity_type"),
                r.get("entity_id"),
                r.get("action"),
                r.get("old_values"),
                r.get("new_values"),
                r.get("performed_by"),
                r.get::<Option<IpAddr>, _>("ip_address"),
                r.get("user_agent"),
                r.get("created_at")
            ))
            .collect())
    }
}