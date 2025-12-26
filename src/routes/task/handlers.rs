//! Routes - Task - Handlers

use axum::{Extension, Json};

use crate::{
    domain::task::Task, error::CommonResult, middleware::auth::AuthUser, repositories::task,
    routes::task::types::CreateTaskInput, state::ExtAppState,
};

pub async fn create(
    auth: AuthUser,
    Extension(state): ExtAppState,
    Json(input): Json<CreateTaskInput>,
) -> CommonResult<Task> {
    let pool = state.db.pool();

    let task = task::create(pool, auth.user_id, &input.title, &input.description).await?;

    Ok(Json(task))
}

pub async fn list(auth: AuthUser, Extension(state): ExtAppState) -> CommonResult<Vec<Task>> {
    let pool = state.db.pool();

    let tasks = task::list_by_user(pool, auth.user_id).await?;

    Ok(Json(tasks))
}
