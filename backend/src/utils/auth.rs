use anyhow::Result;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,           // User ID
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
    pub iat: usize,           // Issued at
    pub exp: usize,           // Expiration
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn create_jwt(
    user_id: Uuid,
    email: &str,
    first_name: &str,
    last_name: &str,
    is_admin: bool,
    jwt_secret: &str,
    expiration_hours: u64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(expiration_hours as i64);

    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        is_admin,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str, jwt_secret: &str) -> Result<JwtClaims, AppError> {
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn generate_password_reset_token(user_id: Uuid, jwt_secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(1); // 1 hour expiration

    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: "".to_string(),  // Not needed for password reset
        first_name: "".to_string(),
        last_name: "".to_string(),
        is_admin: false,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}

// Password validation
pub fn validate_password_strength(password: &str, min_length: u32) -> Result<(), AppError> {
    if password.len() < min_length as usize {
        return Err(AppError::bad_request(format!(
            "Password must be at least {} characters long",
            min_length
        )));
    }

    // Check for at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::bad_request(
            "Password must contain at least one lowercase letter"
        ));
    }

    // Check for at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::bad_request(
            "Password must contain at least one uppercase letter"
        ));
    }

    // Check for at least one digit
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::bad_request(
            "Password must contain at least one digit"
        ));
    }

    // Check for at least one special character
    if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
        return Err(AppError::bad_request(
            "Password must contain at least one special character"
        ));
    }

    Ok(())
}

// Generate a secure random token
pub fn generate_secure_token(length: usize) -> Result<String, AppError> {
    use rand::{distributions::Alphanumeric, Rng};
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    Ok(token)
}