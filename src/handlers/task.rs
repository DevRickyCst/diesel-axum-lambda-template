use axum::extract::{Json, Path};
use axum_diesel_api::{CreateTaskRequest, TaskResponse, UpdateTaskRequest};
use uuid::Uuid;

use crate::error::AppError;
use crate::response::AppResponse;
use crate::services::TaskService;

/// List all tasks
pub async fn list_tasks() -> Result<AppResponse<Vec<TaskResponse>>, AppError> {
    let tasks = TaskService::get_all()?;
    Ok(AppResponse::ok(tasks))
}

/// Get a single task by ID
pub async fn get_task(Path(id): Path<Uuid>) -> Result<AppResponse<TaskResponse>, AppError> {
    let task = TaskService::get_by_id(id)?;
    Ok(AppResponse::ok(task))
}

/// Create a new task
pub async fn create_task(
    Json(req): Json<CreateTaskRequest>,
) -> Result<AppResponse<TaskResponse>, AppError> {
    let task = TaskService::create(req)?;
    Ok(AppResponse::created(task))
}

/// Update an existing task
pub async fn update_task(
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<AppResponse<TaskResponse>, AppError> {
    let task = TaskService::update(id, req)?;
    Ok(AppResponse::ok(task))
}

/// Delete a task
pub async fn delete_task(Path(id): Path<Uuid>) -> Result<AppResponse<()>, AppError> {
    TaskService::delete(id)?;
    Ok(AppResponse::no_content())
}
