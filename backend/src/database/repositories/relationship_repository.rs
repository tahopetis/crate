use anyhow::Result;
use crate::models::{
    RelationshipType, CreateRelationshipTypeRequest,
    UpdateRelationshipTypeRequest, RelationshipTypeFilter, RelationshipTypeResponse,
    RelationshipTypeSummary
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct RelationshipRepository {
    pool: PgPool,
}

impl RelationshipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        request: &CreateRelationshipTypeRequest,
        created_by: Uuid,
    ) -> Result<RelationshipType> {
        let attributes_schema = request.attributes_schema.clone().unwrap_or_default();

        let row = sqlx::query(
            r#"
            INSERT INTO relationship_types (
                name, description, from_ci_type_id, to_ci_type_id,
                is_bidirectional, reverse_name, attributes_schema, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id, name, description, from_ci_type_id, to_ci_type_id,
                is_bidirectional, reverse_name, attributes_schema, created_by,
                created_at, updated_at
            "#
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.from_ci_type_id)
        .bind(request.to_ci_type_id)
        .bind(request.is_bidirectional)
        .bind(&request.reverse_name)
        .bind(attributes_schema)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(RelationshipType {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            from_ci_type_id: row.get("from_ci_type_id"),
            to_ci_type_id: row.get("to_ci_type_id"),
            is_bidirectional: row.get("is_bidirectional"),
            reverse_name: row.get("reverse_name"),
            attributes_schema: row.get("attributes_schema"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<RelationshipType>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, name, description, from_ci_type_id, to_ci_type_id,
                is_bidirectional, reverse_name, attributes_schema, created_by,
                created_at, updated_at
            FROM relationship_types
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(RelationshipType {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                from_ci_type_id: row.get("from_ci_type_id"),
                to_ci_type_id: row.get("to_ci_type_id"),
                is_bidirectional: row.get("is_bidirectional"),
                reverse_name: row.get("reverse_name"),
                attributes_schema: row.get("attributes_schema"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    pub async fn list(&self, filter: &RelationshipTypeFilter) -> Result<Vec<RelationshipTypeSummary>> {
        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);

        let mut query = String::from(
            r#"
            SELECT
                rt.id, rt.name, rt.description, rt.is_bidirectional, rt.reverse_name,
                from_ct.name as from_ci_type_name,
                to_ct.name as to_ci_type_name,
                0 as relationship_count
            FROM relationship_types rt
            LEFT JOIN ci_types from_ct ON rt.from_ci_type_id = from_ct.id
            LEFT JOIN ci_types to_ct ON rt.to_ci_type_id = to_ct.id
            WHERE rt.deleted_at IS NULL
            "#
        );

        let mut param_count = 0;

        if let Some(_search) = &filter.search {
            param_count += 1;
            query.push_str(&format!(" AND (rt.name ILIKE ${} OR rt.description ILIKE ${}) ", param_count, param_count + 1));
            param_count += 1;
        }

        if let Some(_from_ci_type_id) = filter.from_ci_type_id {
            param_count += 1;
            query.push_str(&format!(" AND rt.from_ci_type_id = ${} ", param_count));
        }

        if let Some(_to_ci_type_id) = filter.to_ci_type_id {
            param_count += 1;
            query.push_str(&format!(" AND rt.to_ci_type_id = ${} ", param_count));
        }

        if let Some(_is_bidirectional) = filter.is_bidirectional {
            param_count += 1;
            query.push_str(&format!(" AND rt.is_bidirectional = ${} ", param_count));
        }

        query.push_str(" ORDER BY rt.name ASC ");
        query.push_str(&format!(" LIMIT {} OFFSET {} ", limit, offset));

        // Prepare search patterns if needed
        let (search_pattern1, search_pattern2) = if let Some(search) = &filter.search {
            let sp1 = format!("%{}%", search);
            let sp2 = sp1.clone();
            (Some(sp1), Some(sp2))
        } else {
            (None, None)
        };

        let mut query_builder = sqlx::query(&query);

        if let Some(ref sp1) = search_pattern1 {
            query_builder = query_builder.bind(sp1);
        }
        if let Some(ref sp2) = search_pattern2 {
            query_builder = query_builder.bind(sp2);
        }
        if let Some(from_ci_type_id) = filter.from_ci_type_id {
            query_builder = query_builder.bind(from_ci_type_id);
        }
        if let Some(to_ci_type_id) = filter.to_ci_type_id {
            query_builder = query_builder.bind(to_ci_type_id);
        }
        if let Some(is_bidirectional) = filter.is_bidirectional {
            query_builder = query_builder.bind(is_bidirectional);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await?;

        let relationship_types = rows.into_iter().map(|row| {
            RelationshipTypeSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                is_bidirectional: row.get("is_bidirectional"),
                reverse_name: row.get("reverse_name"),
                from_ci_type_name: row.get("from_ci_type_name"),
                to_ci_type_name: row.get("to_ci_type_name"),
                relationship_count: row.get("relationship_count"),
            }
        }).collect();

        Ok(relationship_types)
    }

    pub async fn update(
        &self,
        id: Uuid,
        request: &UpdateRelationshipTypeRequest,
    ) -> Result<RelationshipType> {
        // Build dynamic update query - simplified version
        let mut updates = Vec::new();
        let mut params = Vec::new();
        let mut param_count = 0;

        if let Some(name) = &request.name {
            param_count += 1;
            updates.push(format!("name = ${}", param_count));
            params.push(Box::new(name.clone()) as Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>);
        }

        if let Some(description) = &request.description {
            param_count += 1;
            updates.push(format!("description = ${}", param_count));
            params.push(Box::new(description.clone()) as Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>);
        }

        if updates.is_empty() {
            return Err(anyhow::anyhow!("No fields to update"));
        }

        let query = format!(
            "UPDATE relationship_types SET {} WHERE id = ${} AND deleted_at IS NULL RETURNING *",
            updates.join(", "),
            param_count + 1
        );

        // For simplicity, just update basic fields for now
        let row = sqlx::query(
            r#"
            UPDATE relationship_types
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                updated_at = NOW()
            WHERE id = $3 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(RelationshipType {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            from_ci_type_id: row.get("from_ci_type_id"),
            to_ci_type_id: row.get("to_ci_type_id"),
            is_bidirectional: row.get("is_bidirectional"),
            reverse_name: row.get("reverse_name"),
            attributes_schema: row.get("attributes_schema"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            "UPDATE relationship_types SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Relationship type not found"));
        }

        Ok(())
    }

    pub async fn check_name_exists(&self, name: &str, exclude_id: Option<Uuid>) -> Result<bool> {
        let count: i64 = if let Some(exclude_id) = exclude_id {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM relationship_types WHERE name = $1 AND id != $2 AND deleted_at IS NULL"
            )
            .bind(name)
            .bind(exclude_id)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM relationship_types WHERE name = $1 AND deleted_at IS NULL"
            )
            .bind(name)
            .fetch_one(&self.pool)
            .await?
        };

        Ok(count > 0)
    }
}