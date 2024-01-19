use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{Gender, TaskPriority, UserRole, UserStatus},
    repositories::user::models::User,
};

#[derive(Debug, Clone)]
pub struct NewTask {
    pub event_id: Uuid,
    pub creator_id: Uuid, // references event_staff
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
}

// TODO needs to be kept the same as in task/models.rs => TaskUserFlattened
#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub event_id: Uuid,
    pub creator_id: Uuid, // references event_staff
    pub title: String,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDateTime>,
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
    pub creator_id: Uuid,
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

#[derive(Debug, Deserialize, Clone)]
pub struct TaskData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub finished_at: Option<NaiveDateTime>,
    pub priority: Option<TaskPriority>,
    pub accepts_staff: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

//////////////////////////////////////////////////

// TODO needs to be kept the same as in user/models.rs => User
// TODO needs to be kept the same as in task/models.rs => Task
#[derive(Debug, FromRow)]
pub struct TaskUserFlattened {
    pub task_id: Uuid,
    pub task_event_id: Uuid,
    pub task_creator_id: Uuid,
    pub task_title: String,
    pub task_description: Option<String>,
    pub task_finished_at: Option<NaiveDateTime>,
    pub task_priority: TaskPriority,
    pub task_accepts_staff: bool,
    pub task_created_at: NaiveDateTime,
    pub task_edited_at: NaiveDateTime,
    pub task_deleted_at: Option<NaiveDateTime>,

    pub user_id: Uuid, // same as task_creator_id
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

impl From<TaskUserFlattened> for TaskExtended {
    fn from(value: TaskUserFlattened) -> Self {
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
            task_id: value.task_id,
            event_id: value.task_event_id,
            creator_id: value.task_creator_id,
            creator: tmp_user,
            title: value.task_title,
            description: value.task_description,
            finished_at: value.task_finished_at,
            priority: value.task_priority,
            accepts_staff: value.task_accepts_staff,
            created_at: value.task_created_at,
            edited_at: value.task_edited_at,
            deleted_at: value.task_deleted_at,
        }
    }
}
