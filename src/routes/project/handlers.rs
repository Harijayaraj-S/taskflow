//! Routes - Project - Handlers

use axum::{Extension, Json};

use crate::{
    domain::project::Project, error::CommonResult, middleware::auth::AuthUser,
    routes::project::types::CreateProjectRequest, service::project, state::ExtAppState,
};

pub async fn create(
    auth: AuthUser,
    Extension(state): ExtAppState,
    Json(payload): Json<CreateProjectRequest>,
) -> CommonResult<Project> {
    let pool = state.db.pool();
    let project = project::create_project(pool, &payload.name, auth.user_id).await?;
    Ok(Json(project))
}

pub async fn list(auth: AuthUser, Extension(state): ExtAppState) -> CommonResult<Vec<Project>> {
    let pool = state.db.pool();
    let projects = project::list_projects(pool, auth.user_id).await?;
    Ok(Json(projects))
}
