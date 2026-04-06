//! Service - Project

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::Project;
use crate::error::app::AppError;
use crate::repositories::project as project_repo;
use crate::repositories::user as user_repo;

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

pub async fn invite_user(
    pool: &PgPool,
    project_id: Uuid,
    email: &str,
    current_user_id: Uuid,
) -> Result<(), AppError> {
    // 1. Verify current user is a member of the project
    if !project_repo::is_member(pool, project_id, current_user_id).await? {
        return Err(AppError::Forbidden);
    }

    // 2. Find user by email
    let user = match user_repo::find_user_by_email(pool, email).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return Err(AppError::NotFound),
        Err(e) => return Err(e.into()),
    };

    // 3. Insert into project_members
    if let Err(e) = project_repo::add_member(pool, project_id, user.id).await {
        // Handle unique constraint violation (conflict)
        if let Some(db_err) = e.as_database_error() {
            if db_err.code().unwrap_or_default() == "23505" {
                return Err(AppError::Conflict(
                    "User is already a member of this project".to_string(),
                ));
            }
        }
        return Err(AppError::Database(e));
    }

    Ok(())
}
