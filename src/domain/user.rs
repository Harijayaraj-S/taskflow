//! Domain - User

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
