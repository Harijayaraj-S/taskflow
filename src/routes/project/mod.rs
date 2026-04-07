//! Routes - Project - Mod

use axum::{
    Router,
    routing::{get, post},
};

mod handlers;
mod types;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(handlers::create))
        .route("/", get(handlers::list))
        .route("/{project_id}/invite", post(handlers::invite))
        .route("/{project_id}/members", get(handlers::members))
}
