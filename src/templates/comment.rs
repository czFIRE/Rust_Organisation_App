use askama::Template;
use sqlx::types::uuid;
use uuid::Uuid;
use chrono::NaiveDateTime;

use super::user::UserLiteTemplate;

#[derive(Template)]
#[template(path = "comment/comment.html")]
pub struct CommentTemplate {
    pub id: Uuid,
    pub parent_category_id: Uuid,
    pub author: UserLiteTemplate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime
}