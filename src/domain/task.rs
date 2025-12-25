//! Domain - Task

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
