use anyhow::Result;
use crate::models::{
    RelationshipType, CreateRelationshipTypeRequest,
    UpdateRelationshipTypeRequest, RelationshipTypeFilter, RelationshipTypeResponse,
    RelationshipTypeSummary,
    Relationship, CreateRelationshipRequest, UpdateRelationshipRequest,
    RelationshipFilter, RelationshipResponse, RelationshipWithDetails
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

    // === Relationship Instance Methods (Phase 3.1) ===

    /// Create a new relationship instance between two CI assets
    pub async fn create_relationship(
        &self,
        request: &CreateRelationshipRequest,
        created_by: Uuid,
    ) -> Result<Relationship> {
        let attributes = request.attributes.clone().unwrap_or_default();

        let row = sqlx::query(
            r#"
            INSERT INTO relationships (
                relationship_type_id, from_ci_asset_id, to_ci_asset_id, attributes, created_by
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, relationship_type_id, from_ci_asset_id, to_ci_asset_id,
                     attributes, created_by, created_at, updated_at
            "#
        )
        .bind(request.relationship_type_id)
        .bind(request.from_ci_asset_id)
        .bind(request.to_ci_asset_id)
        .bind(attributes)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(Relationship {
            id: row.get("id"),
            relationship_type_id: row.get("relationship_type_id"),
            from_ci_asset_id: row.get("from_ci_asset_id"),
            to_ci_asset_id: row.get("to_ci_asset_id"),
            attributes: row.get("attributes"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Get a relationship by ID with full details
    pub async fn get_relationship_by_id(&self, id: Uuid) -> Result<Option<RelationshipWithDetails>> {
        let row = sqlx::query(
            r#"
            SELECT
                r.id, r.relationship_type_id, r.from_ci_asset_id, r.to_ci_asset_id,
                r.attributes, r.created_by, r.created_at, r.updated_at,
                rt.name as relationship_type_name, rt.is_bidirectional,
                from_asset.name as from_ci_asset_name,
                to_asset.name as to_ci_asset_name,
                from_type.name as from_ci_type_name,
                to_type.name as to_ci_type_name,
                u.first_name || ' ' || u.last_name as created_by_name
            FROM relationships r
            JOIN relationship_types rt ON r.relationship_type_id = rt.id
            JOIN ci_assets from_asset ON r.from_ci_asset_id = from_asset.id
            JOIN ci_assets to_asset ON r.to_ci_asset_id = to_asset.id
            JOIN ci_types from_type ON from_asset.ci_type_id = from_type.id
            JOIN ci_types to_type ON to_asset.ci_type_id = to_type.id
            JOIN users u ON r.created_by = u.id
            WHERE r.id = $1 AND r.deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(RelationshipWithDetails {
                id: row.get("id"),
                relationship_type_id: row.get("relationship_type_id"),
                relationship_type_name: row.get("relationship_type_name"),
                is_bidirectional: row.get("is_bidirectional"),
                from_ci_asset_id: row.get("from_ci_asset_id"),
                from_ci_asset_name: row.get("from_ci_asset_name"),
                from_ci_type_name: row.get("from_ci_type_name"),
                to_ci_asset_id: row.get("to_ci_asset_id"),
                to_ci_asset_name: row.get("to_ci_asset_name"),
                to_ci_type_name: row.get("to_ci_type_name"),
                attributes: row.get("attributes"),
                created_by: row.get("created_by"),
                created_by_name: row.get("created_by_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    /// List relationships with optional filtering
    pub async fn list_relationships(&self, filter: &RelationshipFilter) -> Result<Vec<RelationshipResponse>> {
        let limit = filter.limit.unwrap_or(100);
        let offset = filter.offset.unwrap_or(0);

        let mut where_clauses = vec!["r.deleted_at IS NULL"];
        let mut param_count = 0;

        if filter.relationship_type_id.is_some() {
            param_count += 1;
            where_clauses.push(&format!("r.relationship_type_id = ${}", param_count));
        }

        if filter.from_ci_asset_id.is_some() {
            param_count += 1;
            where_clauses.push(&format!("r.from_ci_asset_id = ${}", param_count));
        }

        if filter.to_ci_asset_id.is_some() {
            param_count += 1;
            where_clauses.push(&format!("r.to_ci_asset_id = ${}", param_count));
        }

        if filter.ci_asset_id.is_some() {
            param_count += 1;
            where_clauses.push(&format!("(r.from_ci_asset_id = ${} OR r.to_ci_asset_id = ${})", param_count, param_count));
        }

        let where_clause = where_clauses.join(" AND ");

        let query = format!(
            r#"
            SELECT
                r.id, r.relationship_type_id, r.from_ci_asset_id, r.to_ci_asset_id,
                r.attributes, r.created_by, r.created_at, r.updated_at,
                rt.name as relationship_type_name, rt.is_bidirectional,
                from_asset.name as from_ci_asset_name,
                to_asset.name as to_ci_asset_name,
                from_type.name as from_ci_type_name,
                to_type.name as to_ci_type_name,
                u.first_name || ' ' || u.last_name as created_by_name
            FROM relationships r
            JOIN relationship_types rt ON r.relationship_type_id = rt.id
            JOIN ci_assets from_asset ON r.from_ci_asset_id = from_asset.id
            JOIN ci_assets to_asset ON r.to_ci_asset_id = to_asset.id
            JOIN ci_types from_type ON from_asset.ci_type_id = from_type.id
            JOIN ci_types to_type ON to_asset.ci_type_id = to_type.id
            JOIN users u ON r.created_by = u.id
            WHERE {}
            ORDER BY r.created_at DESC
            LIMIT {} OFFSET {}
            "#,
            where_clause, limit, offset
        );

        let mut query_builder = sqlx::query(&query);

        if let Some(type_id) = filter.relationship_type_id {
            query_builder = query_builder.bind(type_id);
        }
        if let Some(from_id) = filter.from_ci_asset_id {
            query_builder = query_builder.bind(from_id);
        }
        if let Some(to_id) = filter.to_ci_asset_id {
            query_builder = query_builder.bind(to_id);
        }
        if let Some(asset_id) = filter.ci_asset_id {
            query_builder = query_builder.bind(asset_id);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await?;

        let relationships = rows.into_iter().map(|row| {
            RelationshipResponse {
                id: row.get("id"),
                relationship_type_id: row.get("relationship_type_id"),
                relationship_type_name: row.get("relationship_type_name"),
                is_bidirectional: row.get("is_bidirectional"),
                from_ci_asset_id: row.get("from_ci_asset_id"),
                from_ci_asset_name: row.get("from_ci_asset_name"),
                from_ci_type_name: row.get("from_ci_type_name"),
                to_ci_asset_id: row.get("to_ci_asset_id"),
                to_ci_asset_name: row.get("to_ci_asset_name"),
                to_ci_type_name: row.get("to_ci_type_name"),
                attributes: row.get("attributes"),
                created_by: row.get("created_by"),
                created_by_name: row.get("created_by_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        Ok(relationships)
    }

    /// Update a relationship's attributes
    pub async fn update_relationship(
        &self,
        id: Uuid,
        request: &UpdateRelationshipRequest,
    ) -> Result<Relationship> {
        let attributes = request.attributes.clone().unwrap_or_default();

        let row = sqlx::query(
            r#"
            UPDATE relationships
            SET attributes = $1, updated_at = NOW()
            WHERE id = $2 AND deleted_at IS NULL
            RETURNING id, relationship_type_id, from_ci_asset_id, to_ci_asset_id,
                     attributes, created_by, created_at, updated_at
            "#
        )
        .bind(attributes)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Relationship {
            id: row.get("id"),
            relationship_type_id: row.get("relationship_type_id"),
            from_ci_asset_id: row.get("from_ci_asset_id"),
            to_ci_asset_id: row.get("to_ci_asset_id"),
            attributes: row.get("attributes"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Delete a relationship (soft delete)
    pub async fn delete_relationship(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            "UPDATE relationships SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Relationship not found"));
        }

        Ok(())
    }

    /// Check if a relationship already exists between two assets
    pub async fn relationship_exists(
        &self,
        relationship_type_id: Uuid,
        from_asset_id: Uuid,
        to_asset_id: Uuid,
    ) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM relationships
            WHERE relationship_type_id = $1
              AND from_ci_asset_id = $2
              AND to_ci_asset_id = $3
              AND deleted_at IS NULL
            "#
        )
        .bind(relationship_type_id)
        .bind(from_asset_id)
        .bind(to_asset_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}