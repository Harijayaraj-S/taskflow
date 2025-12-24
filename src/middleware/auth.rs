//! Middleware - Auth

use axum::{Extension, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;
use uuid::Uuid;

use crate::{domain::auth::JwtClaims, error::app::AppError, state::types::AppState};

/// Extracted authenticated user
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<()> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &()) -> Result<Self, Self::Rejection> {
        // 1. Extract AppState from Extension
        let Extension(state): Extension<Arc<AppState>> =
            Extension::from_request_parts(parts, _state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        // 2. Read Authorization header
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // 3. Expect "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        // 4. Decode & validate JWT
        let claims = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?
        .claims;

        // 5. Return authenticated user
        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
