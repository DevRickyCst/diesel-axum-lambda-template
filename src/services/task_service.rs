use axum_diesel_api::{CreateTaskRequest, TaskResponse, UpdateTaskRequest};
use uuid::Uuid;

use crate::db::connection;
use crate::db::models::{NewTask, UpdateTask};
use crate::db::repositories::TaskRepository;
use crate::error::AppError;

pub struct TaskService;

impl TaskService {
    /// Get all tasks
    pub fn get_all() -> Result<Vec<TaskResponse>, AppError> {
        let mut conn = connection::get_connection()
            .map_err(|e| AppError::internal(format!("Failed to get connection: {}", e)))?;

        let tasks = TaskRepository::find_all(&mut conn)?;
        Ok(tasks.into_iter().map(TaskResponse::from).collect())
    }

    /// Get a task by ID
    pub fn get_by_id(id: Uuid) -> Result<TaskResponse, AppError> {
        let mut conn = connection::get_connection()
            .map_err(|e| AppError::internal(format!("Failed to get connection: {}", e)))?;

        let task = TaskRepository::find_by_id(&mut conn, id)?;
        Ok(TaskResponse::from(task))
    }

    /// Create a new task
    pub fn create(req: CreateTaskRequest) -> Result<TaskResponse, AppError> {
        // Validation
        if req.title.trim().is_empty() {
            return Err(AppError::validation("Title cannot be empty"));
        }

        if req.title.len() > 255 {
            return Err(AppError::validation(
                "Title must be less than 255 characters",
            ));
        }

        let mut conn = connection::get_connection()
            .map_err(|e| AppError::internal(format!("Failed to get connection: {}", e)))?;

        let new_task = NewTask {
            title: req.title.trim().to_string(),
            description: req
                .description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            completed: req.completed,
        };

        let task = TaskRepository::create(&mut conn, new_task)?;
        Ok(TaskResponse::from(task))
    }

    /// Update an existing task
    pub fn update(id: Uuid, req: UpdateTaskRequest) -> Result<TaskResponse, AppError> {
        // Validate title if provided
        if let Some(ref title) = req.title {
            if title.trim().is_empty() {
                return Err(AppError::validation("Title cannot be empty"));
            }

            if title.len() > 255 {
                return Err(AppError::validation(
                    "Title must be less than 255 characters",
                ));
            }
        }

        let mut conn = connection::get_connection()
            .map_err(|e| AppError::internal(format!("Failed to get connection: {}", e)))?;

        // Check if task exists
        TaskRepository::find_by_id(&mut conn, id)?;

        let update_task = UpdateTask {
            title: req.title.map(|t| t.trim().to_string()),
            description: req
                .description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            completed: req.completed,
        };

        let task = TaskRepository::update(&mut conn, id, update_task)?;
        Ok(TaskResponse::from(task))
    }

    /// Delete a task
    pub fn delete(id: Uuid) -> Result<(), AppError> {
        let mut conn = connection::get_connection()
            .map_err(|e| AppError::internal(format!("Failed to get connection: {}", e)))?;

        // Check if task exists
        TaskRepository::find_by_id(&mut conn, id)?;

        TaskRepository::delete(&mut conn, id)?;
        Ok(())
    }
}
