use diesel::prelude::*;
use uuid::Uuid;

use crate::db::error::RepositoryError;
use crate::db::models::{NewTask, Task, UpdateTask};
use crate::db::schema::tasks;
use crate::db::connection::DbConnection;

pub struct TaskRepository;

impl TaskRepository {
    /// Find all tasks
    pub fn find_all(conn: &mut DbConnection) -> Result<Vec<Task>, RepositoryError> {
        tasks::table
            .select(Task::as_select())
            .load(conn)
            .map_err(Into::into)
    }

    /// Find a task by ID
    pub fn find_by_id(conn: &mut DbConnection, task_id: Uuid) -> Result<Task, RepositoryError> {
        tasks::table
            .find(task_id)
            .select(Task::as_select())
            .first(conn)
            .map_err(Into::into)
    }

    /// Create a new task
    pub fn create(conn: &mut DbConnection, new_task: NewTask) -> Result<Task, RepositoryError> {
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result(conn)
            .map_err(Into::into)
    }

    /// Update an existing task
    pub fn update(
        conn: &mut DbConnection,
        task_id: Uuid,
        update_task: UpdateTask,
    ) -> Result<Task, RepositoryError> {
        diesel::update(tasks::table.find(task_id))
            .set(&update_task)
            .returning(Task::as_returning())
            .get_result(conn)
            .map_err(Into::into)
    }

    /// Delete a task
    pub fn delete(conn: &mut DbConnection, task_id: Uuid) -> Result<(), RepositoryError> {
        diesel::delete(tasks::table.find(task_id))
            .execute(conn)
            .map(|_| ())
            .map_err(Into::into)
    }
}
