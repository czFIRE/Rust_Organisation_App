use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::{EmployeeLevel, EventRole},
    repositories::{
        employment::employment_repo::EmploymentRepository,
        event::{
            event_repo::EventRepository,
            models::{EventData, EventFilter, NewEvent},
        },
        event_staff::event_staff_repo::StaffRepository,
    },
    templates::event::{
        EventCreateTemplate, EventEditTemplate, EventLite, EventTemplate, EventsTemplate,
    },
};

#[get("/event")]
pub async fn get_events(
    params: web::Query<EventFilter>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = event_repo.read_all(query_params).await;

    if let Ok(events) = result {
        let lite_events: Vec<EventLite> = events.into_iter().map(|event| event.into()).collect();

        let template = EventsTemplate {
            events: lite_events,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be okay now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/event/{event_id}")]
pub async fn get_event(
    event_id: web::Path<String>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = event_repo.read_one(parsed_id).await;

    if let Ok(event) = result {
        let template: EventTemplate = event.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[post("/event")]
pub async fn create_event(
    new_event: web::Json<NewEvent>,
    event_repo: web::Data<EventRepository>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    if new_event.start_date > new_event.end_date {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let employee_res = employment_repo
        .read_one(new_event.creator_id, new_event.company_id)
        .await;

    if employee_res.is_err() {
        return handle_database_error(employee_res.expect_err("Should be error."));
    }

    let employee = employee_res.expect("Should be OK");

    if employee.level != EmployeeLevel::CompanyAdministrator {
        return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
    }

    let result = event_repo.create(new_event.into_inner()).await;

    if let Ok(event) = result {
        let template: EventTemplate = event.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn is_update_data_empty(event_data: EventData) -> bool {
    event_data.name.is_none()
        && event_data.description.is_none()
        && event_data.website.is_none()
        && event_data.start_date.is_none()
        && event_data.end_date.is_none()
        && event_data.accepts_staff.is_none()
}

#[patch("/event/{event_id}")]
pub async fn update_event(
    event_id: web::Path<String>,
    event_data: web::Json<EventData>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    if is_update_data_empty(event_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_repo.update(parsed_id, event_data.into_inner()).await;

    if let Ok(event) = result {
        let template: EventTemplate = event.into();

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

#[patch("/event/{event_id}/acceptance")]
pub async fn switch_event_accepts_staff(
    event_id: web::Path<String>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_repo.switch_accepts_staff(parsed_id).await;
    if result.is_ok() {
        return HttpResponse::NoContent().finish();
    }
    handle_database_error(result.expect_err("Should be error."))
}

#[delete("/event/{event_id}")]
pub async fn delete_event(
    event_id: web::Path<String>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_repo.delete(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}

#[get("/event/{event_id}/edit-mode/{staff_id}")]
pub async fn toggle_event_edit_mode(
    path: web::Path<(String, String)>,
    event_repo: web::Data<EventRepository>,
    staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, staff_id) = parsed_ids.unwrap();
    let staff_res = staff_repo.read_one(staff_id).await;
    if staff_res.is_err() {
        return handle_database_error(staff_res.expect_err("Should be an error."));
    }
    let staff = staff_res.expect("Should be valid.");
    // Check if the staffer is an organizer for this event.
    if staff.role != EventRole::Organizer || staff.event_id != event_id {
        return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
    }
    let result = event_repo.read_one(event_id).await;
    if let Ok(event) = result {
        let template = EventEditTemplate {
            event: event.into(),
            editor: staff.into(),
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

#[get("/user/{user_id}/employment/{company_id}/event")]
pub async fn toggle_event_creation_mode(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let employment_res = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = employment_res {
        if employment.level != EmployeeLevel::CompanyAdministrator {
            return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
        }

        let template = EventCreateTemplate {
            creator_id: employment.user_id,
            company_id: employment.company.id,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }
    handle_database_error(employment_res.expect_err("Should be an error."))
}

//TODO: Once file store/load is done.
#[get("/event/{event_id}/avatar")]
pub async fn get_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[put("/event/{event_id}/avatar")]
pub async fn upload_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/avatar")]
pub async fn remove_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}
