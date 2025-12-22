//! Routes Mod

use axum::{Router, routing::get};

mod auth;
mod health;

pub fn buid() -> Router {
    Router::new().route("/health", get(health::handler))
}
