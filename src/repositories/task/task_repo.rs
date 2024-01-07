use crate::{common::DbResult, repositories::task::models::TaskUserFlattened};
use async_trait::async_trait;
use sqlx::{postgres::PgPool, Postgres, Transaction};
use std::{ops::DerefMut, sync::Arc};
use uuid::Uuid;

use super::models::{NewTask, Task, TaskData, TaskExtended, TaskFilter};

use crate::models::{Gender, TaskPriority, UserRole, UserStatus};

#[derive(Clone)]
pub struct TaskRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for TaskRepository {
    /// Database repository constructor
    #[must_use]
    fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Method allowing the database repository to disconnect from the database pool gracefully
    async fn disconnect(&mut self) -> () {
        self.pool.close().await;
    }
}

impl TaskRepository {
    pub async fn create(&self, data: NewTask) -> DbResult<TaskExtended> {
        let mut tx = self.pool.begin().await?;

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
        .fetch_one(tx.deref_mut())
        .await?;

        let new_task = self.read_one_tx(new_task.id, tx).await?;

        Ok(new_task)
    }

    pub async fn read_one(&self, task_id: Uuid) -> DbResult<TaskExtended> {
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
            INNER JOIN event_staff ON task.creator_id=event_staff.id
            INNER JOIN user_record ON event_staff.user_id=user_record.id 
            WHERE task.id=$1"#,
            task_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(task_user_flattened.into())
    }

    async fn read_one_tx(
        &self,
        task_id: Uuid,
        mut tx: Transaction<'_, Postgres>,
    ) -> DbResult<TaskExtended> {
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
            INNER JOIN event_staff ON task.creator_id=event_staff.id
            INNER JOIN user_record ON event_staff.user_id=user_record.id 
            WHERE task.id=$1"#,
            task_id,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        tx.commit().await?;

        Ok(task_user_flattened.into())
    }

    pub async fn read_all(&self, filter: TaskFilter) -> DbResult<Vec<TaskExtended>> {
        let executor = self.pool.as_ref();

        let tasks: Vec<TaskUserFlattened> = sqlx::query_as!(
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
            INNER JOIN event_staff ON task.creator_id=event_staff.id
            INNER JOIN user_record ON event_staff.user_id=user_record.id 
            LIMIT $1 OFFSET $2"#,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(tasks.into_iter().map(|t| t.into()).collect())
    }

    pub async fn read_all_for_event(
        &self,
        event_id: Uuid,
        filter: TaskFilter,
    ) -> DbResult<Vec<TaskExtended>> {
        let executor = self.pool.as_ref();

        let tasks: Vec<TaskUserFlattened> = sqlx::query_as!(
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
            INNER JOIN event_staff ON task.creator_id=event_staff.id
            INNER JOIN user_record ON event_staff.user_id=user_record.id 
            WHERE task.event_id=$1
              AND task.deleted_at IS NULL
            LIMIT $2 OFFSET $3"#,
            event_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(tasks.into_iter().map(|t| t.into()).collect())
    }

    pub async fn update(&self, task_id: Uuid, data: TaskData) -> DbResult<TaskExtended> {
        if data.description.is_none()
            && data.finished_at.is_none()
            && data.priority.is_none()
            && data.title.is_none()
        {
            // TODO - add better error
            return Err(sqlx::Error::TypeNotFound {
                type_name: "User Error".to_string(),
            });
        }

        let mut tx = self.pool.begin().await?;

        let task_res: Option<Task> = sqlx::query_as!(
            Task,
            r#"UPDATE 
                task 
            SET 
                title = COALESCE($1, title), 
                description = COALESCE($2, description), 
                finished_at = COALESCE($3, finished_at), 
                priority = COALESCE($4, priority),
                accepts_staff = COALESCE($5, accepts_staff),
                edited_at = NOW() 
            WHERE 
                id = $6 
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
                edited_at;
            "#,
            data.title,
            data.description,
            data.finished_at,
            data.priority as Option<TaskPriority>,
            data.accepts_staff,
            task_id,
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if task_res.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let task = self
            .read_one_tx(task_res.expect("Should be some.").id, tx)
            .await?;

        Ok(task)
    }

    pub async fn delete(&self, task_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let result = sqlx::query_as!(
            Task,
            r#"UPDATE task
            SET deleted_at = NOW(), 
                edited_at = NOW()
            WHERE id = $1
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
                edited_at;
            "#,
            task_id,
        )
        .fetch_optional(executor)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
