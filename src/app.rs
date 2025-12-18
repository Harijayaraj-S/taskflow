//! App

use axum::Router;

use crate::routes;

pub fn build() -> Router {
    Router::new().merge(routes::buid())
}
