use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;

use crate::models::EmployeeLevel;

#[derive(Deserialize)]
pub struct NewEmploymentData {
    user_id: Uuid,
    company_id: Uuid,
    manager_id: Uuid,
    employment_type: EmployeeContract,
    hourly_rate: f64,
    employee_level: EmployeeLevel,
    description: Option<String>,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct EmploymentData {
    manager_id: Option<Uuid>,
    employment_type: Option<EmployeeContract>,
    hourly_rate: Option<f64>,
    employee_level: Option<EmployeeLevel>,
    description: Option<String>,
    start_date: Option<chrono::DateTime<Utc>>,
    end_date: Option<chrono::DateTime<Utc>>,
}

#[get("/user/{user_id}/employment")]
pub async fn get_employments_per_user(_user_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/user/{user_id}/employment/{company_id}")]
pub async fn get_employment(_user_id: web::Path<String>, _company_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/user/{user_id}/employment/{company_id}/subordinates")]
pub async fn get_subordinates(_user_id: web::Path<String>, _company_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[post("/employment")]
pub async fn create_employment(_new_employment: web::Form<NewEmploymentData>) -> HttpResponse {
    todo!()
}

#[patch("/user/{user_id}/employment/{company_id}")]
pub async fn update_employment(_user_id: web::Path<String>, _employment_data: web::Form<EmploymentData>) -> HttpResponse {
    todo!()
}

#[delete("/user/{user_id}/employment/{company_id}")]
pub async fn delete_employment(_user_id: web::Path<String>, _company_id: web::Path<String>) -> HttpResponse {
    todo!()
}