//! State Mod

use axum::Extension;
use std::sync::Arc;

use crate::state::{db::DbManager, types::AppState};

pub mod db;
pub mod types;

// Types
pub type ExtAppState = Extension<Arc<AppState>>;
pub type ExtDbState = Extension<Arc<DbManager>>;
