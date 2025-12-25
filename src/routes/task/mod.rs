//! Routes - Task - Mod

use axum::{Router, routing::post};

mod handlers;
mod types;

pub fn routes() -> Router {
    Router::new().route("/", post(handlers::create))
}
