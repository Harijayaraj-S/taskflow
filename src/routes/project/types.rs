//! Routes - Project - Types

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct InviteUserRequest {
    pub email: String,
}
