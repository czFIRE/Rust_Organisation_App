use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{EmployeeContract, EmployeeLevel, Gender, UserRole, UserStatus},
    repositories::{company::models::Company, user::models::User},
};

#[derive(Debug)]
pub struct NewEmployment {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmployeeContract,
    pub level: EmployeeLevel,
}

#[derive(Debug, FromRow)]
pub struct Employment {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmployeeContract,
    pub level: EmployeeLevel,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct EmploymentExtended {
    pub user_id: Uuid,
    pub company: Company,
    pub manager_id: User,
    pub hourly_wage: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub employment_type: EmployeeContract,
    pub level: EmployeeLevel,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct EmploymentData {
    pub manager_id: Option<Uuid>,
    pub hourly_wage: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub employment_type: Option<EmployeeContract>,
    pub level: Option<EmployeeLevel>,
}

#[derive(Debug)]
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
    pub employment_type: EmployeeContract,
    pub employment_level: EmployeeLevel,
    pub employment_created_at: NaiveDateTime,
    pub employment_edited_at: NaiveDateTime,
    pub employment_deleted_at: Option<NaiveDateTime>,

    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_birth: NaiveDate,
    pub user_avatar_url: Option<String>, // TODO: Now is the same as in INIT.SQL but do we want this?
    pub user_gender: Gender,
    pub user_role: UserRole,
    pub user_status: UserStatus,
    pub user_created_at: NaiveDateTime,
    pub user_edited_at: NaiveDateTime,
    pub user_deleted_at: Option<NaiveDateTime>,

    pub company_id: Uuid,
    pub company_name: String,
    pub company_description: Option<String>,
    pub company_phone: String,
    pub company_email: String,
    pub company_avatar_url: Option<String>,
    pub company_website: Option<String>,
    pub company_crn: String,
    pub company_vatin: String,
    pub company_created_at: NaiveDateTime,
    pub company_edited_at: NaiveDateTime,
    pub company_deleted_at: Option<NaiveDateTime>,
}

impl From<EmploymentUserCompanyFlattened> for EmploymentExtended {
    fn from(value: EmploymentUserCompanyFlattened) -> Self {
        let tmp_user = User {
            id: value.user_id,
            name: value.user_name,
            email: value.user_email,
            birth: value.user_birth,
            avatar_url: value.user_avatar_url,
            gender: value.user_gender,
            role: value.user_role,
            status: value.user_status,
            created_at: value.user_created_at,
            edited_at: value.user_edited_at,
            deleted_at: value.user_deleted_at,
        };

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

        Self {
            user_id: value.employment_user_id,
            company: tmp_company,
            manager_id: tmp_user,
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
