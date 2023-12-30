use crate::common::DbResult;
use crate::models::ApprovalStatus;
use crate::repositories::timesheet::models::{
    TimesheetCreateData, TimesheetDb, TimesheetReadAllData, TimesheetUpdateData,
    TimesheetWithWorkdays, Workday,
};
use sqlx::postgres::PgPool;
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

    pub async fn _create(&self, data: TimesheetCreateData) -> DbResult<TimesheetDb> {
        let executor = self.pool.as_ref();

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
        .fetch_one(executor)
        .await?;

        Ok(timesheet)
    }

    pub async fn _read_one(&self, timesheet_id: Uuid) -> DbResult<TimesheetWithWorkdays> {
        let mut tx = self.pool.begin().await?;

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"
            SELECT id, start_date, end_date, total_hours, is_editable, status AS "status!:ApprovalStatus", manager_note, user_id, company_id, event_id, created_at, edited_at, deleted_at 
            FROM timesheet WHERE id = $1;
            "#,
            timesheet_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let workdays = sqlx::query_as!(
            Workday,
            "SELECT date, total_hours, comment, is_editable FROM workday WHERE timesheet_id = $1;",
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
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(
            TimesheetDb,
            r#"
            SELECT id, start_date, end_date, total_hours, is_editable, status AS "status!:ApprovalStatus", manager_note, user_id, company_id, event_id, created_at, edited_at, deleted_at 
            FROM timesheet LIMIT $1 OFFSET $2;
            "#,
            data.limit,
            data.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(timesheets)
    }

    pub async fn _update(
        &self,
        timesheet_id: Uuid,
        data: TimesheetUpdateData,
    ) -> DbResult<TimesheetDb> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            TimesheetDb,
            r#"
            UPDATE timesheet SET start_date = $1, end_date = $2, total_hours = $3, is_editable = $4, status = $5, manager_note = $6, user_id = $7, company_id = $8, event_id = $9, edited_at = NOW() 
            WHERE id = $10 RETURNING id, start_date, end_date, total_hours, is_editable, status AS "status!:ApprovalStatus", manager_note, user_id, company_id, event_id, created_at, edited_at, deleted_at;
            "#,
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.status as Option<ApprovalStatus>,
            data.manager_note,
            data.user_id,
            data.company_id,
            data.event_id,
            timesheet_id
        )
        .fetch_one(executor)
        .await?;

        Ok(timesheet)
    }

    pub async fn _delete(&self, timesheet_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            "UPDATE timesheet SET edited_at = NOW(), deleted_at = NOW() WHERE id = $1;",
            timesheet_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    // Other

    pub async fn _reset_timesheet(&self, uuid: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            "UPDATE workday SET total_hours = 0, comment = NULL, edited_at = NOW() WHERE timesheet_id = $1;",
            uuid
        )
        .execute(executor)
        .await?;

        sqlx::query!(
            "UPDATE workday SET total_hours = 0, comment = NULL, edited_at = NOW() WHERE timesheet_id = $1;",
            uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
