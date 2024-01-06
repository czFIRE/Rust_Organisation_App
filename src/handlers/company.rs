use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    handlers::common::QueryParams,
    repositories::company::{
        company_repo::CompanyRepository,
        models::{AddressData, AddressUpdateData, CompanyData, CompanyFilter, NewCompany},
    },
    templates::{
        self,
        company::{CompaniesTemplate, CompanyLiteTemplate, CompanyTemplate},
    },
};

#[derive(Deserialize)]
pub struct NewCompanyData {
    name: String,
    description: Option<String>,
    website: Option<String>,
    crn: String,
    vatin: String,
    country: String,
    region: String,
    city: String,
    street: String,
    number: String,
    postal_code: String,
    phone: String,
    email: String,
}

#[derive(Deserialize, Clone)]
pub struct CompanyUpdateData {
    name: Option<String>,
    description: Option<String>,
    website: Option<String>,
    crn: Option<String>,
    vatin: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    street: Option<String>,
    number: Option<String>,
    postal_code: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

#[get("/company")]
pub async fn get_all_companies(
    params: web::Query<QueryParams>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let filter = CompanyFilter {
        limit: query_params.limit,
        offset: query_params.offset,
    };

    let result = company_repo.read_all(filter).await;

    if let Ok(companies) = result {
        let lite_companies = companies
            .into_iter()
            .map(|company| CompanyLiteTemplate {
                id: company.id,
                name: company.name,
                avatar_url: company.avatar_url,
            })
            .collect();

        let template = CompaniesTemplate {
            companies: lite_companies,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be okay now."));
    }

    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

#[get("/company/{company_id}")]
pub async fn get_company(
    company_id: web::Path<String>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = company_repo.read_one_extended(parsed_id).await;

    if let Ok(company) = result {
        let template = CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number,
            created_at: company.created_at,
            edited_at: company.edited_at,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
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

#[post("/company")]
pub async fn create_company(
    new_company: web::Form<NewCompanyData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let company_data = NewCompany {
        name: new_company.name.clone(),
        description: new_company.description.clone(),
        phone: new_company.phone.clone(),
        email: new_company.email.clone(),
        website: new_company.website.clone(),
        crn: new_company.crn.clone(),
        vatin: new_company.vatin.clone(),
    };

    let address = AddressData {
        country: new_company.country.clone(),
        region: new_company.region.clone(),
        city: new_company.city.clone(),
        postal_code: new_company.postal_code.clone(),
        street: new_company.street.clone(),
        street_number: new_company.number.clone(),
    };

    let result = company_repo.create(company_data, address).await;

    if let Ok(company) = result {
        let template = CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number,
            created_at: company.created_at,
            edited_at: company.edited_at,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
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

fn is_update_data_empty(company_data: CompanyUpdateData) -> bool {
    company_data.name.is_none()
        && company_data.description.is_none()
        && company_data.website.is_none()
        && company_data.crn.is_none()
        && company_data.vatin.is_none()
        && company_data.country.is_none()
        && company_data.region.is_none()
        && company_data.city.is_none()
        && company_data.street.is_none()
        && company_data.number.is_none()
        && company_data.postal_code.is_none()
        && company_data.phone.is_none()
        && company_data.email.is_none()
}

#[patch("/company/{company_id}")]
pub async fn update_company(
    company_id: web::Path<String>,
    company_data: web::Form<CompanyUpdateData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let data = company_data.into_inner();

    if is_update_data_empty(data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let company_update_data = CompanyData {
        name: data.name.clone(),
        description: data.description.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        avatar_url: None, //ToDo: Include avatar in company update?
        website: data.website.clone(),
        crn: data.crn.clone(),
        vatin: data.vatin.clone(),
    };

    let address_update_data = AddressUpdateData {
        country: data.country.clone(),
        city: data.city.clone(),
        region: data.region.clone(),
        postal_code: data.postal_code.clone(),
        street: data.street.clone(),
        street_number: data.number.clone(),
    };

    let result = company_repo
        .update(parsed_id, company_update_data, address_update_data)
        .await;

    if let Ok(company) = result {
        let template = CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number,
            created_at: company.created_at,
            edited_at: company.edited_at,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
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

#[delete("/company/{company_id}")]
pub async fn delete_company(
    company_id: web::Path<String>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = company_repo.delete(parsed_id).await;

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

//TODO: Once file store/load is done.
#[get("/company/{company_id}/avatar")]
pub async fn get_company_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[put("/company/{company_id}/avatar")]
pub async fn upload_company_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[delete("/company/{company_id}/avatar")]
pub async fn remove_company_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}