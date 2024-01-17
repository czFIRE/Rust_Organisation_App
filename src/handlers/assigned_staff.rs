use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::EventRole,
    repositories::{
        assigned_staff::{
            assigned_staff_repo::AssignedStaffRepository,
            models::{AssignedStaffData, AssignedStaffFilter, NewAssignedStaff},
        },
        event_staff::event_staff_repo::StaffRepository,
    },
    templates::staff::{
        AllAssignedStaffTemplate, AssignedStaff, AssignedStaffManagementTemplate,
        AssignedStaffTemplate,
    },
};

#[derive(Deserialize)]
pub struct NewAssignedStaffData {
    staff_id: Uuid,
}

async fn get_staff_per_task(
    task_id: Uuid,
    query: AssignedStaffFilter,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let result = assigned_repo.read_all_per_task(task_id, query).await;

    if let Ok(assigned) = result {
        let assigned_vec: Vec<AssignedStaff> = assigned
            .into_iter()
            .map(|assigned_staff| assigned_staff.into())
            .collect();
        let template = AllAssignedStaffTemplate {
            staff: assigned_vec,
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

#[get("/task/{task_id}/staff")]
pub async fn get_all_assigned_staff(
    task_id: web::Path<String>,
    query: web::Query<AssignedStaffFilter>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.unwrap() <= 0)
        || (query.offset.is_some() && query.offset.unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    get_staff_per_task(parsed_id, query.into_inner(), assigned_repo).await
}

#[get("/task/{task_id}/staff/{staff_id}")]
pub async fn get_assigned_staff(
    path: web::Path<(String, String)>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (task_id, staff_id) = parsed_ids.unwrap();
    let result = assigned_repo.read_one(task_id, staff_id).await;

    if let Ok(assigned_staff) = result {
        let template: AssignedStaffTemplate = assigned_staff.into();
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

#[post("/task/{task_id}/staff")]
pub async fn create_assigned_staff(
    task_id: web::Path<String>,
    new_task_staff: web::Json<NewAssignedStaffData>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let task_staff_data = NewAssignedStaff {
        task_id: parsed_id,
        staff_id: new_task_staff.staff_id,
    };

    let result = assigned_repo.create(task_staff_data).await;
    if let Ok(assigned_staff) = result {
        let template: AssignedStaffTemplate = assigned_staff.into();
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

#[patch("/task/{task_id}/staff/{staff_id}")]
pub async fn update_assigned_staff(
    path: web::Path<(String, String)>,
    task_staff_data: web::Json<AssignedStaffData>,
    assigned_repo: web::Data<AssignedStaffRepository>,
    staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (task_id, staff_id) = parsed_ids.unwrap();

    let decider = staff_repo.read_one(task_staff_data.decided_by).await;
    if decider.is_err() {
        // Might specify this error further. But the decider needs to exist in the request, so it's a bad request.
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let decider_unwrapped = decider.expect("Should be valid here.");

    let staff = staff_repo.read_one(staff_id).await;
    if staff.is_err() {
        return HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND));
    }
    let staff_unwrapped = staff.expect("Should be valid here.");

    if decider_unwrapped.event_id != staff_unwrapped.event_id
        || decider_unwrapped.role != EventRole::Organizer
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = assigned_repo
        .update(task_id, staff_id, task_staff_data.into_inner())
        .await;

    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }

    let query = AssignedStaffFilter {
        offset: None,
        limit: None,
    };
    get_staff_per_task(task_id, query, assigned_repo).await
}

#[delete("/task/{task_id}/staff")]
pub async fn delete_all_rejected_assigned_staff(
    task_id: web::Path<String>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = assigned_repo.delete_rejected(parsed_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    let query = AssignedStaffFilter {
        offset: None,
        limit: None,
    };
    get_staff_per_task(parsed_id, query, assigned_repo).await
}

#[delete("task/{task_id}/staff/{staff_id}")]
pub async fn delete_assigned_staff(
    path: web::Path<(String, String)>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (task_id, staff_id) = parsed_ids.unwrap();
    let result = assigned_repo.delete(task_id, staff_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    let query = AssignedStaffFilter {
        offset: None,
        limit: None,
    };
    get_staff_per_task(task_id, query, assigned_repo).await
}

#[get("/task/{task_id}/staff/{staff_id}/management")]
pub async fn initialize_assigned_staff_management_panel(
    path: web::Path<(String, String)>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let (task_id, staff_id) = parsed_ids.expect("Should be okay");

    let result = assigned_repo.read_one(task_id, staff_id).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error here."));
    }

    let requester = result.expect("Should be valid here.");
    if requester.staff.role != EventRole::Organizer {
        return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
    }

    let template = AssignedStaffManagementTemplate {
        requester: requester.into(),
        task_id,
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid here."))
}
