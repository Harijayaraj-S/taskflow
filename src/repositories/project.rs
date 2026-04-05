//! Repositories - Project

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::Project;

pub async fn create_project(
    pool: &PgPool,
    name: &str,
    owner_id: Uuid,
) -> Result<Project, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (id, name, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id, name, owner_id, created_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(owner_id)
    .fetch_one(pool)
    .await
}

pub async fn list_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, owner_id, created_at
        FROM projects
        WHERE owner_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, project_id: Uuid) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, owner_id, created_at
        FROM projects
        WHERE id = $1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await
}
