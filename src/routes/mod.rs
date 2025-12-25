//! Routes Mod

use axum::{Router, routing::get};

mod auth;
mod health;
mod task;

pub fn buid() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .route("/health", get(health::handler))
        .nest("/tasks", task::routes())
}
