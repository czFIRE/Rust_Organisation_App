use crate::repositories::timesheet::models::{TimesheetWithEvent, Workday};
use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{models::ApprovalStatus, repositories::timesheet::models::TimesheetWithWorkdays};

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/workday.html")]
pub struct WorkdayTemplate {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: f32,
    pub comment: Option<String>,
    pub is_editable: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<Workday> for WorkdayTemplate {
    fn from(workday: Workday) -> Self {
        WorkdayTemplate {
            timesheet_id: workday.timesheet_id,
            date: workday.date,
            total_hours: workday.total_hours,
            comment: workday.comment,
            is_editable: workday.is_editable,
            created_at: workday.created_at,
            edited_at: workday.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/workday-edit.html")]
pub struct WorkdayEditTemplate {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: f32,
    pub comment: Option<String>,
    pub is_editable: bool,
}

impl From<Workday> for WorkdayEditTemplate {
    fn from(workday: Workday) -> Self {
        WorkdayEditTemplate {
            timesheet_id: workday.timesheet_id,
            date: workday.date,
            total_hours: workday.total_hours,
            comment: workday.comment,
            is_editable: workday.is_editable,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheet.html")]
pub struct TimesheetTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub event_avatar_url: String,
    pub event_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_hours: f32,
    pub work_days: Vec<WorkdayTemplate>,
    pub calculated_wage: u128, // Mind this field: It isn't in the DB and needs to be calculated. This is in CZK.
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub manager_note: Option<String>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<TimesheetWithWorkdays> for TimesheetTemplate {
    fn from(full_timesheet: TimesheetWithWorkdays) -> Self {
        let workdays = full_timesheet
            .workdays
            .into_iter()
            .map(|workday| WorkdayTemplate {
                timesheet_id: workday.timesheet_id,
                date: workday.date,
                total_hours: workday.total_hours,
                comment: workday.comment,
                is_editable: workday.is_editable,
                created_at: workday.created_at,
                edited_at: workday.edited_at,
            })
            .collect();

        TimesheetTemplate {
            id: full_timesheet.timesheet.id,
            user_id: full_timesheet.timesheet.user_id,
            company_id: full_timesheet.timesheet.company_id,
            event_id: full_timesheet.timesheet.event_id,
            event_avatar_url: full_timesheet.timesheet.event_avatar_url,
            event_name: full_timesheet.timesheet.event_name,
            start_date: full_timesheet.timesheet.start_date,
            end_date: full_timesheet.timesheet.end_date,
            total_hours: full_timesheet.timesheet.total_hours,
            work_days: workdays,
            calculated_wage: 0,
            is_editable: full_timesheet.timesheet.is_editable,
            status: full_timesheet.timesheet.approval_status,
            manager_note: full_timesheet.timesheet.manager_note,
            created_at: full_timesheet.timesheet.created_at,
            edited_at: full_timesheet.timesheet.edited_at,
        }
    }
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

impl From<TimesheetWithEvent> for TimesheetLiteTemplate {
    fn from(timesheet: TimesheetWithEvent) -> Self {
        TimesheetLiteTemplate {
            id: timesheet.id,
            user_id: timesheet.user_id,
            company_id: timesheet.company_id,
            event_id: timesheet.event_id,
            event_avatar_url: timesheet.event_avatar_url,
            event_name: timesheet.event_name,
            start_date: timesheet.start_date,
            end_date: timesheet.end_date,
            is_editable: timesheet.is_editable,
            status: timesheet.approval_status,
            has_note: timesheet.manager_note.is_some(),
            created_at: timesheet.created_at,
            edited_at: timesheet.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheets.html")]
pub struct TimesheetsTemplate {
    pub timesheets: Vec<TimesheetLiteTemplate>,
}
