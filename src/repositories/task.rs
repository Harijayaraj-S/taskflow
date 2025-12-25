//! Repositories - Task

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::task::Task;

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
