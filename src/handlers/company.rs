use actix_web::{delete, get, patch, post, put, web, HttpResponse, http};
use askama::Template;
use serde::Deserialize;

use crate::{repositories::company::{company_repo::CompanyRepository, models::CompanyFilter}, handlers::common::QueryParams, templates::company::{CompaniesTemplate, CompanyLiteTemplate}, errors::parse_error};

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
pub async fn get_company(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[post("/company")]
pub async fn create_company(_new_company: web::Form<NewCompanyData>) -> HttpResponse {
    todo!()
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
