//! Routes Mod

use axum::{Router, routing::get};

mod health;

pub fn buid() -> Router {
    Router::new().route("/health", get(health::handler))
}
