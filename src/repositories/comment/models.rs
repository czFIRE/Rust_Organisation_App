use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{Gender, UserRole, UserStatus},
    repositories::user::models::User,
};

#[derive(Debug, Clone)]
pub struct NewComment {
    pub author_id: Uuid,
    pub event_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub content: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub author_id: Uuid,
    pub event_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CommentData {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct CommentFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/////////////////////////////////////////////

// TODO needs to be kept the same as in user/models.rs => User
// TODO needs to be kept the same as in comment/models.rs => Comment
#[derive(Debug, FromRow)]
pub struct CommentUserFlattened {
    pub comment_id: Uuid,
    pub comment_author_id: Uuid,
    pub comment_event_id: Option<Uuid>,
    pub comment_task_id: Option<Uuid>,
    pub comment_content: String,
    pub comment_created_at: NaiveDateTime,
    pub comment_edited_at: NaiveDateTime,
    pub comment_deleted_at: Option<NaiveDateTime>,

    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_birth: NaiveDate,
    pub user_avatar_url: String,
    pub user_gender: Gender,
    pub user_role: UserRole,
    pub user_status: UserStatus,
    pub user_created_at: NaiveDateTime,
    pub user_edited_at: NaiveDateTime,
    pub user_deleted_at: Option<NaiveDateTime>,
}

impl From<CommentUserFlattened> for CommentExtended {
    fn from(value: CommentUserFlattened) -> Self {
        let tmp_user = User {
            id: value.user_id,
            name: value.user_name,
            email: value.user_email,
            birth: value.user_birth,
            avatar_url: value.user_avatar_url,
            gender: value.user_gender,
            role: value.user_role,
            status: value.user_status,
            created_at: value.user_created_at,
            edited_at: value.user_edited_at,
            deleted_at: value.user_deleted_at,
        };

        Self {
            comment_id: value.comment_id,
            author: tmp_user,
            event_id: value.comment_event_id,
            task_id: value.comment_task_id,
            content: value.comment_content,
            created_at: value.comment_created_at,
            edited_at: value.comment_edited_at,
            deleted_at: value.comment_deleted_at,
        }
    }
}
