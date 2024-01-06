use crate::common::DbResult;
use crate::models::ApprovalStatus;
use crate::repositories::timesheet::models::{
    TimesheetCreateData, TimesheetDb, TimesheetReadAllData, TimesheetUpdateData,
    TimesheetWithWorkdays, Workday,
};
use chrono::{Duration, NaiveDate};
use sqlx::postgres::PgPool;
use sqlx::{Postgres, Transaction};
use std::ops::DerefMut;
use std::sync::Arc;
use uuid::Uuid;

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
        Ok(workdays)
    }

    pub async fn _create(&self, data: TimesheetCreateData) -> DbResult<TimesheetWithWorkdays> {
        // let executor = self.pool.as_ref();
        let mut tx = self.pool.begin().await?;

        if data.manager_note.is_some()
            && data.manager_note.clone().expect("Should be some").len() == 0
        {
            return Err(sqlx::Error::ColumnNotFound(
                "Manager note is some and empty.".to_string(),
            )); // ToDo: Rewrite for a proper error.
        }

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"
            INSERT INTO timesheet (start_date, end_date, total_hours, is_editable, status, manager_note, user_id, company_id, event_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
            RETURNING id, start_date, end_date, total_hours, is_editable, status AS "status!:ApprovalStatus", manager_note, user_id, company_id, event_id, created_at, edited_at, deleted_at;
            "#,
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.status as ApprovalStatus,
            data.manager_note,
            data.user_id,
            data.company_id,
            data.event_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let workdays = Self::create_workdays(
            &self,
            tx,
            timesheet.id,
            timesheet.start_date,
            timesheet.end_date,
        )
        .await?;

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _read_one(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"
            SELECT id, 
                   start_date, 
                   end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "status!:ApprovalStatus", 
                   manager_note, 
                   user_id, 
                   company_id, 
                   event_id, 
                   created_at, 
                   edited_at, 
                   deleted_at 
            FROM timesheet WHERE id = $1 AND deleted_at IS NULL;
            "#,
            timesheet_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        if timesheet.deleted_at.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        let workdays = sqlx::query_as!(
            Workday,
            "SELECT timesheet_id,
                    date, 
                    total_hours, 
                    comment, 
                    is_editable,
                    created_at,
                    edited_at 
            FROM workday WHERE timesheet_id = $1;",
            timesheet_id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        tx.commit().await?;

        Ok(result)
    }

    pub async fn _read_all(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetDb>> {
        // This is for redis.

        self._read_all_db(data).await
    }

    async fn _read_all_db(&self, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetDb>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetDb,
            r#"
            SELECT id, start_date, end_date, total_hours, is_editable, status AS "status!:ApprovalStatus", manager_note, user_id, company_id, event_id, created_at, edited_at, deleted_at 
            FROM timesheet 
            WHERE deleted_at IS NULL
            LIMIT $1 OFFSET $2;
            "#,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    pub async fn read_all_timesheets_per_employment(&self, user_id: Uuid, company_id: Uuid, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetDb>> {
        // For Redis

        self.read_all_per_employment_db(user_id, company_id, data).await
    }

    async fn read_all_per_employment_db(&self, user_id: Uuid, company_id: Uuid, data: TimesheetReadAllData) -> DbResult<Vec<TimesheetDb>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetDb,
            r#"
            SELECT id, 
                   start_date, 
                   end_date, 
                   total_hours, 
                   is_editable, 
                   status AS "status!:ApprovalStatus", 
                   manager_note, 
                   user_id, 
                   company_id,
                   event_id, 
                   created_at, 
                   edited_at, 
                   deleted_at 
            FROM timesheet 
            WHERE user_id = $1
              AND company_id = $2
              AND deleted_at IS NULL
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

    pub async fn _update(
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

        let timesheet = sqlx::query_as!(
            TimesheetDb,
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
            RETURNING id,
                      start_date,
                      end_date,
                      total_hours,
                      is_editable,
                      status AS "status!:ApprovalStatus",
                      manager_note,
                      user_id,
                      company_id,
                      event_id,
                      created_at,
                      edited_at,
                      deleted_at;
            "#,
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.status as Option<ApprovalStatus>,
            data.manager_note,
            timesheet_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        if timesheet.deleted_at.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

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
         */
        let workdays = sqlx::query_as!(
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
            WHERE timesheet_id = $1;"#,
            timesheet.id
        )
        .fetch_all(tx.deref_mut())
        .await?;

        let result = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(result)
    }

    pub async fn _delete(&self, timesheet_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"UPDATE timesheet 
            SET edited_at = NOW(), 
            deleted_at = NOW() 
            WHERE id = $1
              AND deleted_at IS NULL
            RETURNING id,
                      start_date,
                      end_date,
                      total_hours,
                      is_editable,
                      status AS "status!:ApprovalStatus",
                      manager_note,
                      user_id,
                      company_id,
                      event_id,
                      created_at,
                      edited_at,
                      deleted_at;"#,
            timesheet_id
        )
        .fetch_all(executor)
        .await?;

        if timesheet.len() == 0 {
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

    pub async fn _reset_timesheet(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"UPDATE timesheet 
            SET total_hours = 0,
                edited_at = NOW()
            WHERE id = $1
            RETURNING id,
                      start_date,
                      end_date,
                      total_hours,
                      is_editable,
                      status AS "status!:ApprovalStatus",
                      manager_note,
                      user_id,
                      company_id,
                      event_id,
                      created_at,
                      edited_at,
                      deleted_at;"#,
            timesheet_id
        )
        .fetch_one(executor)
        .await?;

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
        .fetch_all(executor)
        .await?;

        let edited_data = TimesheetWithWorkdays {
            timesheet,
            workdays,
        };

        Ok(edited_data)
    }
}
