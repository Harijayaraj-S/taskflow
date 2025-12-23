//! Routes - Auth - Handlers

use axum::{Extension, Json};

use crate::{
    error::{CommonResult, app::AppError},
    repositories::user,
    routes::auth::types::{AuthTokenResponse, LoginRequest, SignupRequest},
    service::auth,
    state::ExtAppState,
};

pub async fn signup(
    Extension(state): ExtAppState,
    Json(payload): Json<SignupRequest>,
) -> CommonResult<AuthTokenResponse> {
    let password_hash = auth::hash_password(&payload.password)?;

    let pool = state.db.pool();
    let user = user::create_user(pool, &payload.email, &password_hash).await?;

    let access_token = auth::generate_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(AuthTokenResponse {
        access_token,
        expires_in: 3600,
    }))
}

pub async fn login(
    Extension(state): ExtAppState,
    Json(payload): Json<LoginRequest>,
) -> CommonResult<AuthTokenResponse> {
    // 1. Fetch user by email
    let pool = state.db.pool();
    let user = user::find_user_by_email(pool, &payload.email)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    // 2. Verify password
    let is_valid = auth::verify_password(&payload.password, &user.password_hash)?;

    if !is_valid {
        return Err(AppError::Unauthorized);
    }
    let token = auth::generate_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(AuthTokenResponse {
        access_token: token,
        expires_in: 3600,
    }))
}
