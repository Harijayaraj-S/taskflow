//! Routes - Task - Types

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CreateTaskInput {
    pub title: String,

    #[serde(default)]
    pub description: String,
}
