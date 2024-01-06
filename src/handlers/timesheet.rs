use actix_web::{delete, get, patch, post, web, HttpResponse, http};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{handlers::common::{QueryParams, extract_user_company_ids}, models::ApprovalStatus, repositories::timesheet::{timesheet_repo::TimesheetRepository, models::TimesheetReadAllData}, errors::parse_error};

#[derive(Deserialize)]
pub struct NewTimesheetData {
    user_id: Uuid,
    company_id: Uuid,
    event_id: Uuid,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct WorkDay {
    total_hours: Option<u8>, // If none, the default is 0.
    comment: Option<String>,
    is_editable: Option<bool>,
}

#[derive(Deserialize)]
pub struct TimesheetData {
    work_days: Option<Vec<WorkDay>>,
    is_editable: Option<bool>,
    status: Option<ApprovalStatus>,
    manager_note: Option<String>,
}

#[get("/user/{user_id}/employment/{company_id}/sheet")]
pub async fn get_all_timesheets_for_employment(
    path: web::Path<(String, String)>,
    query: web::Query<TimesheetReadAllData>,
    timesheet_repo: web::Data<TimesheetRepository>
) -> HttpResponse {
    let query_params = query.into_inner();

    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_user_company_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (company_id, user_id) = parsed_ids.unwrap();
    todo!()
}

// Note: This is done automatically whenever event_staff is accepted to work on an event.
#[post("/timesheet")]
pub async fn create_timesheet(_new_timesheet: web::Form<NewTimesheetData>) -> HttpResponse {
    // Default approval status: NotRequested
    todo!()
}

#[get("/timesheet/{timesheet_id}")]
pub async fn get_timesheet(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[patch("/timesheet/{timesheet_id}")]
pub async fn update_timesheet(
    _timesheet_id: web::Path<String>,
    _timesheet_data: web::Form<TimesheetData>,
) -> HttpResponse {
    todo!()
}

/*
* Reset every work_day for a corresponding timesheet, as well as worked_hours and comments in the timesheet record.
*/
#[delete("/timesheet/{timesheet_id}/workdays")]
pub async fn reset_timesheet_data(_timesheet_id: web::Path<String>) -> HttpResponse {
    todo!()
}
