use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    repositories::timesheet::{
        models::{TimesheetCreateData, TimesheetReadAllData, TimesheetUpdateData},
        timesheet_repo::TimesheetRepository,
    },
    templates::timesheet::{TimesheetTemplate, TimesheetsTemplate},
};

#[get("/user/{user_id}/employment/{company_id}/sheet")]
pub async fn get_all_timesheets_for_employment(
    path: web::Path<(String, String)>,
    query: web::Query<TimesheetReadAllData>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let query_params = query.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() <= 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = timesheet_repo
        .read_all_per_employment(user_id, company_id, query_params)
        .await;

    if let Ok(timesheets) = result {
        let timesheet_vec = timesheets
            .into_iter()
            .map(|timesheet| timesheet.into())
            .collect();

        let template = TimesheetsTemplate {
            timesheets: timesheet_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

// Note: This is done automatically whenever event_staff is accepted to work on an event.
#[post("/timesheet")]
pub async fn create_timesheet(
    new_timesheet: web::Json<TimesheetCreateData>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    if new_timesheet.end_date < new_timesheet.start_date {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = timesheet_repo.create(new_timesheet.into_inner()).await;

    if let Ok(full_timesheet) = result {
        let template: TimesheetTemplate = full_timesheet.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
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
        let template: TimesheetTemplate = full_timesheet.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn is_data_empty(data: TimesheetUpdateData) -> bool {
    data.start_date.is_none()
        && data.end_date.is_none()
        && data.total_hours.is_none()
        && data.is_editable.is_none()
        && data.status.is_none()
        && (data.manager_note.is_none()
            || (data.manager_note.is_some() && data.manager_note.unwrap().is_empty()))
        && (data.workdays.is_none()
            || (data.workdays.is_some() && data.workdays.unwrap().is_empty()))
        || (data.start_date.is_some()
            && data.end_date.is_some()
            && data.start_date.unwrap() > data.end_date.unwrap())
}

#[patch("/timesheet/{timesheet_id}")]
pub async fn update_timesheet(
    timesheet_id: web::Path<String>,
    timesheet_data: web::Json<TimesheetUpdateData>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    if is_data_empty(timesheet_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = timesheet_repo
        .update(parsed_id, timesheet_data.into_inner())
        .await;

    if let Ok(full_timesheet) = result {
        // Now we need
        //
        // todo: Write a function converting TimesheetWithWorkdays
        //       to TimesheetWithWorkdaysExtended.
        // let TimesheetWithWorkdaysExtended = {
        //     .. full_timesheet

        //     hourly_wage: ,
        //     employment_type: ,
        //     wage_preset: ,
        // }

        let template: TimesheetTemplate = full_timesheet.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

/*
 * Reset every workday for a corresponding timesheet, as well as worked_hours
 * and comments in the timesheet record.
 */
#[delete("/timesheet/{timesheet_id}/workdays")]
pub async fn reset_timesheet_data(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = timesheet_repo.reset_timesheet(parsed_id).await;
    if let Ok(full_timesheet) = result {
        let template: TimesheetTemplate = full_timesheet.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}
