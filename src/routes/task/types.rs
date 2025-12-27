//! Routes - Task - Types

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CreateTaskInput {
    pub title: String,

    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}
