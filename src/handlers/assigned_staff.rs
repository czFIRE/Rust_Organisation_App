use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    handlers::common::extract_path_tuple_ids,
    repositories::assigned_staff::{
        assigned_staff_repo::AssignedStaffRepository,
        models::{AssignedStaffData, AssignedStaffFilter, NewAssignedStaff},
    },
    templates::staff::{AllAssignedStaffTemplate, AssignedStaffTemplate},
};

#[derive(Deserialize)]
pub struct NewAssignedStaffData {
    staff_id: Uuid,
}

#[get("/task/{task_id}/staff")]
pub async fn get_all_assigned_staff(
    task_id: web::Path<String>,
    query: web::Query<AssignedStaffFilter>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() <= 0)
        || (query.offset.is_some() && query.offset.clone().unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = assigned_repo
        .read_all_per_task(parsed_id, query.into_inner())
        .await;

    if let Ok(assigned) = result {
        let assigned_vec: Vec<AssignedStaffTemplate> = assigned
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

    let error = result.err().expect("Should be an error");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
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

    let error = result.err().expect("Should be an error");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[post("/task/{task_id}/staff")]
pub async fn create_assigned_staff(
    task_id: web::Path<String>,
    new_task_staff: web::Form<NewAssignedStaffData>,
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

    let error = result.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[patch("/task/{task_id}/staff/{staff_id}")]
pub async fn update_assigned_staff(
    path: web::Path<(String, String)>,
    task_staff_data: web::Form<AssignedStaffData>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (task_id, staff_id) = parsed_ids.unwrap();
    let result = assigned_repo
        .update(task_id, staff_id, task_staff_data.into_inner())
        .await;
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

    let error = result.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
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
        return match error {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
    }

    HttpResponse::NoContent().finish()
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
        return match error {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
    }

    HttpResponse::NoContent().finish()
}
