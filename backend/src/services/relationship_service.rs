use anyhow::Result;
use validator::Validate;
use crate::database::repositories::{RelationshipRepository, CIRepository, GraphRepository};
use crate::models::{
    RelationshipType, CreateRelationshipTypeRequest,
    UpdateRelationshipTypeRequest, RelationshipTypeFilter, RelationshipTypeResponse,
    RelationshipTypeSummary, CIType,
    Relationship, CreateRelationshipRequest, UpdateRelationshipRequest,
    RelationshipFilter, RelationshipResponse, RelationshipWithDetails
};
use uuid::Uuid;
use std::sync::Arc;

pub struct RelationshipService {
    relationship_repository: RelationshipRepository,
    ci_repository: CIRepository,
    graph_repository: Arc<GraphRepository>,
}

impl RelationshipService {
    pub fn new(
        relationship_repository: RelationshipRepository,
        ci_repository: CIRepository,
        graph_repository: Arc<GraphRepository>,
    ) -> Self {
        Self {
            relationship_repository,
            ci_repository,
            graph_repository,
        }
    }

    pub async fn create_relationship_type(
        &self,
        request: CreateRelationshipTypeRequest,
        user_id: Uuid,
    ) -> Result<RelationshipType> {
        // Validate request
        if let Err(validation_errors) = request.validate() {
            return Err(anyhow::anyhow!("Validation failed: {}", validation_errors));
        }

        // Check if name already exists
        if self
            .relationship_repository
            .check_name_exists(&request.name, None)
            .await?
        {
            return Err(anyhow::anyhow!("Relationship type with this name already exists"));
        }

        // Validate CI types if specified
        if let Some(from_ci_type_id) = request.from_ci_type_id {
            if self
                .ci_repository
                .get_ci_type_by_id(from_ci_type_id)
                .await?
                .is_none()
            {
                return Err(anyhow::anyhow!("Source CI type not found"));
            }
        }

        if let Some(to_ci_type_id) = request.to_ci_type_id {
            if self
                .ci_repository
                .get_ci_type_by_id(to_ci_type_id)
                .await?
                .is_none()
            {
                return Err(anyhow::anyhow!("Target CI type not found"));
            }
        }

        // Validate bidirectional constraints
        if request.is_bidirectional && (request.reverse_name.is_none() || request.reverse_name.as_ref().unwrap().is_empty()) {
            return Err(anyhow::anyhow!("Reverse name is required for bidirectional relationships"));
        }

        // Validate no self-relationship
        if let (Some(from_id), Some(to_id)) = (request.from_ci_type_id, request.to_ci_type_id) {
            if from_id == to_id {
                return Err(anyhow::anyhow!("Self-relationships are not allowed"));
            }
        }

        let relationship_type = self
            .relationship_repository
            .create(&request, user_id)
            .await?;

        // Initialize Neo4j constraints for this relationship type
        let (from_ci_type_name, to_ci_type_name) = match (request.from_ci_type_id, request.to_ci_type_id) {
            (Some(from_id), Some(to_id)) => {
                let from_type = self.ci_repository.get_ci_type_by_id(from_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Source CI type not found"))?;
                let to_type = self.ci_repository.get_ci_type_by_id(to_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Target CI type not found"))?;
                (Some(from_type.name), Some(to_type.name))
            }
            (Some(from_id), None) => {
                let from_type = self.ci_repository.get_ci_type_by_id(from_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Source CI type not found"))?;
                (Some(from_type.name), None)
            }
            (None, Some(to_id)) => {
                let to_type = self.ci_repository.get_ci_type_by_id(to_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Target CI type not found"))?;
                (None, Some(to_type.name))
            }
            (None, None) => (None, None)
        };

        // Initialize constraints in Neo4j
        if let Err(e) = self.graph_repository.initialize_relationship_constraints(
            &relationship_type.name,
            from_ci_type_name.as_deref(),
            to_ci_type_name.as_deref(),
            relationship_type.is_bidirectional,
        ).await {
            // Log error but don't fail the operation
            eprintln!("Warning: Failed to initialize Neo4j constraints for relationship type '{}': {}",
                     relationship_type.name, e);
        }

        Ok(relationship_type)
    }

    pub async fn get_relationship_type(&self, id: Uuid) -> Result<Option<RelationshipType>> {
        self.relationship_repository
            .get_by_id(id)
            .await
    }

    pub async fn list_relationship_types(
        &self,
        filter: RelationshipTypeFilter,
    ) -> Result<Vec<RelationshipTypeSummary>> {
        self.relationship_repository
            .list(&filter)
            .await
    }

    pub async fn update_relationship_type(
        &self,
        id: Uuid,
        request: UpdateRelationshipTypeRequest,
    ) -> Result<RelationshipType> {
        // Validate request
        if let Err(validation_errors) = request.validate() {
            return Err(anyhow::anyhow!("Validation failed: {}", validation_errors));
        }

        // Check if relationship type exists
        let existing = self
            .relationship_repository
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Relationship type not found"))?;

        // Check if name already exists (excluding current)
        if let Some(ref name) = request.name {
            if name != &existing.name
                && self
                    .relationship_repository
                    .check_name_exists(name, Some(id))
                    .await?
            {
                return Err(anyhow::anyhow!("Relationship type with this name already exists"));
            }
        }

        let updated = self
            .relationship_repository
            .update(id, &request)
            .await?;

        Ok(updated)
    }

    pub async fn delete_relationship_type(&self, id: Uuid) -> Result<()> {
        self.relationship_repository
            .delete(id)
            .await?;
        Ok(())
    }

    // === Relationship Instance Methods (Phase 3.1) ===

    /// Create a new relationship instance between two CI assets
    pub async fn create_relationship_instance(
        &self,
        request: CreateRelationshipRequest,
        user_id: Uuid,
    ) -> Result<RelationshipWithDetails> {
        // Validate request
        if let Err(validation_errors) = request.validate() {
            return Err(anyhow::anyhow!("Validation failed: {}", validation_errors));
        }

        // Validate that assets are not the same
        if request.from_ci_asset_id == request.to_ci_asset_id {
            return Err(anyhow::anyhow!("Cannot create relationship from an asset to itself"));
        }

        // Get relationship type to validate constraints
        let rel_type = self.relationship_repository
            .get_by_id(request.relationship_type_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Relationship type not found"))?;

        // Get the from and to assets
        let from_asset = self.ci_repository
            .get_ci_asset_by_id(request.from_ci_asset_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Source asset not found"))?;

        let to_asset = self.ci_repository
            .get_ci_asset_by_id(request.to_ci_asset_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Target asset not found"))?;

        // Validate relationship type constraints (if specified)
        if let Some(from_ci_type_id) = rel_type.from_ci_type_id {
            if from_asset.ci_type_id != from_ci_type_id {
                return Err(anyhow::anyhow!(
                    "Source asset type does not match relationship type constraint"
                ));
            }
        }

        if let Some(to_ci_type_id) = rel_type.to_ci_type_id {
            if to_asset.ci_type_id != to_ci_type_id {
                return Err(anyhow::anyhow!(
                    "Target asset type does not match relationship type constraint"
                ));
            }
        }

        // Check if relationship already exists
        if self.relationship_repository
            .relationship_exists(
                request.relationship_type_id,
                request.from_ci_asset_id,
                request.to_ci_asset_id,
            )
            .await?
        {
            return Err(anyhow::anyhow!(
                "Relationship already exists between these assets"
            ));
        }

        // Create relationship in PostgreSQL
        let relationship = self.relationship_repository
            .create_relationship(&request, user_id)
            .await?;

        // Get CI type names for Neo4j
        let from_ci_type = self.ci_repository
            .get_ci_type_by_id(from_asset.ci_type_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Source CI type not found"))?;

        let to_ci_type = self.ci_repository
            .get_ci_type_by_id(to_asset.ci_type_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Target CI type not found"))?;

        // Sync to Neo4j
        if let Err(e) = self.graph_repository
            .create_relationship(
                request.from_ci_asset_id,
                request.to_ci_asset_id,
                &rel_type.name,
                request.relationship_type_id,
                request.attributes.clone(),
                &from_ci_type.name,
                &to_ci_type.name,
                rel_type.is_bidirectional,
            )
            .await
        {
            tracing::warn!("Failed to sync relationship to Neo4j: {}", e);
            // Don't fail the request if Neo4j sync fails, just log it
        }

        // Get the full relationship details to return
        let relationship_details = self.relationship_repository
            .get_relationship_by_id(relationship.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve created relationship"))?;

        Ok(relationship_details)
    }

    /// Get a relationship by ID
    pub async fn get_relationship_instance(&self, id: Uuid) -> Result<Option<RelationshipWithDetails>> {
        self.relationship_repository
            .get_relationship_by_id(id)
            .await
    }

    /// List relationships with optional filtering
    pub async fn list_relationship_instances(
        &self,
        filter: RelationshipFilter,
    ) -> Result<Vec<RelationshipResponse>> {
        self.relationship_repository
            .list_relationships(&filter)
            .await
    }

    /// Update a relationship's attributes
    pub async fn update_relationship_instance(
        &self,
        id: Uuid,
        request: UpdateRelationshipRequest,
    ) -> Result<RelationshipWithDetails> {
        // Validate request
        if let Err(validation_errors) = request.validate() {
            return Err(anyhow::anyhow!("Validation failed: {}", validation_errors));
        }

        // Update in PostgreSQL
        self.relationship_repository
            .update_relationship(id, &request)
            .await?;

        // Get the updated relationship with full details
        let relationship_details = self.relationship_repository
            .get_relationship_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Relationship not found"))?;

        // Note: We don't update Neo4j for attribute changes since Neo4j relationships
        // are primarily for graph visualization and topology, not attribute storage

        Ok(relationship_details)
    }

    /// Delete a relationship
    pub async fn delete_relationship_instance(&self, id: Uuid) -> Result<()> {
        // Get relationship details before deletion
        let relationship = self.relationship_repository
            .get_relationship_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Relationship not found"))?;

        // Delete from PostgreSQL
        self.relationship_repository
            .delete_relationship(id)
            .await?;

        // Delete from Neo4j
        if let Err(e) = self.graph_repository
            .delete_relationship(
                relationship.from_ci_asset_id,
                relationship.to_ci_asset_id,
                relationship.relationship_type_id,
            )
            .await
        {
            tracing::warn!("Failed to delete relationship from Neo4j: {}", e);
            // Don't fail the request if Neo4j deletion fails, just log it
        }

        Ok(())
    }
}