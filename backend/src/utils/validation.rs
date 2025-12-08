use crate::{error::AppError, models::CreateCITypeRequest};
use validator::Validate;

pub fn validate_ci_type(request: &CreateCITypeRequest) -> Result<(), AppError> {
    request.validate().map_err(|e| AppError::validation(format!("Validation failed: {}", e)))
}

pub fn validate_ci_asset(request: &crate::models::CreateCIAssetRequest) -> Result<(), AppError> {
    request.validate().map_err(|e| AppError::validation(format!("Validation failed: {}", e)))
}

pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::validation("Password must be at least 8 characters long".to_string()));
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(AppError::validation("Password must contain at least one uppercase letter".to_string()));
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(AppError::validation("Password must contain at least one lowercase letter".to_string()));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::validation("Password must contain at least one digit".to_string()));
    }

    Ok(())
}