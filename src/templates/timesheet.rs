use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::ApprovalStatus;

use super::event::EventLiteTemplate;

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/workday.html")]
pub struct WorkdayTemplate {
    pub timesheet_id: Uuid,
    pub work_date: NaiveDate,
    pub total_hours: u8,
    pub comment: Option<String>,
    pub is_editable: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheet.html")]
pub struct TimesheetTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event: EventLiteTemplate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: u16,
    pub work_days: Vec<WorkdayTemplate>,
    pub calculated_wage: Option<u128>, // Mind this field: It isn't in the DB and needs to be calculated. This is in CZK.
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub manager_note: Option<String>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheet-lite.html")]
pub struct TimesheetLiteTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub event_avatar_url: String,
    pub event_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub has_note: bool, // This is just an indicator for the presence / absence of a manager's note.
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheets.html")]
pub struct TimesheetsTemplate {
    pub timesheets: Vec<TimesheetLiteTemplate>,
}
