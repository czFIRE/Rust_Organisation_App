use crate::common::DbResult;
use crate::models::ApprovalStatus;
use crate::repositories::timesheet::models::{
    TimesheetCreateData, TimesheetReadAllData, TimesheetStructureData, TimesheetUpdateData,
    TimesheetWithEvent, TimesheetWithWorkdays, Workday,
};
use chrono::{Duration, NaiveDate};
use sqlx::postgres::PgPool;
use sqlx::{Postgres, Transaction};
use std::ops::DerefMut;
use std::sync::Arc;
use uuid::Uuid;

use super::models::WorkdayUpdateData;

#[derive(Clone)]
pub struct TimesheetRepository {
    pub pool: Arc<PgPool>,
}

impl TimesheetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // CRUD
    async fn create_workdays(
        &self,
        mut tx: Transaction<'_, Postgres>,
        timesheet_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> DbResult<Vec<Workday>> {
        let mut workdays = vec![];
        let mut workdate = start_date;
        while workdate <= end_date {
            let workday = sqlx::query_as!(
                Workday,
                r#"
                INSERT INTO workday (timesheet_id,
                                     date,
                                     is_editable)
                VALUES ($1, $2, $3)
                RETURNING timesheet_id,
                          date,
                          total_hours,
                          comment,
                          is_editable,
                          created_at,
                          edited_at;
                "#,
                timesheet_id,
                workdate,
                true
            )
            .fetch_one(tx.deref_mut())
            .await?;
            workdays.push(workday);
            workdate += Duration::days(1);
        }
        tx.commit().await?;
        Ok(workdays)
    }

    pub async fn create(&self, data: TimesheetCreateData) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet_structure = sqlx::query_as!(
            TimesheetStructureData,
            r#"
            INSERT INTO timesheet (start_date, end_date, user_id, company_id, event_id) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, start_date, end_date;
            "#,
            data.start_date,
            data.end_date,
            data.user_id,
            data.company_id,
            data.event_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        // ToDo: This can probably be done better, but we need the event there as well.
        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!:ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_structure.id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let workdays = self
            .create_workdays(
                tx,
                timesheet_structure.id,
                timesheet_structure.start_date,
                timesheet_structure.end_date,
            )
            .await?;

        // Tx commit in create_workdays

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _read_one(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let sheet_clone = timesheet.clone().unwrap();

        let workdays = sqlx::query_as!(
            Workday,
            r#"SELECT timesheet_id,
                    date, 
                    total_hours, 
                    comment AS "comment?", 
                    is_editable,
                    created_at,
                    edited_at 
            FROM workday 
            WHERE timesheet_id = $1
              AND date >= $2
              AND date <= $3;"#,
            timesheet_id,
            sheet_clone.start_date,
            sheet_clone.end_date,
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let result = TimesheetWithWorkdays {
            timesheet: timesheet.expect("Should be valid here."),
            workdays,
        };

        tx.commit().await?;

        Ok(result)
    }

    // Warning!! The tx will be commited, use this as the last call in a transaction.
    async fn _read_one_tx(
        &self,
        timesheet_id: Uuid,
        mut tx: Transaction<'_, Postgres>,
    ) -> DbResult<TimesheetWithEvent> {
        let timesheet = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at,
                   timesheet.edited_at
            FROM timesheet 
            JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.id = $1 
              AND timesheet.deleted_at IS NULL;
            "#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        tx.commit().await?;

        Ok(timesheet.expect("Should be valid here."))
    }

    pub async fn _read_all(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetWithEvent>> {
        // This is for redis.

        self._read_all_db(data).await
    }

    async fn _read_all_db(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetWithEvent>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at
            FROM timesheet 
             JOIN event ON timesheet.event_id = event.id
            WHERE timesheet.deleted_at IS NULL
            LIMIT $1 OFFSET $2;
            "#,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    pub async fn read_all_timesheets_per_employment(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        data: TimesheetReadAllData,
    ) -> DbResult<Vec<TimesheetWithEvent>> {
        // For Redis

        self.read_all_per_employment_db(user_id, company_id, data)
            .await
    }

    async fn read_all_per_employment_db(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        data: TimesheetReadAllData,
    ) -> DbResult<Vec<TimesheetWithEvent>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetWithEvent,
            r#"
            SELECT timesheet.id, 
                   timesheet.start_date, 
                   timesheet.end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "approval_status!: ApprovalStatus", 
                   manager_note AS "manager_note?", 
                   user_id, 
                   company_id,
                   event_id,
                   event.avatar_url AS event_avatar_url,
                   event.name AS event_name,
                   timesheet.created_at, 
                   timesheet.edited_at 
            FROM timesheet 
             JOIN event ON timesheet.event_id = event.id
            WHERE user_id = $1
              AND company_id = $2
              AND timesheet.deleted_at IS NULL
            LIMIT $3 OFFSET $4;
            "#,
            user_id,
            company_id,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    fn _is_data_empty(data: TimesheetUpdateData) -> bool {
        data.start_date.is_none()
            && data.end_date.is_none()
            && data.total_hours.is_none()
            && data.is_editable.is_none()
            && data.status.is_none()
            && data.manager_note.is_none()
            && data.workdays.is_none()
    }

    /* Methods for workday are kept in timesheet_repo because
     * workdays are semantically bound to timesheets.
     */
    pub async fn read_one_workday(&self, timesheet_id: Uuid, date: NaiveDate) -> DbResult<Workday> {
        let executor = self.pool.as_ref();

        let workday = sqlx::query_as!(
            Workday,
            r#"
            SELECT timesheet_id,
                   date,
                   total_hours,
                   comment,
                   is_editable,
                   created_at,
                   edited_at
            FROM workday
            WHERE timesheet_id = $1
              AND date = $2
              AND deleted_at IS NULL
            "#,
            timesheet_id,
            date,
        )
        .fetch_one(executor)
        .await?;

        Ok(workday)
    }

    pub async fn update_workday(
        &self,
        timesheet_id: Uuid,
        date: NaiveDate,
        data: WorkdayUpdateData,
    ) -> DbResult<Workday> {
        let executor = self.pool.as_ref();

        let workday = sqlx::query_as!(
            Workday,
            r#"
            UPDATE workday
            SET total_hours = COALESCE($1, total_hours),
                comment = COALESCE($2, comment),
                is_editable = COALESCE($3, is_editable),
                edited_at = NOW()
            WHERE timesheet_id = $4
              AND date = $5
              AND deleted_at IS NULL
            RETURNING timesheet_id,
                      date,
                      total_hours,
                      comment,
                      is_editable,
                      created_at,
                      edited_at;"#,
            data.total_hours,
            data.comment,
            data.is_editable,
            timesheet_id,
            date
        )
        .fetch_one(executor)
        .await?;

        Ok(workday)
    }

    pub async fn update(
        &self,
        timesheet_id: Uuid,
        data: TimesheetUpdateData,
    ) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        if Self::_is_data_empty(data.clone()) {
            return Err(sqlx::Error::TypeNotFound {
                type_name: "User error.".to_string(),
            });
        }

        sqlx::query!(
            r#"
            UPDATE timesheet
            SET start_date = COALESCE($1, start_date),
                end_date = COALESCE($2, end_date),
                total_hours = COALESCE($3, total_hours),
                is_editable = COALESCE($4, is_editable),
                status = COALESCE($5, status),
                manager_note = COALESCE($6, manager_note),
                edited_at = NOW()
            WHERE id = $7
              AND deleted_at IS NULL
            "#,
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.status as Option<ApprovalStatus>,
            data.manager_note,
            timesheet_id
        )
        .execute(tx.deref_mut())
        .await?;

        // This should likely be in a separate function.
        if data.workdays.is_some() {
            for workday in data.workdays.expect("Should be some!").into_iter() {
                sqlx::query!(
                    r#"
                    UPDATE workday
                    SET total_hours = COALESCE($1, total_hours),
                        comment = COALESCE($2, comment),
                        is_editable = COALESCE($3, is_editable),
                        edited_at = NOW()
                    WHERE timesheet_id = $4
                      AND date = $5
                      AND deleted_at IS NULL;"#,
                    workday.total_hours,
                    workday.comment,
                    workday.is_editable,
                    workday.timesheet_id,
                    workday.date
                )
                .execute(tx.deref_mut())
                .await?;
            }
        }

        /* You may think this is redundant, BUT
         *  since not all workdays may be edited,
         *  it's better to just retrieve all of them after the change.
         * Also we don't get event data from the update.
         * But if we get the time, we should think about improving this.
         */

        let workdays = sqlx::query_as!(
            Workday,
            r#"
            SELECT timesheet_id,
                    date, 
                    total_hours, 
                    comment AS "comment?", 
                    is_editable,
                    created_at,
                    edited_at 
            FROM workday 
            WHERE timesheet_id = $1;"#,
            timesheet_id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let timesheet = self._read_one_tx(timesheet_id, tx).await?;

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _delete(&self, timesheet_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            TimesheetStructureData,
            r#"UPDATE timesheet 
            SET edited_at = NOW(), 
            deleted_at = NOW() 
            WHERE id = $1
              AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date;"#,
            timesheet_id
        )
        .fetch_optional(executor)
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        /* We don't need to check if workdays are already
         * deleted because we do so for the timesheet.
         */
        sqlx::query!(
            "UPDATE workday
             SET edited_at = NOW(),
                 deleted_at = NOW()
             WHERE timesheet_id = $1;",
            timesheet_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn reset_timesheet(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetStructureData,
            r#"UPDATE timesheet 
            SET total_hours = 0,
                edited_at = NOW()
            WHERE id = $1
                  AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date;"#,
            timesheet_id
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if timesheet.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let workdays = sqlx::query_as!(
            Workday,
            r#"UPDATE workday
            SET total_hours = 0,
                comment = NULL,
                edited_at = NOW()
            WHERE timesheet_id = $1
              AND deleted_at IS NULL
            RETURNING timesheet_id,
                      date,
                      total_hours,
                      comment,
                      is_editable,
                      created_at,
                      edited_at;"#,
            timesheet_id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let timesheet = self._read_one_tx(timesheet_id, tx).await?;

        let edited_data = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(edited_data)
    }
}
