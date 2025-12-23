//! Routes Mod

use axum::{Router, routing::get};

mod auth;
mod health;

pub fn buid() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .route("/health", get(health::handler))
}
