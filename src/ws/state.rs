//! Ws State

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast::Sender};
use uuid::Uuid;

use crate::ws::event::WsEvent;

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

    pub async fn send_event<T: Serialize>(&self, user_id: uuid::Uuid, event: WsEvent<T>) {
        if let Some(sender) = self.channels.lock().await.get(&user_id) {
            if let Ok(json) = serde_json::to_string(&event) {
                let _ = sender.send(json);
            }
        }
    }
}
