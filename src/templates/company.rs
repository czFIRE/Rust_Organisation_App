use askama::Template;
use chrono::NaiveDateTime;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::Association;

#[derive(Template)]
#[template(path = "company/company.html")]
pub struct CompanyTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub avatar_url: String,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
    // timestamps
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Template)]
#[template(path = "company/company-lite.html")]
pub struct CompanyLiteTemplate {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: String,
}

#[derive(Template)]
#[template(path = "company/associated-company.html")]
pub struct AssociatedCompanyTemplate {
    pub event_id: Uuid,
    pub company: CompanyLiteTemplate,
    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}
