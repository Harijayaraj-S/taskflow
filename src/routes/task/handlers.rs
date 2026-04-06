//! Routes - Task - Handlers

use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use std::sync::Arc;

use crate::{
    domain::task::Task,
    error::CommonResult,
    middleware::auth::AuthUser,
    routes::task::types::{CreateTaskRequest, ListTasksQuery, UpdateTaskStatusRequest},
    service::task,
    sse::event::{SseEvent, SseEventType},
    state::types::AppState,
};

pub async fn create(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> CommonResult<Task> {
    let pool = state.db.pool();
    let created = task::create_task(pool, payload.project_id, &payload.title, auth.user_id).await?;

    state.sse.broadcast(SseEvent {
        event: SseEventType::TaskCreated,
        task_id: created.id,
        project_id: created.project_id,
        data: serde_json::to_value(&created).unwrap_or_default(),
    });

    Ok(Json(created))
}

pub async fn list(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListTasksQuery>,
) -> CommonResult<Vec<Task>> {
    let pool = state.db.pool();
    let tasks = task::list_tasks(pool, query.project_id, auth.user_id).await?;
    Ok(Json(tasks))
}

pub async fn update_status(
    auth: AuthUser,
    Path(task_id): Path<uuid::Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<UpdateTaskStatusRequest>,
) -> CommonResult<Task> {
    let pool = state.db.pool();
    let updated = task::update_status(pool, task_id, payload.status, auth.user_id).await?;

    state.sse.broadcast(SseEvent {
        event: SseEventType::TaskUpdated,
        task_id: updated.id,
        project_id: updated.project_id,
        data: serde_json::to_value(&updated).unwrap_or_default(),
    });

    Ok(Json(updated))
}

pub async fn delete(
    auth: AuthUser,
    Path(task_id): Path<uuid::Uuid>,
    Extension(state): Extension<Arc<AppState>>,
) -> CommonResult<bool> {
    let pool = state.db.pool();

    // Capture project_id before deletion for the SSE event
    let task = crate::repositories::task::find_by_id(pool, task_id)
        .await
        .map_err(crate::error::app::AppError::Database)?
        .ok_or(crate::error::app::AppError::NotFound)?;

    let deleted = task::delete_task(pool, task_id, auth.user_id).await?;

    if deleted {
        state.sse.broadcast(SseEvent {
            event: SseEventType::TaskDeleted,
            task_id,
            project_id: task.project_id,
            data: serde_json::json!({ "id": task_id }),
        });
    }

    Ok(Json(deleted))
}
