use crate::database::PgPool;
use crate::models::CIType;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct CIRepository {
    pool: PgPool,
}

impl CIRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // CI Type CRUD operations

    pub async fn create_ci_type(
        &self,
        name: &str,
        description: Option<&str>,
        attributes: &Value,
        created_by: Uuid,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO ci_types (id, name, description, attributes, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            "#
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(attributes)
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_ci_type_by_id(&self, id: Uuid) -> Result<Option<CIType>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, attributes, created_by, created_at, updated_at, deleted_at
            FROM ci_types
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r: PgRow| CIType {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            attributes: r.get("attributes"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn get_ci_type_by_name(&self, name: &str) -> Result<Option<CIType>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, attributes, created_by, created_at, updated_at, deleted_at
            FROM ci_types
            WHERE name = $1 AND deleted_at IS NULL
            "#
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r: PgRow| CIType {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            attributes: r.get("attributes"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn list_ci_types(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CIType>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, attributes, created_by, created_at, updated_at, deleted_at
            FROM ci_types
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter()
            .map(|r: PgRow| CIType {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                attributes: r.get("attributes"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
            })
            .collect())
    }

    pub async fn update_ci_type(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        attributes: Option<&Value>,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE ci_types
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                attributes = COALESCE($3, attributes),
                updated_at = NOW()
            WHERE id = $4 AND deleted_at IS NULL
            "#
        )
        .bind(name)
        .bind(description)
        .bind(attributes)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_ci_type(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE ci_types
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_ci_types(&self) -> Result<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM ci_types
            WHERE deleted_at IS NULL
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn create_ci_asset(
        &self,
        ci_type_id: Uuid,
        name: &str,
        attributes: &Value,
        created_by: Uuid,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO ci_assets (id, ci_type_id, name, attributes, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            "#
        )
        .bind(id)
        .bind(ci_type_id)
        .bind(name)
        .bind(attributes)
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_ci_asset(&self, id: Uuid) -> Result<Option<(Uuid, String, Value, Uuid, Uuid)>> {
        let row = sqlx::query(
            r#"
            SELECT a.id, a.name, a.attributes, a.ci_type_id, a.created_by
            FROM ci_assets a
            WHERE a.id = $1 AND a.deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r: PgRow| (
            r.get("id"),
            r.get("name"),
            r.get("attributes"),
            r.get("ci_type_id"),
            r.get("created_by")
        )))
    }

    pub async fn list_ci_assets(
        &self,
        ci_type_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(Uuid, String, Value, Uuid)>> {
        let rows = if let Some(type_id) = ci_type_id {
            sqlx::query(
                r#"
                SELECT id, name, attributes, ci_type_id
                FROM ci_assets
                WHERE ci_type_id = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(type_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT id, name, attributes, ci_type_id
                FROM ci_assets
                WHERE deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows.into_iter()
            .map(|r: PgRow| (
                r.get("id"),
                r.get("name"),
                r.get("attributes"),
                r.get("ci_type_id")
            ))
            .collect())
    }

    pub async fn update_ci_asset(
        &self,
        id: Uuid,
        name: Option<&str>,
        attributes: Option<&Value>,
        updated_by: Uuid,
    ) -> Result<bool> {
        let result = if let (Some(name), Some(attributes)) = (name, attributes) {
            sqlx::query(
                r#"
                UPDATE ci_assets
                SET name = $1, attributes = $2, updated_by = $3, updated_at = NOW()
                WHERE id = $4 AND deleted_at IS NULL
                "#
            )
            .bind(name)
            .bind(attributes)
            .bind(updated_by)
            .bind(id)
            .execute(&self.pool)
            .await?
        } else if let Some(name) = name {
            sqlx::query(
                r#"
                UPDATE ci_assets
                SET name = $1, updated_by = $2, updated_at = NOW()
                WHERE id = $3 AND deleted_at IS NULL
                "#
            )
            .bind(name)
            .bind(updated_by)
            .bind(id)
            .execute(&self.pool)
            .await?
        } else if let Some(attributes) = attributes {
            sqlx::query(
                r#"
                UPDATE ci_assets
                SET attributes = $1, updated_by = $2, updated_at = NOW()
                WHERE id = $3 AND deleted_at IS NULL
                "#
            )
            .bind(attributes)
            .bind(updated_by)
            .bind(id)
            .execute(&self.pool)
            .await?
        } else {
            return Ok(false);
        };

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_ci_asset(&self, id: Uuid, deleted_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE ci_assets
            SET deleted_at = NOW(), deleted_by = $1
            WHERE id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(deleted_by)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}