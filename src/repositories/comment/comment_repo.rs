use crate::common::DbResult;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    Comment, CommentData, CommentExtended, CommentFilter, CommentUserFlattened, NewComment,
};

use crate::models::{Gender, UserRole, UserStatus};

#[derive(Clone)]
pub struct CommentRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for CommentRepository {
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

impl CommentRepository {
    pub async fn create(&self, data: NewComment) -> DbResult<Comment> {
        if data.content.chars().count() < 1 {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        if (data.event_id.is_none() && data.task_id.is_none())
            || (data.event_id.is_some() && data.task_id.is_some())
        {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        let executor = self.pool.as_ref();

        let comment: Comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comment (author_id, event_id, task_id, content)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            data.author_id,
            data.event_id,
            data.task_id,
            data.content,
        )
        .fetch_one(executor)
        .await?;

        Ok(comment)
    }

    pub async fn read_one(&self, comment_id: Uuid) -> DbResult<CommentExtended> {
        // Redis here
        self.read_one_db(comment_id).await
    }

    async fn read_one_db(&self, comment_id: Uuid) -> DbResult<CommentExtended> {
        let executor = self.pool.as_ref();

        let comment: CommentUserFlattened = sqlx::query_as!(
            CommentUserFlattened,
            r#"
            SELECT 
                comment.id AS comment_id, 
                comment.author_id AS comment_author_id, 
                comment.event_id AS comment_event_id, 
                comment.task_id AS comment_task_id, 
                comment.content AS comment_content, 
                comment.created_at AS comment_created_at, 
                comment.edited_at AS comment_edited_at, 
                comment.deleted_at AS comment_deleted_at, 
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
            FROM 
                comment 
                INNER JOIN user_record ON comment.author_id = user_record.id 
            WHERE 
                comment.id = $1          
            "#,
            comment_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(comment.into())
    }

    pub async fn read_all_per_event(
        &self,
        event_id: Uuid,
        filter: CommentFilter,
    ) -> DbResult<Vec<CommentExtended>> {
        let executor = self.pool.as_ref();

        let comments: Vec<CommentUserFlattened> = sqlx::query_as!(
            CommentUserFlattened,
            r#"
            SELECT 
                comment.id AS comment_id, 
                comment.author_id AS comment_author_id, 
                comment.event_id AS comment_event_id, 
                comment.task_id AS comment_task_id, 
                comment.content AS comment_content, 
                comment.created_at AS comment_created_at, 
                comment.edited_at AS comment_edited_at, 
                comment.deleted_at AS comment_deleted_at, 
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
            FROM 
                comment 
                INNER JOIN user_record ON comment.author_id = user_record.id 
            WHERE 
                comment.event_id = $1    
            LIMIT $2 OFFSET $3      
            "#,
            event_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(comments.into_iter().map(|c| c.into()).collect())
    }

    pub async fn read_all_per_task(
        &self,
        task_id: Uuid,
        filter: CommentFilter,
    ) -> DbResult<Vec<CommentExtended>> {
        let executor = self.pool.as_ref();

        let comments: Vec<CommentUserFlattened> = sqlx::query_as!(
            CommentUserFlattened,
            r#"
            SELECT 
                comment.id AS comment_id, 
                comment.author_id AS comment_author_id, 
                comment.event_id AS comment_event_id, 
                comment.task_id AS comment_task_id, 
                comment.content AS comment_content, 
                comment.created_at AS comment_created_at, 
                comment.edited_at AS comment_edited_at, 
                comment.deleted_at AS comment_deleted_at, 
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
            FROM 
                comment 
                INNER JOIN user_record ON comment.author_id = user_record.id 
            WHERE 
                comment.task_id = $1    
            LIMIT $2 OFFSET $3      
            "#,
            task_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(comments.into_iter().map(|c| c.into()).collect())
    }

    pub async fn update(&self, comment_id: Uuid, data: CommentData) -> DbResult<Comment> {
        if data.content.chars().count() < 1 {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        let executor = self.pool.as_ref();

        let comment_check = self.read_one_db(comment_id).await?;

        if comment_check.deleted_at.is_some() {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        let comment: Comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comment
            SET content = $1, 
            edited_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
            data.content,
            comment_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(comment)
    }

    pub async fn delete(&self, comment_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let comment_check = self.read_one_db(comment_id).await?;

        if comment_check.deleted_at.is_some() {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            r#"
            UPDATE comment 
            SET deleted_at = NOW(), 
            edited_at = NOW()
            WHERE id = $1
            "#,
            comment_id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
