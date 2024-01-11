use std::str::FromStr;

use crate::{
    errors::handle_database_error,
    handlers::common::extract_path_tuple_ids,
    repositories::employment::models::{EmploymentData, NewEmployment},
    templates::employment::{EmploymentLiteTemplate, EmploymentTemplate, EmploymentLite},
};
use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    repositories::employment::{employment_repo::EmploymentRepository, models::EmploymentFilter},
    templates::employment::EmploymentsTemplate,
};

#[get("/user/{user_id}/employment")]
pub async fn get_employments_per_user(
    user_id: web::Path<String>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();
    println!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEPY");
    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = employment_repo
        .read_all_for_user(parsed_id, query_params)
        .await;

    if let Ok(employments) = result {
        let employment_vec: Vec<EmploymentLite> = employments
            .into_iter()
            .map(|employment| employment.into())
            .collect();
        println!("VEEEEEEEEEEEEEEEEEEEEEEEC");
        let template = EmploymentsTemplate {
            employments: employment_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        println!("RENDEEEEEEEEEEEEEER");

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }
    println!("DB ERRRRRRRRRRRRRRRRRRRRRRRRRRRRRR");
    handle_database_error(result.expect_err("Should be error."))
}

async fn get_full_employment(
    user_id: Uuid,
    company_id: Uuid,
    employment_repo: web::Data<EmploymentRepository>,
    is_created: bool,
) -> HttpResponse {
    let result = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = result {
        let template: EmploymentTemplate = employment.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return if is_created {
            HttpResponse::Created()
                .content_type("text/html")
                .body(body.expect("Should be valid now."))
        } else {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(body.expect("Should be valid now."))
        };
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/user/{user_id}/employment/{company_id}")]
pub async fn get_employment(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    get_full_employment(user_id, company_id, employment_repo, false).await
}

#[get("/user/{user_id}/employment/{company_id}/subordinates")]
pub async fn get_subordinates(
    path: web::Path<(String, String)>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo
        .read_subordinates(user_id, company_id, query_params)
        .await;

    if let Ok(employments) = result {
        let employment_vec: Vec<EmploymentLite> = employments
            .into_iter()
            .map(|employment| employment.into())
            .collect();

        let template = EmploymentsTemplate {
            employments: employment_vec,
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

#[post("/employment")]
pub async fn create_employment(
    new_employment: web::Json<NewEmployment>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let user_id = new_employment.user_id;
    let company_id = new_employment.company_id;

    let result = employment_repo.create(new_employment.into_inner()).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    // This isn't very pleasant, but it is what it is. Maybe fix later.
    // This is done because the repo doesn't return the necessary data from the function.
    get_full_employment(user_id, company_id, employment_repo, true).await
}

fn is_data_empty(data: EmploymentData) -> bool {
    data.manager_id.is_none()
        && data.hourly_wage.is_none()
        && data.start_date.is_none()
        && data.end_date.is_none()
        && data.description.is_none()
        && data.employment_type.is_none()
        && data.level.is_none()
}

#[patch("/user/{user_id}/employment/{company_id}")]
pub async fn update_employment(
    path: web::Path<(String, String)>,
    employment_data: web::Json<EmploymentData>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    if is_data_empty(employment_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo
        .update(user_id, company_id, employment_data.into_inner())
        .await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    // This isn't very pleasant, but it is what it is. Maybe fix later.
    // This is done because the repo doesn't return the necessary data from the function.
    get_full_employment(user_id, company_id, employment_repo, false).await
}

#[delete("/user/{user_id}/employment/{company_id}")]
pub async fn delete_employment(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();

    let result = employment_repo.delete(user_id, company_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}
