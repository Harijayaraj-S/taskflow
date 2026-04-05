//! App

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::routes;

pub fn build() -> Router {
    let frontend = ServeDir::new("frontend").fallback(ServeFile::new("frontend/index.html"));

    Router::new()
        .merge(routes::buid())
        .fallback_service(frontend)
        .layer(CorsLayer::permissive())
}
