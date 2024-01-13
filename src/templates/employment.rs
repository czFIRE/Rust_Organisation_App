use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{
    models::{EmployeeLevel, EmploymentContract},
    repositories::employment::models::{EmploymentExtended, Subordinate},
};

use super::{company::CompanyLite, user::UserLiteTemplate};

#[derive(Template, Deserialize)]
#[template(path = "employment/employment.html")]
pub struct EmploymentTemplate {
    pub user_id: Uuid,
    pub company: CompanyLite,
    pub manager: Option<UserLiteTemplate>,
    pub employment_type: EmploymentContract,
    pub hourly_wage: f64,
    pub level: EmployeeLevel,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<EmploymentExtended> for EmploymentTemplate {
    fn from(employment: EmploymentExtended) -> Self {
        let manager = employment.manager.map(|user| user.into());

        EmploymentTemplate {
            user_id: employment.user_id,
            company: employment.company.into(),
            manager,
            employment_type: employment.employment_type,
            hourly_wage: employment.hourly_wage,
            level: employment.level,
            description: employment
                .description
                .unwrap_or("No description.".to_string()),
            start_date: employment.start_date,
            end_date: employment.end_date,
            created_at: employment.created_at,
            edited_at: employment.edited_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EmploymentLite {
    pub user_id: Uuid,
    pub company: CompanyLite,
    pub employment_type: EmploymentContract,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}


impl From<EmploymentExtended> for EmploymentLite {
    fn from(employment: EmploymentExtended) -> Self {
        EmploymentLite {
            user_id: employment.user_id,
            company: employment.company.into(),
            employment_type: employment.employment_type,
            start_date: employment.start_date,
            end_date: employment.end_date,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/employment-lite.html")]
pub struct EmploymentLiteTemplate {
    pub user_id: Uuid,
    pub company: CompanyLite,
    pub employment_type: EmploymentContract,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl From<EmploymentExtended> for EmploymentLiteTemplate {
    fn from(employment: EmploymentExtended) -> Self {
        EmploymentLiteTemplate {
            user_id: employment.user_id,
            company: employment.company.into(),
            employment_type: employment.employment_type,
            start_date: employment.start_date,
            end_date: employment.end_date,
        }

        // EmploymentLiteTemplate { employment }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/employments.html")]
pub struct EmploymentsTemplate {
    pub employments: Vec<EmploymentLite>,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/employment-edit.html")]
pub struct EmploymentEditTemplate {
    pub editor_id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmploymentContract,
    pub level: EmployeeLevel,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "employment/subordinates.html")]
pub struct SubordinatesTemplate {
    pub user_id: Uuid,
    pub subordinates: Vec<Subordinate>
}
