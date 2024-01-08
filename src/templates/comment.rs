use askama::Template;
use chrono::NaiveDateTime;
use crate::repositories::{comment::models::CommentExtended, user::models::UserLite};
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

impl From<CommentExtended> for CommentTemplate {
    fn from(value: CommentExtended) -> Self {
        let author_lite: UserLite = value.author.into();
        CommentTemplate {
            id: value.comment_id,
            parent_category_id: value.event_id.expect("Should be set!"),
            author: author_lite.into(),
            content: value.content,
            created_at: value.created_at,
            edited_at: value.created_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/comments.html")]
pub struct CommentsTemplate {
    pub comments: Vec<CommentTemplate>,
}
