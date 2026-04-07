//! Repositories - Project

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::{Project, ProjectMemberItem};

pub async fn create_project(
    pool: &PgPool,
    name: &str,
    owner_id: Uuid,
) -> Result<Project, sqlx::Error> {
    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (id, name, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id, name, owner_id, created_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(owner_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO project_members (project_id, user_id, role)
        VALUES ($1, $2, 'owner')
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(project)
}

pub async fn list_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        SELECT p.id, p.name, p.owner_id, p.created_at
        FROM projects p
        JOIN project_members pm ON p.id = pm.project_id
        WHERE pm.user_id = $1
        ORDER BY p.created_at DESC
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

pub async fn add_member(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO project_members (project_id, user_id, role)
        VALUES ($1, $2, 'member')
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn is_member(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM project_members
            WHERE project_id = $1 AND user_id = $2
        )
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn get_members_by_id(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<ProjectMemberItem>, sqlx::Error> {
    sqlx::query_as::<_, ProjectMemberItem>(
        r#"
        SELECT u.email AS name, u.email, pm.role
        FROM projects p
        JOIN project_members pm ON p.id = pm.project_id
        JOIN users u ON pm.user_id = u.id
        WHERE pm.project_id = $1
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}
