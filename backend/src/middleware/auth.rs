use axum::{
    extract::{Request, State, FromRequestParts},
    http::{request::Parts, header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
    async_trait,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
    pub exp: usize,  // Expiration time as Unix timestamp
    pub iat: usize,  // Issued at time as Unix timestamp
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::authentication("No authorization header provided"))?;

        // Decode the JWT token
        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret("your-secret-key".as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AppError::authentication("Invalid token"))?;

        let claims = token_data.claims;

        Ok(AuthContext {
            user_id: Uuid::parse_str(&claims.sub)
                .map_err(|_| AppError::authentication("Invalid user ID in token"))?,
            email: claims.email,
            first_name: claims.first_name,
            last_name: claims.last_name,
            is_admin: claims.is_admin,
        })
    }
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let claims = token_data.claims;

    // Check if token is expired
    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Parse user ID from claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let auth_context = AuthContext {
        user_id,
        email: claims.email,
        first_name: claims.first_name,
        last_name: claims.last_name,
        is_admin: claims.is_admin,
    };

    // Add auth context to request extensions
    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

// Helper function to extract auth context from request
pub fn extract_auth_context(request: &Request) -> Result<AuthContext, AppError> {
    request
        .extensions()
        .get::<AuthContext>()
        .cloned()
        .ok_or_else(|| AppError::authentication("Authentication context not found"))
}

// Middleware for admin-only routes
pub async fn admin_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_context = request
        .extensions()
        .get::<AuthContext>()
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    if !auth_context.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}