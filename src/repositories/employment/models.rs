use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{EmployeeLevel, EmploymentContract, Gender, UserRole, UserStatus},
    repositories::{company::models::Company, user::models::User},
};

#[derive(Debug, Deserialize, Clone)]
pub struct NewEmployment {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmploymentContract,
    pub level: EmployeeLevel,
}

#[derive(Debug, FromRow, Clone)]
pub struct Employment {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmploymentContract,
    pub level: EmployeeLevel,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct EmploymentExtended {
    pub user_id: Uuid,
    pub company: Company,
    pub manager: Option<User>,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmploymentContract,
    pub level: EmployeeLevel,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmploymentData {
    pub manager_id: Option<Uuid>,
    pub hourly_wage: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub employment_type: Option<EmploymentContract>,
    pub level: Option<EmployeeLevel>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmploymentFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

//////////////////////////////////////////

// TODO needs to be kept the same as in user/models.rs => User
// TODO needs to be kept the same as in company/models.rs => Company
// TODO needs to be kept the same as in employment/models.rs => Employment
#[derive(Debug, FromRow)]
pub struct EmploymentUserCompanyFlattened {
    pub employment_user_id: Uuid,
    pub employment_company_id: Uuid,
    pub employment_manager_id: Option<Uuid>,
    pub employment_hourly_wage: f64,
    pub employment_start_date: NaiveDate,
    pub employment_end_date: NaiveDate,
    pub employment_description: Option<String>,
    pub employment_type: EmploymentContract,
    pub employment_level: EmployeeLevel,
    pub employment_created_at: NaiveDateTime,
    pub employment_edited_at: NaiveDateTime,
    pub employment_deleted_at: Option<NaiveDateTime>,

    pub manager_id: Option<Uuid>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub manager_birth: Option<NaiveDate>,
    pub manager_avatar_url: Option<String>,
    pub manager_gender: Option<Gender>,
    pub manager_role: Option<UserRole>,
    pub manager_status: Option<UserStatus>,
    pub manager_created_at: Option<NaiveDateTime>,
    pub manager_edited_at: Option<NaiveDateTime>,
    pub manager_deleted_at: Option<NaiveDateTime>,

    pub company_id: Uuid,
    pub company_name: String,
    pub company_description: Option<String>,
    pub company_phone: String,
    pub company_email: String,
    pub company_avatar_url: String,
    pub company_website: Option<String>,
    pub company_crn: String,
    pub company_vatin: String,
    pub company_created_at: NaiveDateTime,
    pub company_edited_at: NaiveDateTime,
    pub company_deleted_at: Option<NaiveDateTime>,
}

impl From<EmploymentUserCompanyFlattened> for EmploymentExtended {
    fn from(value: EmploymentUserCompanyFlattened) -> Self {
        let tmp_company = Company {
            id: value.company_id,
            name: value.company_name,
            description: value.company_description,
            phone: value.company_phone,
            email: value.company_email,
            avatar_url: value.company_avatar_url,
            website: value.company_website,
            crn: value.company_crn,
            vatin: value.company_vatin,
            created_at: value.company_created_at,
            edited_at: value.company_edited_at,
            deleted_at: value.company_deleted_at,
        };

        let tmp_manager = match value.manager_id {
            None => None,
            Some(_) => Some(User {
                id: value.manager_id.unwrap(),
                name: value.manager_name.unwrap(),
                email: value.manager_email.unwrap(),
                birth: value.manager_birth.unwrap(),
                avatar_url: value.manager_avatar_url.unwrap(),
                gender: value.manager_gender.unwrap(),
                role: value.manager_role.unwrap(),
                status: value.manager_status.unwrap(),
                created_at: value.manager_created_at.unwrap(),
                edited_at: value.manager_edited_at.unwrap(),
                deleted_at: value.manager_deleted_at,
            }),
        };

        Self {
            user_id: value.employment_user_id,
            company: tmp_company,
            manager: tmp_manager,
            hourly_wage: value.employment_hourly_wage,
            start_date: value.employment_start_date,
            end_date: value.employment_end_date,
            description: value.employment_description,
            employment_type: value.employment_type,
            level: value.employment_level,
            created_at: value.employment_created_at,
            edited_at: value.employment_edited_at,
            deleted_at: value.employment_deleted_at,
        }
    }
}
