use askama::Template;
use sqlx::types::uuid;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};

use crate::models::TaskPriority;

use super::user::UserLiteTemplate;

#[derive(Template)]
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
    pub edited_at: NaiveDateTime
}