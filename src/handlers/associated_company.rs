use std::str::FromStr;

use actix_web::{delete, get, patch, post, web, HttpResponse, http};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::Association, repositories::associated_company::{models::AssociatedCompanyFilter, associated_company_repo::AssociatedCompanyRepository}, errors::parse_error, templates::company::{AssociatedCompanyTemplate, AssociatedCompaniesTemplate}};

#[derive(Deserialize)]
pub struct NewAssociatedCompanyData {
    company_id: Uuid,
    association_type: Association,
}

#[derive(Deserialize)]
pub struct AssociatedCompanyData {
    association_type: Option<Association>,
}

#[derive(Deserialize)]
pub struct AssociatedCompanyQueryParams {
    association_type: Option<Association>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/event/{event_id}/company")]
pub async fn get_all_associated_companies(
    event_id: web::Path<String>,
    query: web::Query<AssociatedCompanyFilter>,
    associated_repo: web::Data<AssociatedCompanyRepository>
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() < 0)
        || (query.offset.is_some() && query.offset.clone().unwrap() < 0) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    } 

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = associated_repo.read_all_companies_for_event(parsed_id, query.into_inner()).await;
    
    if let Ok(associated_companies) = result {
        let associated_companies_vec: Vec<AssociatedCompanyTemplate> = associated_companies.into_iter().map(|company| company.into()).collect();
        let template = AssociatedCompaniesTemplate {
            associated_companies: associated_companies_vec,
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::BAD_REQUEST));
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

#[post("/event/{event_id}/company")]
pub async fn create_associated_company(
    _event_id: web::Path<Uuid>,
    _new_associated_company: web::Form<NewAssociatedCompanyData>,
) -> HttpResponse {
    todo!()
}

#[patch("/event/{event_id}/company/{company_id}")]
pub async fn update_associated_company(
    _event_id: web::Path<String>,
    _company_id: web::Path<String>,
    _associated_company_data: web::Form<AssociatedCompanyData>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/company/{company_id}")]
pub async fn delete_associated_company(
    _event_id: web::Path<String>,
    _company_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}
