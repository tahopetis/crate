use crate::{
    error::{AppError, AppResult},
    models::{
        LifecycleType, LifecycleState, LifecycleTransition, CITypeLifecycleMapping,
        CreateLifecycleTypeRequest, UpdateLifecycleTypeRequest,
        CreateLifecycleStateRequest, UpdateLifecycleStateRequest,
        CreateLifecycleTransitionRequest, CreateCITypeLifecycleRequest,
        LifecycleTypeResponse, LifecycleTypeSummary,
    },
    database::{LifecycleRepository, CIRepository},
    middleware::AuthContext,
};
use validator::Validate;
use uuid::Uuid;

pub struct LifecycleService {
    lifecycle_repository: LifecycleRepository,
    ci_repository: CIRepository,
}

impl LifecycleService {
    pub fn new(lifecycle_repository: LifecycleRepository, ci_repository: CIRepository) -> Self {
        Self {
            lifecycle_repository,
            ci_repository,
        }
    }

    // Lifecycle Types Management
    pub async fn create_lifecycle_type(
        &self,
        request: CreateLifecycleTypeRequest,
        auth_context: &AuthContext,
    ) -> AppResult<LifecycleType> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::validation(format!("Invalid lifecycle type request: {}", e))
        })?;

        // Check for duplicate name
        let existing = self.lifecycle_repository.list_lifecycle_types(false).await?;
        if existing.iter().any(|lt| lt.name == request.name) {
            return Err(AppError::validation(
                "Lifecycle type with this name already exists".to_string(),
            ));
        }

        // Create lifecycle type
        self.lifecycle_repository
            .create_lifecycle_type(&request, auth_context.user_id)
            .await
    }

    pub async fn get_lifecycle_type(&self, id: Uuid) -> AppResult<LifecycleTypeResponse> {
        let lifecycle_type = self
            .lifecycle_repository
            .get_lifecycle_type_with_details(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        Ok(lifecycle_type)
    }

    pub async fn list_lifecycle_types(&self, include_inactive: bool) -> AppResult<Vec<LifecycleTypeSummary>> {
        self.lifecycle_repository
            .list_lifecycle_types(include_inactive)
            .await
    }

    pub async fn update_lifecycle_type(
        &self,
        id: Uuid,
        request: UpdateLifecycleTypeRequest,
    ) -> AppResult<LifecycleType> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::validation(format!("Invalid lifecycle type update request: {}", e))
        })?;

        // Check if lifecycle type exists
        let existing = self
            .lifecycle_repository
            .get_lifecycle_type(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        // If updating name, check for duplicates
        if let Some(ref name) = request.name {
            if name != &existing.name {
                let all_types = self.lifecycle_repository.list_lifecycle_types(false).await?;
                if all_types.iter().any(|lt| lt.name == *name) {
                    return Err(AppError::validation(
                        "Lifecycle type with this name already exists".to_string(),
                    ));
                }
            }
        }

        self.lifecycle_repository
            .update_lifecycle_type(id, &request)
            .await
    }

    pub async fn delete_lifecycle_type(&self, id: Uuid, auth_context: &AuthContext) -> AppResult<()> {
        // Check if lifecycle type exists
        let lifecycle_type = self
            .lifecycle_repository
            .get_lifecycle_type(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        // Check if lifecycle type is being used by any CI types
        let lifecycles_for_ci_type = self
            .lifecycle_repository
            .get_lifecycles_for_ci_type(id) // This is wrong, need a method to check usage
            .await?;

        // TODO: Implement proper usage check when we have the method
        // For now, allow deletion

        self.lifecycle_repository
            .delete_lifecycle_type(id)
            .await?;

        Ok(())
    }

    // Lifecycle States Management
    pub async fn create_lifecycle_state(
        &self,
        request: CreateLifecycleStateRequest,
        auth_context: &AuthContext,
    ) -> AppResult<LifecycleState> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::validation(format!("Invalid lifecycle state request: {}", e))
        })?;

        // Check if lifecycle type exists
        let lifecycle_type = self
            .lifecycle_repository
            .get_lifecycle_type(request.lifecycle_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        // Check for duplicate state name within the same lifecycle type
        let lifecycle_details = self
            .lifecycle_repository
            .get_lifecycle_type_with_details(request.lifecycle_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        if lifecycle_details
            .states
            .iter()
            .any(|state| state.name == request.name)
        {
            return Err(AppError::validation(
                "State with this name already exists in this lifecycle type".to_string(),
            ));
        }

        // Check for duplicate order index
        if lifecycle_details
            .states
            .iter()
            .any(|state| state.order_index == request.order_index)
        {
            return Err(AppError::validation(
                "State with this order index already exists in this lifecycle type".to_string(),
            ));
        }

        // If setting as initial state, ensure no other initial state exists
        if request.is_initial_state.unwrap_or(false) {
            if lifecycle_details
                .states
                .iter()
                .any(|state| state.is_initial_state)
            {
                return Err(AppError::validation(
                    "An initial state already exists for this lifecycle type".to_string(),
                ));
            }
        }

        self.lifecycle_repository
            .create_lifecycle_state(&request)
            .await
    }

    pub async fn get_lifecycle_state(&self, id: Uuid) -> AppResult<LifecycleState> {
        self.lifecycle_repository
            .get_lifecycle_state(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle state not found"))
    }

    pub async fn update_lifecycle_state(
        &self,
        id: Uuid,
        request: UpdateLifecycleStateRequest,
    ) -> AppResult<LifecycleState> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::validation(format!("Invalid lifecycle state update request: {}", e))
        })?;

        // Check if state exists
        let existing_state = self
            .lifecycle_repository
            .get_lifecycle_state(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle state not found"))?;

        // Get lifecycle type details for validation
        let lifecycle_details = self
            .lifecycle_repository
            .get_lifecycle_type_with_details(existing_state.lifecycle_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        // If updating name, check for duplicates
        if let Some(ref name) = request.name {
            if name != &existing_state.name {
                if lifecycle_details
                    .states
                    .iter()
                    .any(|state| state.name == *name && state.id != id)
                {
                    return Err(AppError::validation(
                        "State with this name already exists in this lifecycle type".to_string(),
                    ));
                }
            }
        }

        // If updating order index, check for duplicates
        if let Some(order_index) = request.order_index {
            if lifecycle_details
                .states
                .iter()
                .any(|state| state.order_index == order_index && state.id != id)
            {
                return Err(AppError::validation(
                    "State with this order index already exists in this lifecycle type".to_string(),
                ));
            }
        }

        // If setting as initial state, ensure no other initial state exists
        if request.is_initial_state.unwrap_or(false) && !existing_state.is_initial_state {
            if lifecycle_details
                .states
                .iter()
                .any(|state| state.is_initial_state && state.id != id)
            {
                return Err(AppError::validation(
                    "An initial state already exists for this lifecycle type".to_string(),
                ));
            }
        }

        self.lifecycle_repository
            .update_lifecycle_state(id, &request)
            .await
    }

    pub async fn delete_lifecycle_state(&self, id: Uuid, auth_context: &AuthContext) -> AppResult<()> {
        // Check if state exists
        let existing_state = self
            .lifecycle_repository
            .get_lifecycle_state(id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle state not found"))?;

        // Check if state is being used in any transitions
        let lifecycle_details = self
            .lifecycle_repository
            .get_lifecycle_type_with_details(existing_state.lifecycle_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        let is_used_in_transitions = lifecycle_details.transitions.iter().any(|transition| {
            transition.from_state_id == Some(id) || transition.to_state_id == id
        });

        if is_used_in_transitions {
            return Err(AppError::validation(
                "Cannot delete state that is used in transitions".to_string(),
            ));
        }

        self.lifecycle_repository
            .delete_lifecycle_state(id)
            .await?;

        Ok(())
    }

    // CI Type to Lifecycle Type Mapping
    pub async fn create_ci_type_lifecycle_mapping(
        &self,
        request: CreateCITypeLifecycleRequest,
        auth_context: &AuthContext,
    ) -> AppResult<CITypeLifecycleMapping> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::validation(format!("Invalid CI type lifecycle mapping request: {}", e))
        })?;

        // Check if CI type exists
        self.ci_repository
            .get_ci_type_by_id(request.ci_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("CI type not found"))?;

        // Check if lifecycle type exists
        self.lifecycle_repository
            .get_lifecycle_type(request.lifecycle_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("Lifecycle type not found"))?;

        // If setting as default, remove existing default for this CI type
        if request.is_default.unwrap_or(false) {
            // TODO: Implement method to remove default mapping
        }

        self.lifecycle_repository
            .create_ci_type_lifecycle_mapping(&request, auth_context.user_id)
            .await
    }

    pub async fn get_lifecycles_for_ci_type(&self, ci_type_id: Uuid) -> AppResult<Vec<LifecycleTypeSummary>> {
        // Check if CI type exists
        self.ci_repository
            .get_ci_type_by_id(ci_type_id)
            .await?
            .ok_or_else(|| AppError::not_found("CI type not found"))?;

        self.lifecycle_repository
            .get_lifecycles_for_ci_type(ci_type_id)
            .await
    }
}