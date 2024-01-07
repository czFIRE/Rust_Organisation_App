use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{models::Association, repositories::associated_company::models::AssociatedCompanyExtented};

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
    pub phone: String,
    pub email: String,
    pub avatar_url: String,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub address_number: String,
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
    pub companies: Vec<CompanyLiteTemplate>,
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

impl From<AssociatedCompanyExtented> for AssociatedCompanyTemplate {
    fn from(associated: AssociatedCompanyExtented) -> Self {
        let company_lite = CompanyLiteTemplate {
            id: associated.company.id,
            name: associated.company.name,
            avatar_url: associated.company.avatar_url,
        };

        AssociatedCompanyTemplate {
            event_id: associated.event.id,
            company: company_lite,
            association_type: associated.association_type,
            created_at: associated.created_at,
            edited_at: associated.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-companies.html")]
pub struct AssociatedCompaniesTemplate {
    pub associated_companies: Vec<AssociatedCompanyTemplate>,
}
