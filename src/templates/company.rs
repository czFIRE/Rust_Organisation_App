use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{
    models::Association,
    repositories::{
        associated_company::models::AssociatedCompanyExtended,
        company::models::{Company, CompanyExtended},
    },
};

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

impl From<CompanyExtended> for CompanyTemplate {
    fn from(company: CompanyExtended) -> Self {
        CompanyTemplate {
            id: company.company_id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number,
            created_at: company.created_at,
            edited_at: company.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/company-lite.html")]
pub struct CompanyLiteTemplate {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: String,
}

impl From<Company> for CompanyLiteTemplate {
    fn from(company: Company) -> Self {
        CompanyLiteTemplate {
            id: company.id,
            name: company.name,
            avatar_url: company.avatar_url,
        }
    }
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

impl From<AssociatedCompanyExtended> for AssociatedCompanyTemplate {
    fn from(associated: AssociatedCompanyExtended) -> Self {
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
