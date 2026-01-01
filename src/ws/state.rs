//! Ws State

use std::collections::HashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
pub struct WsState {
    pub channels: std::sync::Arc<tokio::sync::Mutex<HashMap<Uuid, broadcast::Sender<String>>>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            channels: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }
}
