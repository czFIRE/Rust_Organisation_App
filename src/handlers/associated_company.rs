use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::Association;

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
    _event_id: web::Path<Uuid>,
    _query: web::Query<AssociatedCompanyQueryParams>,
) -> HttpResponse {
    todo!()
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
