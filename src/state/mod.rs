//! State Mod

use axum::Extension;
use std::sync::Arc;

use crate::state::types::AppState;

pub mod db;
pub mod types;

// Types
pub type ExtAppState = Extension<Arc<AppState>>;
