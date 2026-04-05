//! SSE - Broadcaster

use std::sync::Arc;
use tokio::sync::broadcast;

use crate::sse::event::SseEvent;

const CHANNEL_CAPACITY: usize = 128;

#[derive(Clone)]
pub struct SseBroadcaster {
    tx: Arc<broadcast::Sender<SseEvent>>,
}

impl SseBroadcaster {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { tx: Arc::new(tx) }
    }

    /// Subscribe to the event stream. Returns a receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<SseEvent> {
        self.tx.subscribe()
    }

    /// Broadcast an event to all subscribers.
    pub fn broadcast(&self, event: SseEvent) {
        // Ignore errors (no active subscribers is fine)
        let _ = self.tx.send(event);
    }
}
