use crate::database::{PgPool, Neo4jPool, CIRepository, GraphRepository};
use crate::models::{CIType, CreateCITypeRequest, UpdateCITypeRequest, CreateCIAssetRequest, CITypeResponse};
use crate::error::{AppError, AppResult};
use anyhow::Result;
use serde_json::{json, Value};
use uuid::Uuid;
use validator::Validate;

pub struct CIService {
    ci_repository: CIRepository,
    graph_repository: GraphRepository,
}

impl CIService {
    pub fn new(ci_repository: CIRepository, graph_repository: GraphRepository) -> Self {
        Self {
            ci_repository,
            graph_repository,
        }
    }

    // CI Type operations

    pub async fn create_ci_type(&self, request: CreateCITypeRequest, user_id: Uuid) -> AppResult<CIType> {
        // Validate the request
        request.validate()
            .map_err(|e| AppError::validation(&e.to_string()))?;

        // Check if CI type with the same name already exists
        if let Some(_) = self.ci_repository.get_ci_type_by_name(&request.name).await? {
            return Err(AppError::conflict(&format!("CI type '{}' already exists", request.name)));
        }

        // Set default attributes if not provided
        let attributes = request.attributes.unwrap_or_else(|| json!({}));

        // Create the CI type
        let ci_type_id = self.ci_repository.create_ci_type(
            &request.name,
            request.description.as_deref(),
            &attributes,
            user_id,
        ).await?;

        // Retrieve the created CI type
        self.ci_repository.get_ci_type_by_id(ci_type_id)
            .await?
            .ok_or_else(|| AppError::internal("Failed to retrieve created CI type"))
    }

    pub async fn get_ci_type_by_id(&self, id: Uuid) -> AppResult<Option<CIType>> {
        Ok(self.ci_repository.get_ci_type_by_id(id).await?)
    }

    pub async fn get_ci_type_by_name(&self, name: &str) -> AppResult<Option<CIType>> {
        Ok(self.ci_repository.get_ci_type_by_name(name).await?)
    }

    pub async fn list_ci_types(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<CIType>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit < 1 || limit > 100 {
            return Err(AppError::validation("Limit must be between 1 and 100"));
        }

        if offset < 0 {
            return Err(AppError::validation("Offset must be non-negative"));
        }

        Ok(self.ci_repository.list_ci_types(limit, offset).await?)
    }

    pub async fn update_ci_type(&self, id: Uuid, request: UpdateCITypeRequest) -> AppResult<CIType> {
        // Validate the request
        request.validate()
            .map_err(|e| AppError::validation(&e.to_string()))?;

        // Check if CI type exists
        let existing_ci_type = self.ci_repository.get_ci_type_by_id(id).await?
            .ok_or_else(|| AppError::not_found(&format!("CI type with id '{}' not found", id)))?;

        // If updating name, check if new name already exists (and it's not the same CI type)
        if let Some(ref name) = request.name {
            if name != &existing_ci_type.name {
                if let Some(_) = self.ci_repository.get_ci_type_by_name(name).await? {
                    return Err(AppError::conflict(&format!("CI type '{}' already exists", name)));
                }
            }
        }

        // Update the CI type
        let updated = self.ci_repository.update_ci_type(
            id,
            request.name.as_deref(),
            request.description.as_deref(),
            request.attributes.as_ref(),
        ).await?;

        if !updated {
            return Err(AppError::internal("Failed to update CI type"));
        }

        // Retrieve the updated CI type
        self.ci_repository.get_ci_type_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal("Failed to retrieve updated CI type"))
    }

    pub async fn delete_ci_type(&self, id: Uuid) -> AppResult<()> {
        // Check if CI type exists
        let _existing_ci_type = self.ci_repository.get_ci_type_by_id(id).await?
            .ok_or_else(|| AppError::not_found(&format!("CI type with id '{}' not found", id)))?;

        // Check if there are any CI assets of this type
        // Note: This would need to be implemented in the repository
        // For now, we'll allow deletion

        // Delete the CI type (soft delete)
        let deleted = self.ci_repository.delete_ci_type(id).await?;

        if !deleted {
            return Err(AppError::internal("Failed to delete CI type"));
        }

        Ok(())
    }

    pub async fn count_ci_types(&self) -> AppResult<i64> {
        Ok(self.ci_repository.count_ci_types().await?)
    }

    // CI Asset operations (keep existing implementations)

    pub async fn create_ci_asset(&self, request: CreateCIAssetRequest, user_id: Uuid) -> AppResult<Uuid> {
        // Validate the request
        request.validate()
            .map_err(|e| AppError::validation(&e.to_string()))?;

        // Check if CI type exists
        let _ci_type = self.ci_repository.get_ci_type_by_id(request.ci_type_id).await?
            .ok_or_else(|| AppError::not_found(&format!("CI type with id '{}' not found", request.ci_type_id)))?;

        // Set default attributes if not provided
        let attributes = request.attributes.unwrap_or_else(|| json!({}));

        // Create the CI asset
        let asset_id = self.ci_repository.create_ci_asset(
            request.ci_type_id,
            &request.name,
            &attributes,
            user_id,
        ).await?;

        Ok(asset_id)
    }

    pub async fn get_ci_asset(&self, id: Uuid) -> AppResult<Option<(Uuid, String, Value, Uuid, Uuid)>> {
        Ok(self.ci_repository.get_ci_asset(id).await?)
    }

    pub async fn list_ci_assets(
        &self,
        ci_type_id: Option<Uuid>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<(Uuid, String, Value, Uuid)>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit < 1 || limit > 100 {
            return Err(AppError::validation("Limit must be between 1 and 100"));
        }

        if offset < 0 {
            return Err(AppError::validation("Offset must be non-negative"));
        }

        Ok(self.ci_repository.list_ci_assets(ci_type_id, limit, offset).await?)
    }

    pub async fn update_ci_asset(
        &self,
        id: Uuid,
        name: Option<&str>,
        attributes: Option<&Value>,
        user_id: Uuid,
    ) -> AppResult<bool> {
        // Check if CI asset exists
        let _existing_asset = self.ci_repository.get_ci_asset(id).await?
            .ok_or_else(|| AppError::not_found(&format!("CI asset with id '{}' not found", id)))?;

        Ok(self.ci_repository.update_ci_asset(id, name, attributes, user_id).await?)
    }

    pub async fn delete_ci_asset(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        // Check if CI asset exists
        let _existing_asset = self.ci_repository.get_ci_asset(id).await?
            .ok_or_else(|| AppError::not_found(&format!("CI asset with id '{}' not found", id)))?;

        let deleted = self.ci_repository.delete_ci_asset(id, user_id).await?;

        if !deleted {
            return Err(AppError::internal("Failed to delete CI asset"));
        }

        Ok(())
    }
}