use anyhow::Result;
use validator::Validate;
use crate::database::repositories::{RelationshipRepository, CIRepository, GraphRepository};
use crate::models::{
    RelationshipType, CreateRelationshipTypeRequest,
    UpdateRelationshipTypeRequest, RelationshipTypeFilter, RelationshipTypeResponse,
    RelationshipTypeSummary, CIType
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
}