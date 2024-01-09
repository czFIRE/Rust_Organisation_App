use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::QueryParams,
    repositories::company::{
        company_repo::CompanyRepository,
        models::{AddressData, AddressUpdateData, CompanyData, CompanyFilter, NewCompany},
    },
    templates::company::{CompaniesTemplate, CompanyLiteTemplate, CompanyTemplate},
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

    handle_database_error(result.err().expect("Should be error."))
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
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[post("/company")]
pub async fn create_company(
    new_company: web::Json<NewCompanyData>,
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
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

// TODO: This is rather ugly. Might rewrite if there is time left at the end. :copium:
fn is_update_data_empty(company_data: CompanyUpdateData) -> bool {
    (company_data.name.is_none()
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
        && company_data.email.is_none())
        || (company_data.name.is_some() && company_data.name.unwrap().is_empty())
        || (company_data.description.is_some() && company_data.description.unwrap().is_empty())
        || (company_data.website.is_some() && company_data.website.unwrap().is_empty())
        || (company_data.crn.is_some() && company_data.crn.unwrap().is_empty())
        || (company_data.vatin.is_some() && company_data.vatin.unwrap().is_empty())
        || (company_data.country.is_some() && company_data.country.unwrap().is_empty())
        || (company_data.region.is_some() && company_data.region.unwrap().is_empty())
        || (company_data.city.is_some() && company_data.city.unwrap().is_empty())
        || (company_data.street.is_some() && company_data.street.unwrap().is_empty())
        || (company_data.number.is_some() && company_data.number.unwrap().is_empty())
        || (company_data.postal_code.is_some() && company_data.postal_code.unwrap().is_empty())
        || (company_data.phone.is_some() && company_data.phone.unwrap().is_empty())
        || (company_data.email.is_some() && company_data.email.unwrap().is_empty())
}

#[patch("/company/{company_id}")]
pub async fn update_company(
    company_id: web::Path<String>,
    company_data: web::Json<CompanyUpdateData>,
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
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.err().expect("Should be error."))
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
        return handle_database_error(error);
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
