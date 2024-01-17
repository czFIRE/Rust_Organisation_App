use std::{collections::HashMap, str::FromStr};

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    repositories::timesheet::{
        models::{
            TimesheetCreateData, TimesheetReadAllData, TimesheetUpdateData, WorkdayUpdateData,
        },
        timesheet_repo::TimesheetRepository,
    },
    templates::timesheet::{
        DetailedWage, TimesheetCalculateTemplate, TimesheetTemplate, TimesheetWageDetailed,
        TimesheetsTemplate, WorkdayEditTemplate, WorkdayTemplate,
    },
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
        .read_all_timesheets_per_employment(user_id, company_id, query_params)
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

// ToDo: Remove???
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

#[get("/timesheet/{timesheet_id}/expected-wage")]
pub async fn get_expected_wage_calculation(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let sheet_res = timesheet_repo._read_one(parsed_id).await;
    if sheet_res.is_err() {
        return handle_database_error(sheet_res.expect_err("Should be an error."));
    }

    //ToDo: Here we call the calculation function

    //ToDo: Get rid of this mock data.
    let detailed_wage = DetailedWage {
        tax_base: 3000.0,
        net_wage: 4235.52,
        employer_social_insurance: 500.0,
        employee_health_insurance: 250.0,
        employer_health_insurance: 750.0,
        employee_social_insurance: 200.0,
    };

    let mut wages_per_month: HashMap<String, DetailedWage> = HashMap::new();
    wages_per_month.insert("February".to_string(), detailed_wage.clone());
    wages_per_month.insert("Maruary".to_string(), detailed_wage.clone());

    let wage = TimesheetWageDetailed {
        total_wage: detailed_wage,
        wage_currency: "Czk".to_string(),
        month_to_detailed_wage: wages_per_month,
        error_option: None,
    };

    let template = TimesheetCalculateTemplate {
        wage,
        timesheet_id: parsed_id,
        in_submit_mode: false,
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid."))
}

#[get("/timesheet/{timesheet_id}/submit-page")]
pub async fn open_sheet_submit_page(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let sheet_res = timesheet_repo._read_one(parsed_id).await;
    if sheet_res.is_err() {
        return handle_database_error(sheet_res.expect_err("Should be an error."));
    }

    //ToDo: Here we call the calculation function

    //ToDo: Get rid of this mock data.
    let detailed_wage = DetailedWage {
        tax_base: 3000.0,
        net_wage: 4235.52,
        employer_social_insurance: 500.0,
        employee_health_insurance: 250.0,
        employer_health_insurance: 750.0,
        employee_social_insurance: 200.0,
    };

    let mut wages_per_month: HashMap<String, DetailedWage> = HashMap::new();
    wages_per_month.insert("February".to_string(), detailed_wage.clone());
    wages_per_month.insert("Maruary".to_string(), detailed_wage.clone());

    let wage = TimesheetWageDetailed {
        total_wage: detailed_wage,
        wage_currency: "Czk".to_string(),
        month_to_detailed_wage: wages_per_month,
        error_option: None,
    };

    let template = TimesheetCalculateTemplate {
        wage,
        timesheet_id: parsed_id,
        in_submit_mode: true,
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid."))
}

fn is_data_empty(data: TimesheetUpdateData) -> bool {
    data.start_date.is_none()
        && data.end_date.is_none()
        && (data.total_hours.is_none()
            || (data.total_hours.is_some() && data.total_hours.unwrap() < 0.0))
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
* Reset every workday for a corresponding timesheet, as well as worked_hours and comments in the timesheet record.
*/
#[delete("/timesheet/{timesheet_id}/days")]
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

#[patch("/timesheet/{timesheet_id}/day/{date}")]
pub async fn update_work_day(
    path: web::Path<(String, String)>,
    data: web::Json<WorkdayUpdateData>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.0.as_str());
    let date_parse = NaiveDate::parse_from_str(path.1.as_str(), "%Y-%m-%d");

    if id_parse.is_err() || date_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let timesheet_id = id_parse.expect("Should be valid");
    let date = date_parse.expect("Should be valid");

    let result = timesheet_repo
        .update_workday(timesheet_id, date, data.into_inner())
        .await;

    if let Ok(workday) = result {
        let template: WorkdayTemplate = workday.into();

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

#[get("/timesheet/{timesheet_id}/day/{date}/edit-mode")]
pub async fn toggle_work_day_edit_mode(
    path: web::Path<(String, String)>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.0.as_str());
    let date_parse = NaiveDate::parse_from_str(path.1.as_str(), "%Y-%m-%d");

    if id_parse.is_err() || date_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let timesheet_id = id_parse.expect("Should be valid");
    let date = date_parse.expect("Should be valid");

    let result = timesheet_repo.read_one_workday(timesheet_id, date).await;

    if let Ok(workday) = result {
        let template: WorkdayEditTemplate = workday.into();

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

#[get("/timesheet/{timesheet_id}/day/{date}")]
pub async fn get_work_day(
    path: web::Path<(String, String)>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.0.as_str());
    let date_parse = NaiveDate::parse_from_str(path.1.as_str(), "%Y-%m-%d");

    if id_parse.is_err() || date_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let timesheet_id = id_parse.expect("Should be valid");
    let date = date_parse.expect("Should be valid");

    let result = timesheet_repo.read_one_workday(timesheet_id, date).await;

    if let Ok(workday) = result {
        let template: WorkdayTemplate = workday.into();

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
