//! Routes Health

use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Taskflow server is running!</h1>")
}
