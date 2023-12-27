use crate::{common::DbResult, repositories::task::models::TaskUserFlattened};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{NewTask, Task, TaskData, TaskExtended, TaskFilter};

use crate::models::{Gender, TaskPriority, UserRole, UserStatus};

#[derive(Clone)]
pub struct TaskRepository {
    pub pool: Arc<PgPool>,
}

impl TaskRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, data: NewTask) -> DbResult<Task> {
        let executor = self.pool.as_ref();

        let new_task: Task = sqlx::query_as!(
            Task,
            r#" INSERT INTO task (
                event_id, creator_id, title, description, priority
                ) VALUES 
                ($1, $2, $3, $4, $5) RETURNING id, 
                event_id, 
                creator_id, 
                title, 
                description, 
                finished_at, 
                priority AS "priority!: TaskPriority", 
                accepts_staff, 
                created_at, 
                edited_at, 
                deleted_at;"#,
            data.event_id,
            data.creator_id,
            data.title,
            data.description,
            data.priority as TaskPriority,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_task)
    }

    pub async fn _read_one(&self, task_id: Uuid) -> DbResult<TaskExtended> {
        // Redis here
        self.read_one_db(task_id).await
    }

    async fn read_one_db(&self, task_id: Uuid) -> DbResult<TaskExtended> {
        let executor = self.pool.as_ref();

        let task_user_flattened: TaskUserFlattened = sqlx::query_as!(
            TaskUserFlattened,
            r#"SELECT 
                task.id AS task_id, 
                task.event_id AS task_event_id, 
                task.creator_id AS task_creator_id, 
                task.title AS task_title, 
                task.description AS task_description, 
                task.finished_at AS task_finished_at, 
                task.priority AS "task_priority!: TaskPriority", 
                task.accepts_staff AS task_accepts_staff, 
                task.created_at AS task_created_at, 
                task.edited_at AS task_edited_at, 
                task.deleted_at AS task_deleted_at, 
                user_record.id AS user_id, 
                user_record.name AS user_name, 
                user_record.email AS user_email, 
                user_record.birth AS user_birth, 
                user_record.avatar_url AS user_avatar_url, 
                user_record.gender AS "user_gender!: Gender", 
                user_record.role AS "user_role!: UserRole",
                user_record.status AS "user_status!: UserStatus", 
                user_record.created_at AS user_created_at, 
                user_record.edited_at AS user_edited_at, 
                user_record.deleted_at AS user_deleted_at
            FROM task 
            INNER JOIN user_record ON task.creator_id=user_record.id 
            WHERE task.id=$1"#,
            task_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(task_user_flattened.into())
    }

    pub async fn _read_all(&self, filter: TaskFilter) -> DbResult<Vec<Task>> {
        let executor = self.pool.as_ref();

        let tasks: Vec<Task> = sqlx::query_as!(
            Task,
            r#"SELECT 
                id, 
                event_id, 
                creator_id, 
                title, 
                description, 
                finished_at, 
                priority AS "priority!: TaskPriority", 
                accepts_staff, 
                created_at, 
                edited_at, 
                deleted_at 
            FROM 
                task
            LIMIT $1 OFFSET $2"#,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(tasks)
    }

    pub async fn _update_task(&self, task_id: Uuid, data: TaskData) -> DbResult<Task> {
        // TODO - this should support transactions
        let executor = self.pool.as_ref();

        if data.description.is_none()
            && data.finished_at.is_none()
            && data.priority.is_none()
            && data.title.is_none()
        {
            // TODO - add better error
            return Err(sqlx::Error::RowNotFound);
        }

        // Should return error if we can't find the task
        let _task_check = self.read_one_db(task_id).await?;

        let task_res: Task = sqlx::query_as!(
            Task,
            r#"UPDATE 
                task 
            SET 
                title = COALESCE($1, title), 
                description = COALESCE($2, description), 
                finished_at = COALESCE($3, finished_at), 
                priority = COALESCE($4, priority),
                edited_at = NOW() 
            WHERE 
                id = $5 
                AND deleted_at IS NULL 
            RETURNING id, 
                event_id, 
                creator_id, 
                title, 
                description, 
                finished_at, 
                priority as "priority!: TaskPriority", 
                accepts_staff, 
                created_at, 
                deleted_at, 
                edited_at
            "#,
            data.title,
            data.description,
            data.finished_at,
            data.priority as Option<TaskPriority>,
            task_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(task_res)
    }

    pub async fn _delete_task(&self, task_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _task_res = sqlx::query!(
            r#"UPDATE task
            SET deleted_at = NOW(), edited_at = NOW()
            WHERE id = $1
            AND deleted_at IS NULL
            "#,
            task_id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
