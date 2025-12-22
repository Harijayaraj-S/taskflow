//! Repositories - User

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::user::User;

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
        RETURNING id, email, created_at, modified_at
        "#,
        Uuid::new_v4(),
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
