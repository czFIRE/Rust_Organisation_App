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

#[derive(Template, Deserialize)]
#[template(path = "company/company-edit.html")]
pub struct CompanyEditTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub address_number: String,
}

#[derive(Debug, Deserialize)]
pub struct CompanyLite {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: String,
}

impl From<Company> for CompanyLite {
    fn from(company: Company) -> Self {
        CompanyLite {
            id: company.id,
            name: company.name,
            avatar_url: company.avatar_url,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/companies.html")]
pub struct CompaniesTemplate {
    pub companies: Vec<CompanyLite>,
}

#[derive(Debug, Deserialize)]
pub struct AssociatedCompanyInfo {
    pub event_id: Uuid,
    pub company: CompanyLite,
    pub association_type: Association,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<AssociatedCompanyExtended> for AssociatedCompanyInfo {
    fn from(associated: AssociatedCompanyExtended) -> Self {
        let company_lite = CompanyLite {
            id: associated.company.id,
            name: associated.company.name,
            avatar_url: associated.company.avatar_url,
        };

        AssociatedCompanyInfo {
            event_id: associated.event.id,
            company: company_lite,
            association_type: associated.association_type,
            created_at: associated.created_at,
            edited_at: associated.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-company/associated-companies.html")]
pub struct AssociatedCompaniesTemplate {
    pub editable: bool,
    pub associated_companies: Vec<AssociatedCompanyInfo>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-company/associated-company-management.html")]
pub struct AssociatedCompanyManagementTemplate {
    pub event_id: Uuid,
    pub companies: Vec<AssociatedCompanyInfo>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-company/associated-company-edit.html")]
pub struct AssociatedCompanyEditTemplate {
    pub company: AssociatedCompanyInfo,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/associated-company/associated-company-editable.html")]
pub struct EditableAssociatedCompanyTemplate {
    pub company: AssociatedCompanyInfo,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "company/companies-info.html")]
pub struct CompaniesInfoTemplate {
    pub companies: Vec<CompanyLite>,
}
