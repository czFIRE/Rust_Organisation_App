use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{models::TaskPriority, repositories::user::models::User};

#[derive(Debug)]
pub struct NewTask {
    pub event_id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub event_id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDateTime>, // TODO: WHO THE FUCK MADE FINISHED_AT AS DATE
    pub priority: TaskPriority,
    pub accepts_staff: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TaskExtended {
    pub task_id: Uuid,
    pub event_id: Uuid,
    pub creator: User,
    pub title: String,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDateTime>,
    pub priority: TaskPriority,
    pub accepts_staff: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct TaskData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDateTime>,
    pub priority: Option<TaskPriority>,
    pub accepts_staff: Option<bool>,
}

#[derive(Debug)]
pub struct TaskFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
