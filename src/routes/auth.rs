//! Routes - Auth

use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    error::CommonResult,
    repositories::user::create_user,
    service::auth::{generate_token, hash_password},
    state::ExtAppState,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

pub async fn signup(
    Extension(state): ExtAppState,
    Json(payload): Json<SignupRequest>,
) -> CommonResult<Json<Value>> {
    let password_hash = hash_password(&payload.password)?;

    let pool = state.db.pool();
    let user = create_user(&pool, &payload.email, &password_hash).await?;

    let token = generate_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "access_token": token
    }))
    .into())
}
