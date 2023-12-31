use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::Association;

#[derive(Deserialize)]
pub struct Address {
    pub country: String,
    pub region: String,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub address_number: String,
}

#[derive(Template, Deserialize)]
#[template(path = "company/company.html")]
pub struct CompanyTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub address: Address,
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

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/company-lite.html")]
pub struct CompanyLiteTemplate {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: String,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/companies.html")]
pub struct CompaniesTemplate {
    pub companies: Vec<CompanyLiteTemplate>
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-company.html")]
pub struct AssociatedCompanyTemplate {
    pub event_id: Uuid,
    pub company: CompanyLiteTemplate,
    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}


#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-companies.html")]
pub struct AssociatedCompaniesTemplate {
    pub associated_companies: Vec<AssociatedCompanyTemplate>,
}