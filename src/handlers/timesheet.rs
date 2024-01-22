use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::ApprovalStatus,
    repositories::{
        employment::employment_repo::EmploymentRepository,
        timesheet::{
            models::{
                TimesheetCreateData, TimesheetReadAllData, TimesheetUpdateData,
                TimesheetWithWorkdays, WorkdayUpdateData,
            },
            timesheet_repo::TimesheetRepository,
        },
    },
    templates::timesheet::{
        TimesheetCalculateTemplate, TimesheetReviewTemplate, TimesheetTemplate,
        TimesheetsReviewTemplate, TimesheetsTemplate, WorkdayEditTemplate, WorkdayTemplate,
    },
    utils::wage_calc::{calculate_wage::calculate_timesheet_wage, models::TimesheetWageDetailed},
};

#[derive(Deserialize)]
pub struct TimesheetCalcQuery {
    pub pink_paper_signed: bool,
}

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
            user_id,
            company_id,
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

async fn get_calculated_wage(
    target_sheet: &TimesheetWithWorkdays,
    pink_paper_signed: bool,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> Result<TimesheetWageDetailed, String> {
    let relevant_sheets_res = timesheet_repo
        .read_all_with_date_from_to_per_employment_extended_db(
            target_sheet.timesheet.user_id,
            target_sheet.timesheet.company_id,
            target_sheet.timesheet.start_date,
            target_sheet.timesheet.end_date,
        )
        .await;
    if relevant_sheets_res.is_err() {
        return Err("Could not find the timesheets required.".to_string());
    }
    let relevant_sheets = relevant_sheets_res.expect("Should be okay");
    calculate_timesheet_wage(
        pink_paper_signed,
        &relevant_sheets,
        target_sheet.timesheet.id,
    )
}

// The usage of a query for a simple parameter is a bit hacky. I admit.
#[get("/timesheet/{timesheet_id}/expected-wage")]
pub async fn get_expected_wage_calculation(
    timesheet_id: web::Path<String>,
    query: web::Query<TimesheetCalcQuery>,
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
    let sheet = sheet_res.expect("Should be some.");

    let wage = get_calculated_wage(&sheet, query.pink_paper_signed, timesheet_repo).await;
    if wage.is_err() {
        return HttpResponse::BadRequest().body(wage.expect_err("Should be an error."));
    }
    let template = TimesheetCalculateTemplate {
        wage: wage.expect("Should be valid."),
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

// A little hacky, I admit. But time is a precious commodity today.
#[get("/timesheet/{timesheet_id}/hours")]
pub async fn get_sheet_hours(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format".to_string());
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let sheet_res = timesheet_repo._read_one(parsed_id).await;
    if let Err(error) = sheet_res {
        return handle_database_error(error);
    }

    let sheet = sheet_res.expect("Should be some.");

    return HttpResponse::Ok().body(format!("{}", sheet.timesheet.total_hours));
}

#[get("/timesheet/{timesheet_id}/submit-page")]
pub async fn open_sheet_submit_page(
    timesheet_id: web::Path<String>,
    query: web::Query<TimesheetCalcQuery>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let sheet_res = timesheet_repo._read_one(parsed_id).await;
    if let Err(error) = sheet_res {
        return handle_database_error(error);
    }

    let sheet = sheet_res.expect("Should be some.");

    let wage = get_calculated_wage(&sheet, query.pink_paper_signed, timesheet_repo).await;
    if let Err(error) = wage {
        return HttpResponse::BadRequest().body(error);
    }
    let template = TimesheetCalculateTemplate {
        wage: wage.expect("Should be valid."),
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
    data.is_editable.is_none()
        && data.status.is_none()
        && (data.manager_note.is_none()
            || (data.manager_note.is_some() && data.manager_note.unwrap().is_empty()))
}

#[patch("/timesheet/{timesheet_id}")]
pub async fn update_timesheet(
    timesheet_id: web::Path<String>,
    timesheet_data: web::Json<TimesheetUpdateData>,
    employment_repo: web::Data<EmploymentRepository>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    if is_data_empty(timesheet_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let status_change = timesheet_data.status.clone();
    let parsed_id = id_parse.expect("Should be valid.");
    let result = timesheet_repo
        .update(parsed_id, timesheet_data.into_inner())
        .await;

    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }

    let timesheet = result.expect("Should be valid");

    if status_change.is_some() && status_change.expect("Should be some.") == ApprovalStatus::Pending
    {
        let template: TimesheetTemplate = timesheet.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    get_review_sheets(
        timesheet.timesheet.user_id,
        timesheet.timesheet.company_id,
        employment_repo,
        timesheet_repo,
    )
    .await
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

async fn get_review_sheets(
    user_id: Uuid,
    company_id: Uuid,
    employment_repo: web::Data<EmploymentRepository>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let employment_result = employment_repo.read_one(user_id, company_id).await;
    if employment_result.is_err() {
        return handle_database_error(employment_result.expect_err("Should be an error."));
    }
    let employment = employment_result.expect("Should be okay");
    if employment.manager.is_none() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let result = timesheet_repo
        .read_all_timesheets_per_employment(
            user_id,
            company_id,
            TimesheetReadAllData {
                limit: None,
                offset: None,
            },
        )
        .await;
    if let Ok(timesheets) = result {
        let template = TimesheetsReviewTemplate {
            timesheets: timesheets.into_iter().map(|sheet| sheet.into()).collect(),
            manager_id: employment
                .manager
                .expect("Should be okay because of previous check")
                .id,
            company_id,
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be an error."))
}

#[get("/user/{user_id}/employment/{company_id}/sheets-review")]
pub async fn get_timesheets_for_review(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.expect("Should be okay.");
    get_review_sheets(user_id, company_id, employment_repo, timesheet_repo).await
}

#[get("/timesheet/{timesheet_id}/review-mode")]
pub async fn open_timesheet_for_review(
    timesheet_id: web::Path<String>,
    timesheet_repo: web::Data<TimesheetRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(timesheet_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = timesheet_repo._read_one(parsed_id).await;
    if let Ok(sheet) = result {
        let template = TimesheetReviewTemplate {
            sheet: sheet.into(),
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().body(body.expect("Should be okay"));
    }
    handle_database_error(result.expect_err("Should be an error."))
}
