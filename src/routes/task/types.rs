//! Routes - Task - Types

use serde::Deserialize;
use uuid::Uuid;

use crate::domain::task::TaskStatus;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: Uuid,
    pub title: String,
}

#[derive(Deserialize)]
pub struct ListTasksQuery {
    pub project_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
}
