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
    templates::company::{CompaniesTemplate, CompanyLite, CompanyTemplate},
    utils::format_check::check::{check_email_validity, check_phone_validity},
};

#[derive(Deserialize, Clone)]
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

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
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
            .map(|company| CompanyLite {
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

    handle_database_error(result.expect_err("Should be error."))
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

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_creation_data(data: NewCompanyData) -> bool {
    if data.name.is_empty()
        || data.phone.is_empty()
        || data.email.is_empty()
        || data.crn.is_empty()
        || data.vatin.is_empty()
        || data.region.is_empty()
        || data.country.is_empty()
        || data.city.is_empty()
        || data.postal_code.is_empty()
        || data.street.is_empty()
    {
        return false;
    }

    if (data.description.is_some() && data.description.unwrap().is_empty())
        || (data.website.is_some() && data.website.unwrap().is_empty())
    {
        return false;
    }

    if !check_email_validity(data.email) {
        return false;
    }

    check_phone_validity(data.phone)
}

#[post("/company")]
pub async fn create_company(
    new_company: web::Json<NewCompanyData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let data = new_company.into_inner();
    if !validate_creation_data(data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let company_data = NewCompany {
        name: data.name.clone(),
        description: data.description.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        website: data.website.clone(),
        crn: data.crn.clone(),
        vatin: data.vatin.clone(),
    };

    let address = AddressData {
        country: data.country.clone(),
        region: data.region.clone(),
        city: data.city.clone(),
        postal_code: data.postal_code.clone(),
        street: data.street.clone(),
        street_number: data.number.clone(),
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

    handle_database_error(result.expect_err("Should be error."))
}

fn is_data_empty(company_data: &CompanyUpdateData) -> bool {
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

/*
 * On the semantics of 'formatless'. Some strings here could definitely
 * have some format imposed upon them, but we ultimately decided that it may
 * be wiser to leave this as it is and not bother with restricting format of
 * things like CRN and VATIN.
 * Similarly, postal codes could vary for different locations.
 */
fn any_formatless_string_empty(company_data: &CompanyUpdateData) -> bool {
    (company_data.name.is_some() && company_data.name.as_ref().unwrap().is_empty())
        || (company_data.description.is_some()
            && company_data.description.as_ref().unwrap().is_empty())
        || (company_data.website.is_some() && company_data.website.as_ref().unwrap().is_empty())
        || (company_data.crn.is_some() && company_data.crn.as_ref().unwrap().is_empty())
        || (company_data.vatin.is_some() && company_data.vatin.as_ref().unwrap().is_empty())
        || (company_data.country.is_some() && company_data.country.as_ref().unwrap().is_empty())
        || (company_data.region.is_some() && company_data.region.as_ref().unwrap().is_empty())
        || (company_data.city.is_some() && company_data.city.as_ref().unwrap().is_empty())
        || (company_data.street.is_some() && company_data.street.as_ref().unwrap().is_empty())
        || (company_data.number.is_some() && company_data.number.as_ref().unwrap().is_empty())
        || (company_data.postal_code.is_some()
            && company_data.postal_code.as_ref().unwrap().is_empty())
}

fn validate_update_data(company_data: CompanyUpdateData) -> bool {
    if is_data_empty(&company_data) {
        return false;
    }

    if any_formatless_string_empty(&company_data) {
        return false;
    }

    if company_data.email.is_some() && !check_email_validity(company_data.email.unwrap()) {
        return false;
    }

    if company_data.phone.is_some() && !check_phone_validity(company_data.phone.unwrap()) {
        return false;
    }
    true
}

#[patch("/company/{company_id}")]
pub async fn update_company(
    company_id: web::Path<String>,
    company_data: web::Json<CompanyUpdateData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let data = company_data.into_inner();

    if !validate_update_data(data.clone()) {
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

    handle_database_error(result.expect_err("Should be error."))
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
