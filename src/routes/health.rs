//! Routes Health

use axum::response::Html;

use crate::state::ExtAppState;

pub async fn handler(state: ExtAppState) -> Html<String> {
    Html(format!(
        "<h1>Taskflow server is running in {:?} Env</h1>",
        state.config.env
    ))
}
