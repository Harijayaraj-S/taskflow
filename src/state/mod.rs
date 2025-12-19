//! State Mod

use std::sync::Arc;

use axum::Extension;

use crate::state::types::AppState;

pub mod types;

// Types
pub type ExtAppState = Extension<Arc<AppState>>;
