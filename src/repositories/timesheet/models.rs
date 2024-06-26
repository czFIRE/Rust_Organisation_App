use crate::models::{ApprovalStatus, EmploymentContract};
use crate::repositories::wage_preset::models::WagePreset;
use crate::utils::wage_calc::models::YearAndMonth;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use std::collections::HashMap;

#[derive(Debug, FromRow)]
pub struct TimesheetStructureData {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, FromRow)]
pub struct TimesheetWithEvent {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub is_editable: bool,
    pub approval_status: ApprovalStatus,
    pub manager_note: Option<String>,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub event_avatar_url: String,
    pub event_name: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct TimesheetCreateData {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct TimeRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct WorkdayUpdateData {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: Option<f32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct TimesheetUpdateData {
    pub is_editable: Option<bool>,
    pub status: Option<ApprovalStatus>,
    pub manager_note: Option<String>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct TimesheetReadAllData {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Workday {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: f32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Debug, FromRow)]
pub struct TimesheetWithWorkdays {
    pub timesheet: TimesheetWithEvent,
    pub workdays: Vec<Workday>,
}

#[derive(Debug)]
pub struct TimesheetsWithWorkdaysExtended {
    pub timesheets: Vec<TimesheetWithWorkdays>,
    pub hourly_wage: f64,
    pub employment_type: EmploymentContract,

    //
    // A hashmap mapping date to `WagePreset` elems.
    //
    // Note: Empty value means no matching `wage preset` was found.
    //
    pub date_to_wage_presets: HashMap<YearAndMonth, Option<WagePreset>>,
}
