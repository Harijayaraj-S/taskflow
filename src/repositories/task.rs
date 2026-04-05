//! Repositories - Task

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::task::{Task, TaskStatus};

pub async fn create_task(
    pool: &PgPool,
    project_id: Uuid,
    title: &str,
    created_by: Uuid,
) -> Result<Task, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (id, project_id, title, created_by)
        VALUES ($1, $2, $3, $4)
        RETURNING id, project_id, title, description, status, tags,
                  assigned_to, created_by, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(title)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

pub async fn list_by_project(pool: &PgPool, project_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, project_id, title, description, status, tags,
               assigned_to, created_by, created_at, updated_at
        FROM tasks
        WHERE project_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, task_id: Uuid) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, project_id, title, description, status, tags,
               assigned_to, created_by, created_at, updated_at
        FROM tasks
        WHERE id = $1
        "#,
    )
    .bind(task_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_status(
    pool: &PgPool,
    task_id: Uuid,
    status: TaskStatus,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET status = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, project_id, title, description, status, tags,
                  assigned_to, created_by, created_at, updated_at
        "#,
    )
    .bind(task_id)
    .bind(status.to_string())
    .fetch_one(pool)
    .await
}

pub async fn delete_task(pool: &PgPool, task_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(task_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
