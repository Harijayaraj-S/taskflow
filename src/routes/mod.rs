//! Routes Mod

use axum::{Router, routing::get};

mod auth;
mod health;
mod project;
mod task;

pub fn buid() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .route("/health", get(health::handler))
        .nest("/projects", project::routes())
        .nest("/tasks", task::routes())
        .route("/events", get(crate::sse::handler::handler))
}
