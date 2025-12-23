//! Routes - Auth - Mod

use axum::{Router, routing::post};

mod handlers;
mod types;

pub fn routes() -> Router {
    Router::new()
        .route("/signup", post(handlers::signup))
        .route("/login", post(handlers::login))
}
