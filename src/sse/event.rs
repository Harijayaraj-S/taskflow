//! SSE - Event

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum SseEventType {
    TaskCreated,
    TaskUpdated,
    TaskDeleted,
}

#[derive(Debug, Clone, Serialize)]
pub struct SseEvent {
    pub event: SseEventType,
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub data: serde_json::Value,
}
