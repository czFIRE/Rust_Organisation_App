use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::repositories::user::models::User;

#[derive(Debug)]
pub struct NewComment {
    pub author_id: Uuid,
    pub event_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub content: String,
}

#[derive(Debug, FromRow)]
pub struct Comment {
    pub comment_id: Uuid,
    pub author_id: Uuid,
    pub event_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct CommentExtended {
    pub comment_id: Uuid,
    pub author: User,
    pub event_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct CommentData {
    pub content: String,
}

#[derive(Debug)]
pub struct CommentFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
