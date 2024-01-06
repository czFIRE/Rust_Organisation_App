use crate::common::DbResult;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{Event, EventData, EventFilter, NewEvent};

#[derive(Clone)]
pub struct EventRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for EventRepository {
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

impl EventRepository {
    pub async fn create(&self, data: NewEvent) -> DbResult<Event> {
        let executor = self.pool.as_ref();

        let new_event: Event = sqlx::query_as!(
            Event,
            r#" INSERT INTO event (
                name, description, website, start_date, end_date
                ) VALUES 
                ($1, $2, $3, $4, $5) RETURNING id, 
                name, 
                description, 
                website, 
                accepts_staff, 
                start_date, 
                end_date, 
                avatar_url, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.name,
            data.description,
            data.website,
            data.start_date,
            data.end_date,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_event)
    }

    pub async fn read_one(&self, event_id: Uuid) -> DbResult<Event> {
        // Redis here
        self.read_one_db(event_id).await
    }

    async fn read_one_db(&self, event_id: Uuid) -> DbResult<Event> {
        let executor = self.pool.as_ref();

        let event: Event = sqlx::query_as!(
            Event,
            r#"SELECT * 
                   FROM event 
                   WHERE id = $1
                     AND deleted_at IS NULL;"#,
            event_id
        )
        .fetch_one(executor)
        .await?;

        Ok(event)
    }

    // If you would like to get all companies for an event it's in the associated company repo
    pub async fn read_all(&self, filter: EventFilter) -> DbResult<Vec<Event>> {
        let executor = self.pool.as_ref();

        let events: Vec<Event> = match filter.accepts_staff {
            Some(accepts_staff) => {
                sqlx::query_as!(
                    Event,
                    r#" SELECT * 
                        FROM event 
                        WHERE accepts_staff = $1 
                          AND deleted_at IS NULL
                        LIMIT $2 
                        OFFSET $3;"#,
                    accepts_staff,
                    filter.limit,
                    filter.offset,
                )
                .fetch_all(executor)
                .await?
            }
            None => {
                sqlx::query_as!(
                    Event,
                    r#" SELECT * 
                        FROM event
                        WHERE deleted_at IS NULL 
                        LIMIT $1 
                        OFFSET $2;"#,
                    filter.limit,
                    filter.offset,
                )
                .fetch_all(executor)
                .await?
            }
        };

        Ok(events)
    }

    pub async fn update(&self, event_id: Uuid, data: EventData) -> DbResult<Event> {
        if data.name.is_none()
            && data.description.is_none()
            && data.website.is_none()
            && data.start_date.is_none()
            && data.end_date.is_none()
            && data.avatar_url.is_none()
        {
            return Err(sqlx::Error::RowNotFound);
        }

        let executor = self.pool.as_ref();

        let event = sqlx::query_as!(
            Event,
            r#" UPDATE event SET 
                name = COALESCE($1, name), 
                description = COALESCE($2, description), 
                website = COALESCE($3, website), 
                start_date = COALESCE($4, start_date), 
                end_date = COALESCE($5, end_date), 
                avatar_url = COALESCE($6, avatar_url), 
                edited_at = NOW() 
                WHERE id = $7
                  AND deleted_at IS NULL 
                RETURNING id, 
                name, 
                description, 
                website, 
                accepts_staff, 
                start_date, 
                end_date, 
                avatar_url, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.name,
            data.description,
            data.website,
            data.start_date,
            data.end_date,
            data.avatar_url,
            event_id,
        )
        .fetch_optional(executor)
        .await?;

        if event.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(event.expect("Should be valid"))
    }

    pub async fn delete(&self, event_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let result = sqlx::query_as!(
            Event,
            r#"UPDATE event
            SET deleted_at = NOW(), edited_at = NOW()
            WHERE id = $1
            AND deleted_at IS NULL
            RETURNING id, 
                name, 
                description, 
                website, 
                accepts_staff, 
                start_date, 
                end_date, 
                avatar_url, 
                created_at, 
                edited_at, 
                deleted_at;"#,
            event_id,
        )
        .fetch_optional(executor)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
