//! Ws Event

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsEventType {
    TaskCreated,
    TaskUpdated,
    TaskDeleted,
}

#[derive(Debug, Serialize)]
pub struct WsEvent<T> {
    pub r#type: WsEventType,
    pub data: T,
}
