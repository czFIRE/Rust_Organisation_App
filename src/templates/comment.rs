use crate::repositories::{comment::models::CommentExtended, user::models::UserLite};
use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use super::user::UserLiteTemplate;

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/comment-update.html")]
pub struct CommentUpdateModeTemplate {
    pub comment: SingleComment,
}

// This is not quite pleasant, but ... yeah.
#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/single-comment.html")]
pub struct CommentTemplate {
    pub requester_id: Uuid,
    pub comment: SingleComment,
}

#[derive(Debug, Deserialize)]
pub struct SingleComment {
    pub id: Uuid,
    pub parent_category_id: Uuid,
    pub author: UserLiteTemplate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<CommentExtended> for SingleComment {
    fn from(value: CommentExtended) -> Self {
        let author_lite: UserLite = value.author.into();
        SingleComment {
            id: value.comment_id,
            parent_category_id: if value.event_id.is_some() {
                value.event_id.unwrap()
            } else {
                value.task_id.expect("Should be set.")
            },
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
    pub requester_id: Uuid,
    pub comments: Vec<SingleComment>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/event-comments-container.html")]
pub struct EventCommentsContainerTemplate {
    pub comments: Vec<SingleComment>,
    pub requester_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "comment/task-comments-container.html")]
pub struct TaskCommentsContainerTemplate {
    pub comments: Vec<SingleComment>,
    pub requester_id: Uuid,
    pub task_id: Uuid,
}
