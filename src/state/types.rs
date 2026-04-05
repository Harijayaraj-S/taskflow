//! State Types

use std::sync::Arc;

use anyhow::Result;

use crate::{config::types::AppConfig, sse::broadcaster::SseBroadcaster, state::db::DbManager};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DbManager,
    pub jwt_secret: String,
    pub sse: SseBroadcaster,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Arc<Self>> {
        let db = DbManager::new().await?;
        let sse = SseBroadcaster::new();
        let app_state = AppState {
            config: config.clone(),
            db,
            jwt_secret: config.jwt_secret.clone(),
            sse,
        };

        Ok(Arc::new(app_state))
    }
}
