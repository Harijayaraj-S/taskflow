//! Repositories - Task

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::task::{Task, TaskStatus};

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    title: &str,
    description: &str,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        INSERT INTO tasks (id, user_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING
        id, user_id, title, description, created_at,
        status, tags, modified_at
        "#,
        Uuid::new_v4(),
        user_id,
        title,
        description
    )
    .fetch_one(pool)
    .await
}

pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT * FROM tasks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update_task(
    pool: &sqlx::PgPool,
    task_id: Uuid,
    user_id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        UPDATE tasks
        SET
            title = COALESCE($3, title),
            description = COALESCE($4, description)
        WHERE id = $1 AND user_id = $2
        RETURNING *
        "#,
        task_id,
        user_id,
        title,
        description
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_task(
    pool: &sqlx::PgPool,
    task_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM tasks
        WHERE id = $1 AND user_id = $2
        "#,
        task_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() != 0)
}

pub async fn update_status(
    pool: &PgPool,
    task_id: Uuid,
    user_id: Uuid,
    status: TaskStatus,
) -> Result<Task, sqlx::Error> {
    let status_str = match status {
        TaskStatus::Todo => "todo",
        TaskStatus::InProgress => "in_progress",
        TaskStatus::Done => "done",
    };

    sqlx::query_as!(
        Task,
        r#"
        UPDATE tasks
        SET status = COALESCE($3, status)
        WHERE id = $1 AND user_id = $2
        RETURNING *
        "#,
        task_id,
        user_id,
        status_str
    )
    .fetch_one(pool)
    .await
}
