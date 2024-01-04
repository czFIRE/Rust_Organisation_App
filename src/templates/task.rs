use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::TaskPriority;

use super::user::UserLiteTemplate;

#[derive(Template, Deserialize)]
#[template(path = "event/task/task.html")]
pub struct TaskTemplate {
    pub id: Uuid,
    pub event_id: Uuid,
    pub creator: UserLiteTemplate,
    pub title: String,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDate>,
    pub priority: TaskPriority,
    pub accepts_staff: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Deserialize)]
#[template(path = "event/task/tasks.html")]
pub struct TasksTemplate {
    pub tasks: Vec<TaskTemplate>,
}
