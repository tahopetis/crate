use crate::{
    error::{AppError, AppResult},
    models::{
        LifecycleType, LifecycleState, LifecycleTransition, CITypeLifecycleMapping,
        CreateLifecycleTypeRequest, UpdateLifecycleTypeRequest,
        CreateLifecycleStateRequest, UpdateLifecycleStateRequest,
        CreateLifecycleTransitionRequest, CreateCITypeLifecycleRequest,
        LifecycleTypeResponse, LifecycleTypeSummary,
    },
};
use sqlx::{postgres::PgRow, Row, PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct LifecycleRepository {
    pool: PgPool,
}

impl LifecycleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Lifecycle Types CRUD
    pub async fn create_lifecycle_type(
        &self,
        request: &CreateLifecycleTypeRequest,
        created_by: Uuid,
    ) -> AppResult<LifecycleType> {
        let id = Uuid::new_v4();
        let default_color = request.default_color.as_deref().unwrap_or("#6B7280");

        let row = sqlx::query(
            r#"
            INSERT INTO lifecycle_types (id, name, description, default_color, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING
                id, name, description, default_color,
                is_active, created_by, created_at, updated_at, deleted_at
            "#
        )
        .bind(id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(default_color)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create lifecycle type: {}", e)))?;

        Ok(LifecycleType {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            default_color: row.get("default_color"),
            is_active: row.get("is_active"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        })
    }

    pub async fn get_lifecycle_type(&self, id: Uuid) -> AppResult<Option<LifecycleType>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, name, description, default_color,
                is_active, created_by, created_at, updated_at, deleted_at
            FROM lifecycle_types
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get lifecycle type: {}", e)))?;

        Ok(row.map(|r: PgRow| LifecycleType {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            default_color: r.get("default_color"),
            is_active: r.get("is_active"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            deleted_at: r.get("deleted_at"),
        }))
    }

    pub async fn get_lifecycle_type_with_details(&self, id: Uuid) -> AppResult<Option<LifecycleTypeResponse>> {
        let lifecycle_type = match self.get_lifecycle_type(id).await? {
            Some(lt) => lt,
            None => return Ok(None),
        };

        // Get states
        let state_rows = sqlx::query(
            r#"
            SELECT
                id, lifecycle_type_id, name, description, color,
                order_index, is_initial_state, is_terminal_state, created_at, updated_at
            FROM lifecycle_states
            WHERE lifecycle_type_id = $1
            ORDER BY order_index
            "#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get lifecycle states: {}", e)))?;

        let states: Vec<LifecycleState> = state_rows.into_iter().map(|row: PgRow| LifecycleState {
            id: row.get("id"),
            lifecycle_type_id: row.get("lifecycle_type_id"),
            name: row.get("name"),
            description: row.get("description"),
            color: row.get("color"),
            order_index: row.get("order_index"),
            is_initial_state: row.get("is_initial_state"),
            is_terminal_state: row.get("is_terminal_state"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        // Get transitions
        let transition_rows = sqlx::query(
            r#"
            SELECT
                id, lifecycle_type_id, from_state_id, to_state_id,
                transition_name, description, requires_approval, created_at
            FROM lifecycle_transitions
            WHERE lifecycle_type_id = $1
            ORDER BY transition_name, from_state_id, to_state_id
            "#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get lifecycle transitions: {}", e)))?;

        let transitions: Vec<LifecycleTransition> = transition_rows.into_iter().map(|row: PgRow| LifecycleTransition {
            id: row.get("id"),
            lifecycle_type_id: row.get("lifecycle_type_id"),
            from_state_id: row.get("from_state_id"),
            to_state_id: row.get("to_state_id"),
            transition_name: row.get("transition_name"),
            description: row.get("description"),
            requires_approval: row.get("requires_approval"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(Some(LifecycleTypeResponse {
            lifecycle_type,
            states,
            transitions,
        }))
    }

    pub async fn list_lifecycle_types(&self, include_inactive: bool) -> AppResult<Vec<LifecycleTypeSummary>> {
        let sql = if include_inactive {
            r#"
            SELECT
                lt.id, lt.name, lt.description, lt.default_color,
                lt.is_active, lt.created_at,
                COALESCE(state_counts.state_count, 0) as state_count,
                COALESCE(ci_type_counts.ci_type_count, 0) as ci_type_count
            FROM lifecycle_types lt
            LEFT JOIN (
                SELECT lifecycle_type_id, COUNT(*) as state_count
                FROM lifecycle_states
                GROUP BY lifecycle_type_id
            ) state_counts ON lt.id = state_counts.lifecycle_type_id
            LEFT JOIN (
                SELECT lifecycle_type_id, COUNT(*) as ci_type_count
                FROM ci_type_lifecycles
                GROUP BY lifecycle_type_id
            ) ci_type_counts ON lt.id = ci_type_counts.lifecycle_type_id
            WHERE lt.deleted_at IS NULL
            ORDER BY lt.name
            "#
        } else {
            r#"
            SELECT
                lt.id, lt.name, lt.description, lt.default_color,
                lt.is_active, lt.created_at,
                COALESCE(state_counts.state_count, 0) as state_count,
                COALESCE(ci_type_counts.ci_type_count, 0) as ci_type_count
            FROM lifecycle_types lt
            LEFT JOIN (
                SELECT lifecycle_type_id, COUNT(*) as state_count
                FROM lifecycle_states
                GROUP BY lifecycle_type_id
            ) state_counts ON lt.id = state_counts.lifecycle_type_id
            LEFT JOIN (
                SELECT lifecycle_type_id, COUNT(*) as ci_type_count
                FROM ci_type_lifecycles
                GROUP BY lifecycle_type_id
            ) ci_type_counts ON lt.id = ci_type_counts.lifecycle_type_id
            WHERE lt.deleted_at IS NULL AND lt.is_active = true
            ORDER BY lt.name
            "#
        };

        let rows = sqlx::query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("Failed to list lifecycle types: {}", e)))?;

        let result: Vec<LifecycleTypeSummary> = rows.into_iter().map(|row: PgRow| LifecycleTypeSummary {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            default_color: row.get("default_color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            state_count: row.get("state_count"),
            ci_type_count: row.get("ci_type_count"),
        }).collect();

        Ok(result)
    }

    pub async fn update_lifecycle_type(
        &self,
        id: Uuid,
        request: &UpdateLifecycleTypeRequest,
    ) -> AppResult<LifecycleType> {
        // Use individual UPDATE statements for each field to keep it simple
        let mut updated = false;

        if let Some(ref name) = request.name {
            sqlx::query("UPDATE lifecycle_types SET name = $1, updated_at = NOW() WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle type name: {}", e)))?;
            updated = true;
        }

        if let Some(ref description) = request.description {
            sqlx::query("UPDATE lifecycle_types SET description = $1, updated_at = NOW() WHERE id = $2")
                .bind(description)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle type description: {}", e)))?;
            updated = true;
        }

        if let Some(ref default_color) = request.default_color {
            sqlx::query("UPDATE lifecycle_types SET default_color = $1, updated_at = NOW() WHERE id = $2")
                .bind(default_color)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle type default_color: {}", e)))?;
            updated = true;
        }

        if let Some(is_active) = request.is_active {
            sqlx::query("UPDATE lifecycle_types SET is_active = $1, updated_at = NOW() WHERE id = $2")
                .bind(is_active)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle type is_active: {}", e)))?;
            updated = true;
        }

        if !updated {
            return Err(AppError::validation("No fields to update".to_string()));
        }

        // Fetch the updated record
        self.get_lifecycle_type(id).await?.ok_or_else(|| {
            AppError::not_found("Lifecycle type not found after update".to_string())
        })
    }

    pub async fn delete_lifecycle_type(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("UPDATE lifecycle_types SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("Failed to delete lifecycle type: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    // Lifecycle States CRUD
    pub async fn create_lifecycle_state(
        &self,
        request: &CreateLifecycleStateRequest,
    ) -> AppResult<LifecycleState> {
        let id = Uuid::new_v4();
        let color = request.color.as_deref().unwrap_or("#6B7280");
        let is_initial_state = request.is_initial_state.unwrap_or(false);
        let is_terminal_state = request.is_terminal_state.unwrap_or(false);

        let row = sqlx::query(
            r#"
            INSERT INTO lifecycle_states (
                id, lifecycle_type_id, name, description, color,
                order_index, is_initial_state, is_terminal_state, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            RETURNING
                id, lifecycle_type_id, name, description, color,
                order_index, is_initial_state, is_terminal_state, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(request.lifecycle_type_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(color)
        .bind(request.order_index)
        .bind(is_initial_state)
        .bind(is_terminal_state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create lifecycle state: {}", e)))?;

        Ok(LifecycleState {
            id: row.get("id"),
            lifecycle_type_id: row.get("lifecycle_type_id"),
            name: row.get("name"),
            description: row.get("description"),
            color: row.get("color"),
            order_index: row.get("order_index"),
            is_initial_state: row.get("is_initial_state"),
            is_terminal_state: row.get("is_terminal_state"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_lifecycle_state(&self, id: Uuid) -> AppResult<Option<LifecycleState>> {
        let row = sqlx::query(
            r#"
            SELECT
                id, lifecycle_type_id, name, description, color,
                order_index, is_initial_state, is_terminal_state, created_at, updated_at
            FROM lifecycle_states
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get lifecycle state: {}", e)))?;

        Ok(row.map(|r: PgRow| LifecycleState {
            id: r.get("id"),
            lifecycle_type_id: r.get("lifecycle_type_id"),
            name: r.get("name"),
            description: r.get("description"),
            color: r.get("color"),
            order_index: r.get("order_index"),
            is_initial_state: r.get("is_initial_state"),
            is_terminal_state: r.get("is_terminal_state"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn update_lifecycle_state(
        &self,
        id: Uuid,
        request: &UpdateLifecycleStateRequest,
    ) -> AppResult<LifecycleState> {
        // Use individual UPDATE statements for each field to keep it simple
        let mut updated = false;

        if let Some(ref name) = request.name {
            sqlx::query("UPDATE lifecycle_states SET name = $1, updated_at = NOW() WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state name: {}", e)))?;
            updated = true;
        }

        if let Some(ref description) = request.description {
            sqlx::query("UPDATE lifecycle_states SET description = $1, updated_at = NOW() WHERE id = $2")
                .bind(description)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state description: {}", e)))?;
            updated = true;
        }

        if let Some(ref color) = request.color {
            sqlx::query("UPDATE lifecycle_states SET color = $1, updated_at = NOW() WHERE id = $2")
                .bind(color)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state color: {}", e)))?;
            updated = true;
        }

        if let Some(order_index) = request.order_index {
            sqlx::query("UPDATE lifecycle_states SET order_index = $1, updated_at = NOW() WHERE id = $2")
                .bind(order_index)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state order_index: {}", e)))?;
            updated = true;
        }

        if let Some(is_initial_state) = request.is_initial_state {
            sqlx::query("UPDATE lifecycle_states SET is_initial_state = $1, updated_at = NOW() WHERE id = $2")
                .bind(is_initial_state)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state is_initial_state: {}", e)))?;
            updated = true;
        }

        if let Some(is_terminal_state) = request.is_terminal_state {
            sqlx::query("UPDATE lifecycle_states SET is_terminal_state = $1, updated_at = NOW() WHERE id = $2")
                .bind(is_terminal_state)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::internal(format!("Failed to update lifecycle state is_terminal_state: {}", e)))?;
            updated = true;
        }

        if !updated {
            return Err(AppError::validation("No fields to update".to_string()));
        }

        // Fetch the updated record
        self.get_lifecycle_state(id).await?.ok_or_else(|| {
            AppError::not_found("Lifecycle state not found after update".to_string())
        })
    }

    pub async fn delete_lifecycle_state(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM lifecycle_states WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("Failed to delete lifecycle state: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    // CI Type to Lifecycle Type mapping
    pub async fn create_ci_type_lifecycle_mapping(
        &self,
        request: &CreateCITypeLifecycleRequest,
        created_by: Uuid,
    ) -> AppResult<CITypeLifecycleMapping> {
        let id = Uuid::new_v4();
        let is_default = request.is_default.unwrap_or(false);

        let row = sqlx::query(
            r#"
            INSERT INTO ci_type_lifecycles (id, ci_type_id, lifecycle_type_id, is_default, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (ci_type_id, lifecycle_type_id)
            DO UPDATE SET is_default = EXCLUDED.is_default
            RETURNING id, ci_type_id, lifecycle_type_id, is_default, created_by, created_at
            "#
        )
        .bind(id)
        .bind(request.ci_type_id)
        .bind(request.lifecycle_type_id)
        .bind(is_default)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create CI type lifecycle mapping: {}", e)))?;

        Ok(CITypeLifecycleMapping {
            id: row.get("id"),
            ci_type_id: row.get("ci_type_id"),
            lifecycle_type_id: row.get("lifecycle_type_id"),
            is_default: row.get("is_default"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn get_lifecycles_for_ci_type(&self, ci_type_id: Uuid) -> AppResult<Vec<LifecycleTypeSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                lt.id, lt.name, lt.description, lt.default_color,
                lt.is_active, lt.created_at,
                COALESCE(state_counts.state_count, 0) as state_count,
                1 as ci_type_count
            FROM ci_type_lifecycles ctl
            JOIN lifecycle_types lt ON ctl.lifecycle_type_id = lt.id
            LEFT JOIN (
                SELECT lifecycle_type_id, COUNT(*) as state_count
                FROM lifecycle_states
                GROUP BY lifecycle_type_id
            ) state_counts ON lt.id = state_counts.lifecycle_type_id
            WHERE ctl.ci_type_id = $1 AND lt.deleted_at IS NULL AND lt.is_active = true
            ORDER BY ctl.is_default DESC, lt.name
            "#
        )
        .bind(ci_type_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to get lifecycles for CI type: {}", e)))?;

        let result: Vec<LifecycleTypeSummary> = rows.into_iter().map(|row: PgRow| LifecycleTypeSummary {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            default_color: row.get("default_color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            state_count: row.get("state_count"),
            ci_type_count: row.get("ci_type_count"),
        }).collect();

        Ok(result)
    }
}