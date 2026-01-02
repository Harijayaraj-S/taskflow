//! Ws State

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast::Sender};
use uuid::Uuid;

#[derive(Clone)]
pub struct WsState {
    pub channels: Arc<Mutex<HashMap<Uuid, Sender<String>>>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
