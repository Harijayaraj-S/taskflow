//! Routes - Task - Types

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: String,
}
