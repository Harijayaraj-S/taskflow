//! Service - Project

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::Project;
use crate::error::app::AppError;
use crate::repositories::project as project_repo;

pub async fn create_project(
    pool: &PgPool,
    name: &str,
    owner_id: Uuid,
) -> Result<Project, AppError> {
    let project = project_repo::create_project(pool, name, owner_id).await?;
    Ok(project)
}

pub async fn list_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, AppError> {
    let projects = project_repo::list_projects(pool, user_id).await?;
    Ok(projects)
}
