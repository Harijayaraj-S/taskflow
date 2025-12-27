//! Routes - Task - Handlers

use axum::{Extension, Json, extract::Path};

use crate::{
    domain::task::Task,
    error::CommonResult,
    middleware::auth::AuthUser,
    repositories::task,
    routes::task::types::{CreateTaskInput, UpdateTaskRequest},
    state::ExtAppState,
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

pub async fn update(
    auth: AuthUser,
    Path(task_id): Path<uuid::Uuid>,
    Extension(state): ExtAppState,
    Json(payload): Json<UpdateTaskRequest>,
) -> CommonResult<Task> {
    let pool = state.db.pool();

    let task = task::update_task(
        pool,
        task_id,
        auth.user_id,
        payload.title.as_deref(),
        payload.description.as_deref(),
    )
    .await?;

    Ok(Json(task))
}
