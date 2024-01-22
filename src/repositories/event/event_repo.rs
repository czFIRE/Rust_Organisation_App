use crate::{
    common::DbResult,
    models::{Association, EventRole},
    repositories::{
        associated_company::models::AssociatedCompanyMinimal,
        event_staff::models::StaffInfo,
        timesheet::models::{TimeRange, TimesheetStructureData},
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use sqlx::{postgres::PgPool, Postgres, Transaction};
use std::{ops::DerefMut, sync::Arc};
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
        let mut tx = self.pool.begin().await?;

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
        .fetch_one(tx.deref_mut())
        .await?;

        let _: AssociatedCompanyMinimal = sqlx::query_as!(
            AssociatedCompanyMinimal,
            r#"
            INSERT INTO associated_company
                (company_id, event_id, type)
            VALUES ($1, $2, $3)
            RETURNING company_id,
                      event_id;
            "#,
            data.company_id,
            new_event.id,
            Association::Organizer as Association,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let _: StaffInfo = sqlx::query_as!(
            StaffInfo,
            r#"
            INSERT INTO event_staff
                ( user_id, company_id, event_id, role )
            VALUES
                ( $1, $2, $3, $4 )
            RETURNING id;
            "#,
            data.creator_id,
            data.company_id,
            new_event.id,
            EventRole::Organizer as EventRole,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        tx.commit().await?;

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
                        ORDER BY name
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
                        ORDER BY name
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

    pub async fn update_timesheet_range_for_event(
        &self,
        event_id: Uuid,
        data: TimeRange,
        mut tx: Transaction<'_, Postgres>,
    ) -> DbResult<()> {
        let updated_sheets = sqlx::query_as!(
            TimesheetStructureData,
            r#"
            UPDATE timesheet
            SET start_date = $1,
                end_date = $2,
                edited_at = NOW()
            WHERE event_id = $3
              AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date;
            "#,
            data.start_date,
            data.end_date,
            event_id,
        )
        .fetch_all(tx.deref_mut())
        .await?;

        for sheet in updated_sheets.into_iter() {
            let start_date_time: DateTime<Utc> = Utc
                .with_ymd_and_hms(
                    sheet.start_date.year(),
                    sheet.start_date.month(),
                    sheet.start_date.day(),
                    0,
                    0,
                    0,
                )
                .unwrap();
            let end_date_time: DateTime<Utc> = Utc
                .with_ymd_and_hms(
                    sheet.end_date.year(),
                    sheet.end_date.month(),
                    sheet.end_date.day(),
                    0,
                    0,
                    0,
                )
                .unwrap();
            sqlx::query!(
                r#"DELETE FROM workday
                   WHERE timesheet_id = $1 AND (date < $2 OR date > $3);"#,
                sheet.id,
                sheet.start_date,
                sheet.end_date,
            )
            .execute(tx.deref_mut())
            .await?;

            sqlx::query!(
                r#"INSERT INTO workday (timesheet_id, date)
                 SELECT $1, curr_date
                 FROM generate_series($2, $3, interval '1 day') as curr_date
                 ON CONFLICT DO NOTHING;"#,
                sheet.id,
                start_date_time,
                end_date_time,
            )
            .execute(tx.deref_mut())
            .await?;

            sqlx::query!(
                r#"
                UPDATE timesheet 
                SET total_hours = (SELECT SUM(total_hours) 
                                  FROM workday 
                                  WHERE workday.timesheet_id = timesheet_id
                                  GROUP BY timesheet_id)
                WHERE id = $1 AND deleted_at IS NULL;"#,
                sheet.id,
            )
            .execute(tx.deref_mut())
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn update(&self, event_id: Uuid, data: EventData) -> DbResult<Event> {
        if data.name.is_none()
            && data.description.is_none()
            && data.website.is_none()
            && data.start_date.is_none()
            && data.end_date.is_none()
            && data.accepts_staff.is_none()
            && data.avatar_url.is_none()
        {
            return Err(sqlx::Error::TypeNotFound {
                type_name: "User Error".to_string(),
            });
        }

        let mut tx = self.pool.begin().await?;

        let event = sqlx::query_as!(
            Event,
            r#" UPDATE event SET 
                name = COALESCE($1, name), 
                description = COALESCE($2, description), 
                website = COALESCE($3, website), 
                start_date = COALESCE($4, start_date), 
                end_date = COALESCE($5, end_date), 
                accepts_staff = COALESCE($6, accepts_staff),
                avatar_url = COALESCE($7, avatar_url),
                edited_at = NOW() 
                WHERE id = $8
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
            data.accepts_staff,
            data.avatar_url,
            event_id,
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if event.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let result_event = event.expect("Should be valid");

        if data.start_date.is_some() || data.end_date.is_some() {
            let time_range = TimeRange {
                start_date: data.start_date.unwrap_or(result_event.start_date),
                end_date: data.end_date.unwrap_or(result_event.end_date),
            };

            self.update_timesheet_range_for_event(event_id, time_range, tx)
                .await?;
        } else {
            tx.commit().await?;
        }

        Ok(result_event)
    }

    pub async fn switch_accepts_staff(&self, event_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _ = sqlx::query_as!(
            Event,
            r#"UPDATE event
               SET accepts_staff = NOT accepts_staff,
                   edited_at = NOW()
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
        .fetch_one(executor)
        .await?;

        Ok(())
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
