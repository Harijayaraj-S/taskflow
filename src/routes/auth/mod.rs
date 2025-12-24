//! Routes - Auth - Mod

use axum::{
    Router,
    routing::{get, post},
};

mod handlers;
mod types;

pub fn routes() -> Router {
    Router::new()
        .route("/signup", post(handlers::signup))
        .route("/login", post(handlers::login))
        .route("/me", get(handlers::me))
}
