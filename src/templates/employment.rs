use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::{EmployeeContract, EmployeeLevel};

use super::{company::CompanyLiteTemplate, user::UserLiteTemplate};

#[derive(Template, Deserialize)]
#[template(path = "employment/employment.html")]
pub struct EmploymentTemplate {
    pub user_id: Uuid,
    pub company: CompanyLiteTemplate,
    pub manager: UserLiteTemplate,
    pub employment_type: EmployeeContract,
    pub hourly_wage: u32,
    pub level: EmployeeLevel,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/employment-lite.html")]
pub struct EmploymentLiteTemplate {
    pub user_id: Uuid,
    pub company: CompanyLiteTemplate,
    pub employment_type: EmployeeContract,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/employments.html")]
pub struct EmploymentsTemplate {
    pub employments: Vec<EmploymentLiteTemplate>,
}