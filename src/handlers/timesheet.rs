use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    handlers::common::{extract_user_company_ids, QueryParams},
    models::ApprovalStatus,
    repositories::timesheet::{
        models::{TimesheetCreateData, TimesheetReadAllData},
        timesheet_repo::TimesheetRepository,
    },
    templates::{
        event::EventLiteTemplate,
        timesheet::{
            TimesheetLiteTemplate, TimesheetTemplate, TimesheetsTemplate, WorkdayTemplate,
        },
    },
};

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
    timesheet_repo: web::Data<TimesheetRepository>,
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
    let result = timesheet_repo
        .read_all_timesheets_per_employment(user_id, company_id, query_params)
        .await;

    if let Ok(timesheets) = result {
        let timesheet_vec = timesheets
            .into_iter()
            .map(|timesheet| TimesheetLiteTemplate {
                id: timesheet.id,
                user_id: timesheet.user_id,
                company_id: timesheet.company_id,
                event_id: timesheet.event_id,
                event_avatar_url: timesheet.event_avatar_url,
                event_name: timesheet.event_name,
                start_date: timesheet.start_date,
                end_date: timesheet.end_date,
                is_editable: timesheet.is_editable,
                status: timesheet.status,
                has_note: timesheet.manager_note.is_some(),
                created_at: timesheet.created_at,
                edited_at: timesheet.edited_at,
            })
            .collect();

        let template = TimesheetsTemplate {
            timesheets: timesheet_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }

    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

// Note: This is done automatically whenever event_staff is accepted to work on an event.
#[post("/timesheet")]
pub async fn create_timesheet(
    new_timesheet: web::Form<TimesheetCreateData>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let result = timesheet_repo.create(new_timesheet.into_inner()).await;

    if let Ok(full_timesheet) = result {
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

        let template = TimesheetTemplate {
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
            status: full_timesheet.timesheet.status,
            manager_note: full_timesheet
                .timesheet
                .manager_note
                .unwrap_or("No note.".to_string()),
            created_at: full_timesheet.timesheet.created_at,
            edited_at: full_timesheet.timesheet.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created().body(body.expect("Should be valid now."));
    }

    let error = result.err().expect("Should be error here.");
    return match error {
        sqlx::Error::Database(err) => {
            if err.is_unique_violation()
                || err.is_check_violation()
                || err.is_foreign_key_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    };
}

#[get("/timesheet/{timesheet_id}")]
pub async fn get_timesheet(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = timesheet_repo._read_one(parsed_id).await;

    if let Ok(full_timesheet) = result {
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

        let template = TimesheetTemplate {
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
            status: full_timesheet.timesheet.status,
            manager_note: full_timesheet
                .timesheet
                .manager_note
                .unwrap_or("No note.".to_string()),
            created_at: full_timesheet.timesheet.created_at,
            edited_at: full_timesheet.timesheet.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }

    let error = result.err().expect("Should be an error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
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
