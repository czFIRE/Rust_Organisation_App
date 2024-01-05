use std::str::FromStr;

use actix_web::{delete, get, patch, post, put, web, HttpResponse, http};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{repositories::company::{company_repo::CompanyRepository, models::{CompanyFilter, NewCompany, Address, AddressData}}, handlers::common::QueryParams, templates::{company::{CompaniesTemplate, CompanyLiteTemplate, CompanyTemplate}, self}, errors::parse_error};

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

#[derive(Deserialize)]
pub struct CompanyData {
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
pub async fn get_all_companies(params: web::Query<QueryParams>, company_repo: web::Data<CompanyRepository>) -> HttpResponse {
    let query_params = params.into_inner();
    
    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let filter = CompanyFilter {
        limit: query_params.limit,
        offset: query_params.offset
    };
    
    let result = company_repo.read_all(filter).await;

    if let Ok(companies) = result {
        let lite_companies = companies.into_iter().map(| company | CompanyLiteTemplate {
            id: company.id,
            name: company.name,
            avatar_url: company.avatar_url
        }).collect();

        let template = CompaniesTemplate {
            companies: lite_companies
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be okay now."));
    }
    
    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

#[get("/company/{company_id}")]
pub async fn get_company(company_id: web::Path<String>, company_repo: web::Data<CompanyRepository>) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = company_repo.read_one_extended(parsed_id).await;

    if let Ok(company) = result {
        let address = templates::company::Address {
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number
        };

        let template = CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            address,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            created_at: company.created_at,
            edited_at: company.edited_at
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid."));
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
pub async fn create_company(new_company: web::Form<NewCompanyData>, company_repo: web::Data<CompanyRepository>) -> HttpResponse {
    let company_data = NewCompany {
        name: new_company.name.clone(),
        description: new_company.description.clone(),
        phone: new_company.phone.clone(),
        email: new_company.email.clone(),
        website: new_company.website.clone(),
        crn: new_company.crn.clone(),
        vatin: new_company.vatin.clone()
    };

    let address = AddressData {
        country: new_company.country.clone(),
        region: new_company.region.clone(),
        city: new_company.city.clone(),
        postal_code: new_company.postal_code.clone(),
        street: new_company.street.clone(),
        street_number: new_company.number.clone()
    };

    let result = company_repo.create(company_data, address).await;
    
    if let Ok(company) = result {
        let address = templates::company::Address {
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number
        };

        let template = CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            address,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            created_at: company.created_at,
            edited_at: company.edited_at
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created().body(body.expect("Should be valid."));
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

#[patch("/company/{company_id}")]
pub async fn update_company(
    _id: web::Path<String>,
    _company_data: web::Form<CompanyData>,
) -> HttpResponse {
    todo!()
}

#[delete("/company/{company_id}")]
pub async fn delete_company(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

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
