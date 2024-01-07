use std::str::FromStr;

use actix_web::{delete, get, patch, post, web, HttpResponse, http};
use askama::Template;
use crate::{repositories::event_staff::models::{StaffFilter, NewStaff, StaffData}, templates::staff::AllStaffTemplate};
use uuid::Uuid;

use crate::{repositories::event_staff::event_staff_repo::StaffRepository, templates::staff::StaffTemplate, errors::parse_error};

#[get("/event/{event_id}/staff")]
pub async fn get_all_event_staff(event_id: web::Path<String>, query: web::Query<StaffFilter>, event_staff_repo: web::Data<StaffRepository>) -> HttpResponse {
    let query_info = query.into_inner();
    if (query_info.limit.is_some() && query_info.limit.clone().unwrap() <= 0)
        || (query_info.offset.is_some() && query_info.offset.clone().unwrap() <= 0) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.read_all_for_event(parsed_id, query_info).await;
    if let Ok(all_staff) = result {
        let staff_vec = all_staff.into_iter().map(|staff| {
            staff.into()
        }).collect();
        
        let template = AllStaffTemplate {
            staff: staff_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
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

#[get("/event/staff/{staff_id}")]
pub async fn get_event_staff(staff_id: web::Path<String>, event_staff_repo: web::Data<StaffRepository>) -> HttpResponse {
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
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().body(body.expect("Should be valid now."));
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

#[post("/event/{event_id}/staff")]
pub async fn create_event_staff(
    event_id: web::Path<String>,
    new_event_staff: web::Form<NewStaff>,
    event_staff_repo: web::Data<StaffRepository>
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.create(parsed_id, new_event_staff.into_inner()).await;

    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().body(body.expect("Should be valid now."));
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

fn is_data_invalid(data: StaffData) -> bool {
    (data.role.is_none()
    && data.status.is_none()
    && data.decided_by.is_none())
    || (data.status.is_some() && data.decided_by.is_none())
}

#[patch("/event-staff/{staff_id}")]
pub async fn update_event_staff(
    staff_id: web::Path<String>,
    event_staff_data: web::Form<StaffData>,
    event_staff_repo: web::Data<StaffRepository>
) -> HttpResponse {
    if is_data_invalid(event_staff_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.update(parsed_id, event_staff_data.into_inner()).await;

    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().body(body.expect("Should be valid now."));
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

#[delete("/event/{event_id}/staff")]
pub async fn delete_all_rejected_event_staff(
    path: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.delete(parsed_id).await;
    
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

#[delete("/event-staff/{staff_id}")]
pub async fn delete_event_staff(
    staff_id: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.delete(parsed_id).await;
    
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
