//! Middleware - Auth

use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{domain::auth::JwtClaims, error::app::AppError, state::types::AppState};

/// Extracted authenticated user
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Read Authorization header
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // 2. Expect "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        // 3. Decode & validate JWT
        let claims = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?
        .claims;

        // 4. Return authenticated user
        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
