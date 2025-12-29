//! Routes - Task - Mod

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

mod handlers;
mod types;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(handlers::create))
        .route("/", get(handlers::list))
        .route("/{id}", patch(handlers::update))
        .route("/{id}", delete(handlers::delete))
}
