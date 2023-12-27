use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewCompanyData {
    name: String,
    description: Option<String>,
    webiste: Option<String>,
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
    webiste: Option<String>,
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
pub async fn get_all_companies() -> HttpResponse {
    todo!()
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
