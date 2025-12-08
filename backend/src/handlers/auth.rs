use axum::{
    extract::{State, Request},
    http::StatusCode,
    response::Json,
};
use serde_json::json;

use crate::{
    error::{AppError, AppResult},
    models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse},
    middleware::{AuthContext, extract_auth_context},
    utils::{hash_password, verify_password, create_jwt, validate_password_strength},
};

pub async fn login(
    State(app_state): State<crate::AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    // This would typically query the database for the user
    // For now, we'll return a mock response

    // TODO: Implement actual database query and password verification
    let user_id = uuid::Uuid::new_v4();
    let token = create_jwt(
        user_id,
        &request.email,
        "Test",
        "User",
        false,
        &app_state.config.auth.jwt_secret,
        app_state.config.auth.jwt_expiration_hours,
    )?;

    let login_response = LoginResponse {
        user: UserResponse {
            id: user_id,
            email: request.email,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            is_admin: false,
            created_at: chrono::Utc::now(),
        },
        token,
        expires_in: app_state.config.auth.jwt_expiration_hours * 3600,
    };

    Ok(Json(login_response))
}

pub async fn register(
    State(app_state): State<crate::AppState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // Validate password strength
    validate_password_strength(&request.password)?;

    // Hash password
    let password_hash = hash_password(&request.password)?;

    // TODO: Save user to database
    let user_id = uuid::Uuid::new_v4();

    let response = json!({
        "message": "User created successfully",
        "user_id": user_id.to_string()
    });

    Ok(Json(response))
}

pub async fn get_current_user(
    request: Request,
) -> AppResult<Json<UserResponse>> {
    let auth_context = extract_auth_context(&request)?;

    // TODO: Fetch user from database
    let user_response = UserResponse {
        id: auth_context.user_id,
        email: auth_context.email,
        first_name: auth_context.first_name,
        last_name: auth_context.last_name,
        is_admin: auth_context.is_admin,
        created_at: chrono::Utc::now(),
    };

    Ok(Json(user_response))
}

pub async fn logout() -> AppResult<Json<serde_json::Value>> {
    // In a real implementation, you might want to invalidate the token
    // For JWT, this is typically done client-side by deleting the token
    let response = json!({
        "message": "Logged out successfully"
    });

    Ok(Json(response))
}