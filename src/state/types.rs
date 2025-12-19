//! State Types

use std::sync::Arc;

use crate::config::types::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: &AppConfig) -> Arc<Self> {
        let app_state = AppState {
            config: config.clone(),
        };

        Arc::new(app_state)
    }
}
