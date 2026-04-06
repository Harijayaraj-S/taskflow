//! Service - Task

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::task::{Task, TaskStatus};
use crate::error::app::AppError;
use crate::repositories::{project as project_repo, task as task_repo};

/// Validate that the user is a member, then create a task.
pub async fn create_task(
    pool: &PgPool,
    project_id: Uuid,
    title: &str,
    user_id: Uuid,
) -> Result<Task, AppError> {
    if !project_repo::is_member(pool, project_id, user_id).await? {
        return Err(AppError::Forbidden);
    }

    let task = task_repo::create_task(pool, project_id, title, user_id).await?;
    Ok(task)
}

/// List all tasks for a project, validating membership.
pub async fn list_tasks(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<Task>, AppError> {
    if !project_repo::is_member(pool, project_id, user_id).await? {
        return Err(AppError::Forbidden);
    }
    let tasks = task_repo::list_by_project(pool, project_id).await?;
    Ok(tasks)
}

/// Update task status — validates that the user is a member of the project.
pub async fn update_status(
    pool: &PgPool,
    task_id: Uuid,
    status: TaskStatus,
    user_id: Uuid,
) -> Result<Task, AppError> {
    let task = task_repo::find_by_id(pool, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !project_repo::is_member(pool, task.project_id, user_id).await? {
        return Err(AppError::Forbidden);
    }

    let updated = task_repo::update_status(pool, task_id, status).await?;
    Ok(updated)
}

/// Delete a task — validates that the user is a member of the project.
pub async fn delete_task(pool: &PgPool, task_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
    let task = task_repo::find_by_id(pool, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !project_repo::is_member(pool, task.project_id, user_id).await? {
        return Err(AppError::Forbidden);
    }

    let deleted = task_repo::delete_task(pool, task_id).await?;
    Ok(deleted)
}
