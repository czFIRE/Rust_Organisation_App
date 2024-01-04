use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use super::user::UserLiteTemplate;

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/comment.html")]
pub struct CommentTemplate {
    pub id: Uuid,
    pub parent_category_id: Uuid,
    pub author: UserLiteTemplate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/comments.html")]
pub struct CommentsTemplate {
    pub comments: Vec<CommentTemplate>,
}
