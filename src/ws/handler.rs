//! Ws Handler

use axum::{
    Extension,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{middleware::auth::AuthUser, ws::state::WsState};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<WsState>,
    auth: AuthUser,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, auth.user_id, state))
}

async fn handle_socket(mut socket: WebSocket, user_id: Uuid, state: WsState) {
    let mut rx = {
        let mut map = state.channels.lock().await;

        let sender = map.entry(user_id).or_insert_with(|| {
            let (tx, _rx) = tokio::sync::broadcast::channel(32);
            tx
        });

        sender.subscribe()
    };

    while let Ok(msg) = rx.recv().await {
        if socket
            .send(axum::extract::ws::Message::Text(msg.into()))
            .await
            .is_err()
        {
            break;
        }
    }
}
