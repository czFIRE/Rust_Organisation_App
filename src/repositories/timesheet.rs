use crate::common::DbResult;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::postgres::PgPool;
use sqlx::prelude::FromRow;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Timesheet {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub is_editable: bool,
    pub manager_note: Option<String>,
    // foreign keys
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    // timestamps
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct TimesheetData {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub is_editable: bool,
    pub manager_note: Option<String>,
    // foreign keys
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Clone)]
pub struct TimesheetRepository {
    pub pool: Arc<PgPool>,
}

impl TimesheetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // CRUD

    pub async fn _create(&self, data: TimesheetData) -> DbResult<Timesheet> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            Timesheet,
            "INSERT INTO timesheet (start_date, end_date, total_hours, is_editable, manager_note, user_id, company_id, event_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.manager_note,
            data.user_id,
            data.company_id,
            data.event_id
        )
        .fetch_one(executor)
        .await?;

        Ok(timesheet)
    }

    pub async fn _read_one(&self, uuid: Uuid) -> DbResult<Timesheet> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(Timesheet, "SELECT * FROM timesheet WHERE id = $1;", uuid)
            .fetch_one(executor)
            .await?;

        Ok(timesheet)
    }

    pub async fn _read_all(&self) -> DbResult<Vec<Timesheet>> {
        let executor = self.pool.as_ref();

        let timesheets = sqlx::query_as!(Timesheet, "SELECT * FROM timesheet;")
            .fetch_all(executor)
            .await?;

        Ok(timesheets)
    }

    pub async fn _update(&self, uuid: Uuid, data: TimesheetData) -> DbResult<Timesheet> {
        let executor = self.pool.as_ref();

        let timesheet = sqlx::query_as!(
            Timesheet,
            "UPDATE timesheet SET start_date = $1, end_date = $2, total_hours = $3, is_editable = $4, manager_note = $5, user_id = $6, company_id = $7, event_id = $8 WHERE id = $9 RETURNING *;",
            data.start_date,
            data.end_date,
            data.total_hours,
            data.is_editable,
            data.manager_note,
            data.user_id,
            data.company_id,
            data.event_id,
            uuid
        )
        .fetch_one(executor)
        .await?;

        Ok(timesheet)
    }

    pub async fn _delete(&self, uuid: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            "UPDATE timesheet SET deleted_at = NOW() WHERE id = $1;",
            uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
