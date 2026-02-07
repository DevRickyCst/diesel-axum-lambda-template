use axum_diesel_api::TaskResponse;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::tasks;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            completed: task.completed,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}
