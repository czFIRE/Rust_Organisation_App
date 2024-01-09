use std::str::FromStr;

use crate::repositories::timesheet::models::TimesheetCreateData;
use crate::{
    common::DbResult,
    errors::handle_database_error,
    handlers::common::extract_path_tuple_ids,
    models::{AcceptanceStatus, EventRole},
    repositories::{
        event::event_repo::EventRepository,
        event_staff::models::{NewStaff, StaffData, StaffFilter},
        timesheet::timesheet_repo::TimesheetRepository,
    },
    templates::staff::AllStaffTemplate,
};
use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::parse_error, repositories::event_staff::event_staff_repo::StaffRepository,
    templates::staff::StaffTemplate,
};

#[get("/event/{event_id}/staff")]
pub async fn get_all_event_staff(
    event_id: web::Path<String>,
    query: web::Query<StaffFilter>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let query_info = query.into_inner();
    if (query_info.limit.is_some() && query_info.limit.clone().unwrap() <= 0)
        || (query_info.offset.is_some() && query_info.offset.clone().unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo
        .read_all_for_event(parsed_id, query_info)
        .await;
    if let Ok(all_staff) = result {
        let staff_vec = all_staff.into_iter().map(|staff| staff.into()).collect();

        let template = AllStaffTemplate { staff: staff_vec };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[get("/event/staff/{staff_id}")]
pub async fn get_event_staff(
    staff_id: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.read_one(parsed_id).await;
    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[post("/event/{event_id}/staff")]
pub async fn create_event_staff(
    event_id: web::Path<String>,
    new_event_staff: web::Json<NewStaff>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo
        .create(parsed_id, new_event_staff.into_inner())
        .await;

    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

fn is_data_invalid(data: StaffData) -> bool {
    (data.role.is_none() && data.status.is_none() && data.decided_by.is_none())
        || (data.status.is_some()
            && data.status.clone().unwrap() != AcceptanceStatus::Pending
            && data.decided_by.is_none())
        || (data.status.is_none() && data.decided_by.is_some())
}

async fn create_timesheet_for_user(
    user_id: Uuid,
    company_id: Uuid,
    event_id: Uuid,
    timesheet_repo: web::Data<TimesheetRepository>,
    event_repo: web::Data<EventRepository>,
) -> DbResult<()> {
    let event = event_repo.read_one(event_id).await?;

    let timesheet_data = TimesheetCreateData {
        start_date: event.start_date,
        end_date: event.end_date,
        user_id,
        company_id,
        event_id,
    };

    timesheet_repo.create(timesheet_data).await?;
    Ok(())
}

#[patch("/event/{event_id}/staff/{staff_id}")]
pub async fn update_event_staff(
    path: web::Path<(String, String)>,
    event_staff_data: web::Json<StaffData>,
    event_staff_repo: web::Data<StaffRepository>,
    timesheet_repo: web::Data<TimesheetRepository>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    if is_data_invalid(event_staff_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, staff_id) = parsed_ids.unwrap();

    // Extract the old and new status of the event staff to check if status really changes.
    let current_staff = event_staff_repo.read_one(staff_id).await;
    if current_staff.is_err() {
        return HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND));
    }
    let old_staff = current_staff.expect("Should be valid now.");
    let status_change = event_staff_data.status.clone();

    // Make sure the decider is a valid entity in the system.
    if event_staff_data.decided_by.is_some() {
        let decider_id = event_staff_data.decided_by.clone().unwrap();
        let decider = event_staff_repo.read_one(decider_id).await;
        if decider.is_err() {
            // Might specify this error further. But the decider needs to exist in the request, so it's a bad request.
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        let decider_unwrapped = decider.expect("Should be valid here.");
        // Decider is not from this event. Whoops.
        if decider_unwrapped.event_id != event_id || decider_unwrapped.role != EventRole::Organizer
        {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
    }

    let result = event_staff_repo
        .update(staff_id, event_staff_data.into_inner())
        .await;

    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        // If a status change occured, and everything else went well, then we
        // need to create the timesheet.
        if status_change.is_some()
            && status_change.unwrap() == AcceptanceStatus::Accepted
            && old_staff.status != AcceptanceStatus::Accepted
        {
            let timesheet_res = create_timesheet_for_user(
                old_staff.user.id,
                old_staff.company.id,
                event_id,
                timesheet_repo,
                event_repo,
            )
            .await;
            if timesheet_res.is_err() {
                return handle_database_error(timesheet_res.err().expect("Should be err."));
            }
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[delete("/event/{event_id}/staff")]
pub async fn delete_all_rejected_event_staff(
    path: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.delete_rejected(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}

#[delete("/event/staff/{staff_id}")]
pub async fn delete_event_staff(
    staff_id: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.delete(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}
