use std::str::FromStr;

use crate::{
    errors::handle_database_error,
    handlers::common::{extract_path_triple_ids, extract_path_tuple_ids},
    models::{EmployeeLevel, EmploymentContract},
    repositories::employment::models::{EmploymentData, NewEmployment},
    templates::employment::{
        EmploymentEditTemplate, EmploymentLite, EmploymentTemplate, SubordinatesTemplate,
    },
};
use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    repositories::employment::{employment_repo::EmploymentRepository, models::EmploymentFilter},
    templates::employment::EmploymentsTemplate,
};

#[derive(Clone, Debug, Deserialize)]
pub struct EmploymentUpdateData {
    pub editor_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub employment_type: Option<EmploymentContract>,
    pub level: Option<EmployeeLevel>,
}

#[get("/user/{user_id}/employment")]
pub async fn get_employments_per_user(
    user_id: web::Path<String>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();
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

    if let Ok(subordinates) = result {
        let template = SubordinatesTemplate {
            user_id,
            subordinates,
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

#[get("/user/{user_id}/employment/{company_id}/mode/{editor_id}")]
pub async fn toggle_employment_edit(
    path: web::Path<(String, String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_triple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id, editor_id) = parsed_ids.unwrap();
    let result = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = result {
        // Only the direct manager may edit an employee.
        if employment.manager.is_none()
            || employment.manager.is_some() && employment.manager.unwrap().id != editor_id
        {
            return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
        }
        let template: EmploymentEditTemplate = EmploymentEditTemplate {
            editor_id,
            user_id: employment.user_id,
            company_id: employment.company.id,
            employment_type: employment.employment_type,
            hourly_wage: employment.hourly_wage,
            level: employment.level,
            description: employment.description,
            start_date: employment.start_date,
            end_date: employment.end_date,
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

fn is_data_invalid(data: EmploymentUpdateData) -> bool {
    data.manager_id.is_none()
        && (data.hourly_wage.is_none() || data.hourly_wage.unwrap() <= 0.0) // This should likely check against minimum wage instead.
        && data.start_date.is_none()
        && data.end_date.is_none()
        // Not checking for emptiness of description as it should be okay to set the description to be empty.
        && data.description.is_none()
        && data.employment_type.is_none()
        && data.level.is_none()
        || (data.start_date.is_some()
            && data.end_date.is_some()
            && data.start_date.unwrap() > data.end_date.unwrap())
}

#[patch("/user/{user_id}/employment/{company_id}")]
pub async fn update_employment(
    path: web::Path<(String, String)>,
    employment_data: web::Json<EmploymentUpdateData>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    if is_data_invalid(employment_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();

    // We have to compare these dates against old dates.
    if employment_data.start_date.is_some() || employment_data.end_date.is_some() {
        let current_employment = employment_repo.read_one(user_id, company_id).await;

        if current_employment.is_err() {
            return handle_database_error(current_employment.expect_err("Should be error."));
        }

        let current = current_employment.expect("Should be valid now.");

        if employment_data.start_date.is_some()
            && employment_data.start_date.unwrap() > current.end_date
        {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }

        if employment_data.end_date.is_some()
            && employment_data.end_date.unwrap() < current.start_date
        {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
    }

    let data = EmploymentData {
        manager_id: employment_data.manager_id,
        hourly_wage: None,
        // hourly_wage: employment_data.hourly_wage.into(),
        start_date: employment_data.start_date,
        end_date: employment_data.end_date,
        description: employment_data.description.clone(),
        employment_type: employment_data.employment_type.clone(),
        level: employment_data.level.clone(),
    };

    let result = employment_repo.update(user_id, company_id, data).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    // This isn't very pleasant, but it is what it is. Maybe fix later.
    // Editor id because we don't want to render the employee's view.
    get_full_employment(
        employment_data.editor_id,
        company_id,
        employment_repo,
        false,
    )
    .await
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
