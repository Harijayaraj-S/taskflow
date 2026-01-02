//! Service - Auth

use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

use crate::domain::auth::JwtClaims;

const TOKEN_EXPIRY_HOURS: i64 = 1;

/// Hash a plain-text password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("password hashing failed: {e}"))
        .map(|hash| hash.to_string())
}

/// Verify a plain-text password against a stored hash
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| anyhow!("invalid password hash format: {e}"))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generate JWT access token
pub fn generate_token(user_id: Uuid, secret: &str) -> Result<String> {
    let now = Utc::now();

    let claims = JwtClaims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(TOKEN_EXPIRY_HOURS)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!("jwt token generation failed: {e}"))
}
