use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::{EmployeeContract, EmployeeLevel},
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
