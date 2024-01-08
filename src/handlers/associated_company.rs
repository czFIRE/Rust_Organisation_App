use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::Association,
    repositories::associated_company::{
        associated_company_repo::AssociatedCompanyRepository,
        models::{AssociatedCompanyData, AssociatedCompanyFilter, NewAssociatedCompany},
    },
    templates::company::{AssociatedCompaniesTemplate, AssociatedCompanyTemplate},
};

#[derive(Deserialize)]
pub struct NewAssociatedCompanyData {
    company_id: Uuid,
    association_type: Association,
}

#[get("/event/{event_id}/company")]
pub async fn get_all_associated_companies(
    event_id: web::Path<String>,
    query: web::Query<AssociatedCompanyFilter>,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() < 0)
        || (query.offset.is_some() && query.offset.clone().unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = associated_repo
        .read_all_companies_for_event(parsed_id, query.into_inner())
        .await;

    if let Ok(associated_companies) = result {
        let associated_companies_vec: Vec<AssociatedCompanyTemplate> = associated_companies
            .into_iter()
            .map(|company| company.into())
            .collect();
        let template = AssociatedCompaniesTemplate {
            associated_companies: associated_companies_vec,
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[post("/event/{event_id}/company")]
pub async fn create_associated_company(
    event_id: web::Path<String>,
    new_associated_company: web::Form<NewAssociatedCompanyData>,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let data = NewAssociatedCompany {
        company_id: new_associated_company.company_id,
        event_id: parsed_id,
        association_type: new_associated_company.association_type.clone(),
    };
    let result = associated_repo.create(data).await;

    if let Ok(company) = result {
        let template: AssociatedCompanyTemplate = company.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[patch("/event/{event_id}/company/{company_id}")]
pub async fn update_associated_company(
    path: web::Path<(String, String)>,
    associated_company_data: web::Form<AssociatedCompanyData>,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    if associated_company_data.association_type.is_none() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, company_id) = parsed_ids.unwrap();
    let result = associated_repo
        .update(company_id, event_id, associated_company_data.into_inner())
        .await;
    if let Ok(company) = result {
        let template: AssociatedCompanyTemplate = company.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[delete("/event/{event_id}/company/{company_id}")]
pub async fn delete_associated_company(
    path: web::Path<(String, String)>,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, company_id) = parsed_ids.unwrap();
    let result = associated_repo.delete(company_id, event_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}
