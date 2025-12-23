//! Repositories - User

use crate::domain::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, created_at, modified_at, password_hash
        "#,
        Uuid::new_v4(),
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, created_at, modified_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await
}
