//! State Types

use std::sync::Arc;

use anyhow::Result;

use crate::{config::types::AppConfig, state::db::DbManager};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DbManager,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Arc<Self>> {
        let db = DbManager::new().await?;
        let app_state = AppState {
            config: config.clone(),
            db,
        };

        Ok(Arc::new(app_state))
    }
}
