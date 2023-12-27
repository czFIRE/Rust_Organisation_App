use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

// TODO needs to be kept the same as in staff/models.rs => StaffUserCompanyFlattened
// TODO needs to be kept the same as in employment/models.rs => EmploymentUserCompanyFlattened
#[derive(Debug, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
    // timestamps
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct CompanyData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub crn: Option<String>,
    pub vatin: Option<String>,
}

#[derive(Debug)]
pub struct CompanyFilters {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
