use std::str::FromStr;

use crate::{
    handlers::common::extract_user_company_ids,
    repositories::employment::models::{EmploymentData, NewEmployment},
    templates::{
        employment::{EmploymentLiteTemplate, EmploymentTemplate},
        user::UserLiteTemplate,
    },
};
use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    repositories::employment::{employment_repo::EmploymentRepository, models::EmploymentFilter},
    templates::{company::CompanyLiteTemplate, employment::EmploymentsTemplate},
};

#[get("/user/{user_id}/employment")]
pub async fn get_employments_per_user(
    user_id: web::Path<String>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0)
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
        let employment_vec = employments
            .into_iter()
            .map(|employment| {
                let company = CompanyLiteTemplate {
                    id: employment.company.id,
                    name: employment.company.name,
                    avatar_url: employment.company.avatar_url,
                };

                EmploymentLiteTemplate {
                    user_id: employment.user_id,
                    company,
                    employment_type: employment.employment_type,
                    start_date: employment.start_date,
                    end_date: employment.end_date,
                }
            })
            .collect();

        let template = EmploymentsTemplate {
            employments: employment_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."));
    }

    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

async fn get_full_employment(
    user_id: Uuid,
    company_id: Uuid,
    employment_repo: web::Data<EmploymentRepository>,
    is_created: bool,
) -> HttpResponse {
    let result = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = result {
        let company = CompanyLiteTemplate {
            id: employment.company.id,
            name: employment.company.name,
            avatar_url: employment.company.avatar_url,
        };

        let manager = match employment.manager {
            Some(user) => Some(UserLiteTemplate {
                id: user.id,
                name: user.name,
                status: user.status,
                age: chrono::offset::Local::now()
                    .naive_local()
                    .date()
                    .years_since(user.birth)
                    .expect("Should be valid"),
                gender: user.gender,
                avatar_url: user.avatar_url,
            }),
            _ => None,
        };

        let template = EmploymentTemplate {
            user_id: employment.user_id,
            company,
            manager,
            employment_type: employment.employment_type,
            hourly_wage: employment.hourly_wage,
            level: employment.level,
            description: employment
                .description
                .unwrap_or("No description.".to_string()),
            start_date: employment.start_date,
            end_date: employment.end_date,
            created_at: employment.created_at,
            edited_at: employment.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return if is_created {
            HttpResponse::Created().content_type("text/html").body(body.expect("Should be valid now."))
        } else {
            HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."))
        };
    }

    let error = result.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation() || err.is_foreign_key_violation() {
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

#[get("/user/{user_id}/employment/{company_id}")]
pub async fn get_employment(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    // let ids = path.into_inner();
    // let user_id_parse = Uuid::from_str(ids.0.as_str());
    // let company_id_parse = Uuid::from_str(ids.1.as_str());
    // if user_id_parse.is_err() || company_id_parse.is_err() {
    //     return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    // }

    let parsed_ids = extract_user_company_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    // let parsed_company_id = company_id_parse.expect("Should be valid.");
    // let parsed_user_id = user_id_parse.expect("Should be valid.");
    get_full_employment(user_id, company_id, employment_repo, false).await
}

#[get("/user/{user_id}/employment/{company_id}/subordinates")]
pub async fn get_subordinates(
    path: web::Path<(String, String)>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_user_company_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo
        .read_subordinates(user_id, company_id, query_params)
        .await;

    if let Ok(employments) = result {
        if employments.len() == 0 {
            return HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND));
        }

        let employment_vec = employments
            .into_iter()
            .map(|employment| {
                let company = CompanyLiteTemplate {
                    id: employment.company.id,
                    name: employment.company.name,
                    avatar_url: employment.company.avatar_url,
                };

                EmploymentLiteTemplate {
                    user_id: employment.user_id,
                    company,
                    employment_type: employment.employment_type,
                    start_date: employment.start_date,
                    end_date: employment.end_date,
                }
            })
            .collect();

        let template = EmploymentsTemplate {
            employments: employment_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."));
    }

    let error = result.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation() || err.is_foreign_key_violation() {
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

#[post("/employment")]
pub async fn create_employment(
    new_employment: web::Form<NewEmployment>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let user_id = new_employment.user_id;
    let company_id = new_employment.company_id;

    let result = employment_repo.create(new_employment.into_inner()).await;

    if let Err(error) = result {
        return match error {
            sqlx::Error::Database(err) => {
                if err.is_unique_violation()
                    || err.is_check_violation()
                    || err.is_foreign_key_violation()
                {
                    HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
                } else {
                    HttpResponse::InternalServerError()
                        .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
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
    employment_data: web::Form<EmploymentData>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    if is_data_empty(employment_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_user_company_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo
        .update(user_id, company_id, employment_data.into_inner())
        .await;

    if let Err(error) = result {
        return match error {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
            }
            sqlx::Error::Database(err) => {
                if err.is_check_violation() || err.is_foreign_key_violation() {
                    HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
                } else {
                    HttpResponse::InternalServerError()
                        .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
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
    let parsed_ids = extract_user_company_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (user_id, company_id) = parsed_ids.unwrap();

    let result = employment_repo.delete(user_id, company_id).await;
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
