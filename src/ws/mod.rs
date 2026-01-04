//! Ws Mod

pub mod event;
pub mod handler;
pub mod state;

use axum::{Router, routing::get};

use crate::ws::handler::ws_handler;

pub fn build() -> Router {
    Router::new().route("/ws", get(ws_handler))
}
