use crate::repositories::timesheet::models::TimesheetWithEvent;
use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use std::collections::HashMap;
use uuid::Uuid;

use crate::utils::year_and_month::YearAndMonth;
use crate::{models::ApprovalStatus, repositories::timesheet::models::TimesheetWithWorkdays};

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/workday.html")]
pub struct WorkdayTemplate {
    pub timesheet_id: Uuid,
    pub date: NaiveDate,
    pub total_hours: f32,
    pub comment: String,
    pub is_editable: bool,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheet.html")]
//
// Note: We deliberately don't supply `calculated_wage`, this value will be
//       presented to a user only on request (because this value is not
//       stored in DB and is computationally demanding).
//
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
    pub is_editable: bool,
    pub status: ApprovalStatus,
    pub manager_note: String,
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
                comment: workday.comment.unwrap_or("No comment.".to_string()),
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
            is_editable: full_timesheet.timesheet.is_editable,
            status: full_timesheet.timesheet.approval_status,
            manager_note: full_timesheet
                .timesheet
                .manager_note
                .unwrap_or("No note.".to_string()),
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
    // This is just an indicator for the presence / absence of a manager's note.
    pub has_note: bool,
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

#[derive(Debug, Deserialize)]
pub struct DetailedWage {
    // A tax value which is used for computing employee's `net wage` and such.
    pub tax_base: f32,
    //
    // A final wage an employee is supposed to be given.
    //
    pub net_wage: f32,

    // Number of worked hours per timesheet or a whole month.
    pub worked_hours: f32,

    // Note: In `wage_currency` units.
    pub employee_social_insurance: f32,
    pub employee_health_insurance: f32,
    pub employer_social_insurance: f32,
    pub employer_health_insurance: f32,
}

impl Default for DetailedWage {
    fn default() -> DetailedWage {
        DetailedWage {
            tax_base: 0.0,
            net_wage: 0.0,
            worked_hours: 0.0,
            employee_social_insurance: 0.0,
            employee_health_insurance: 0.0,
            employer_social_insurance: 0.0,
            employer_health_insurance: 0.0,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/timesheet/timesheet_wage_detailed.html")]
pub struct TimesheetWageDetailed {
    // A total wage data for selected timesheet's work.
    pub total_wage: DetailedWage,

    pub wage_currency: String,
    pub hourly_wage: f32,

    //
    // A wage employee is supposed to get for selected event's work,
    // divided into months.
    //
    pub month_to_detailed_wage: HashMap<YearAndMonth, DetailedWage>,

    // Note: Empty value means a wage computation went well and data are valid.
    pub error_option: Option<String>,
}

impl Default for TimesheetWageDetailed {
    fn default() -> TimesheetWageDetailed {
        TimesheetWageDetailed {
            total_wage: DetailedWage::default(),
            wage_currency: "".to_string(),
            hourly_wage: 0.0,
            month_to_detailed_wage: HashMap::new(),
            error_option: None,
        }
    }
}
