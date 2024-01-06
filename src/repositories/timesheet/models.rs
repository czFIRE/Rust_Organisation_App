use crate::models::ApprovalStatus;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TimesheetStructureData {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate
}

#[derive(Debug, FromRow)]
pub struct TimesheetWithEvent {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub manager_note: Option<String>,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub event_avatar_url: String,
    pub event_name: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct TimesheetCreateData {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub manager_note: Option<String>,
    // foreign keys
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct WorkdayUpdateData {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: Option<f32>,
    pub comment: Option<String>,
    pub is_editable: Option<bool>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TimesheetUpdateData {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub total_hours: Option<f32>,
    pub is_editable: Option<bool>,
    pub status: Option<ApprovalStatus>,
    pub manager_note: Option<String>,
    pub workdays: Option<Vec<WorkdayUpdateData>>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct TimesheetReadAllData {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct Workday {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: f32,
    pub comment: Option<String>,
    pub is_editable: bool,
    pub created_at: NaiveDate,
    pub edited_at: NaiveDate,
}

#[allow(dead_code)]
pub struct TimesheetWithWorkdays {
    pub timesheet: TimesheetWithEvent,
    pub workdays: Vec<Workday>,
}
