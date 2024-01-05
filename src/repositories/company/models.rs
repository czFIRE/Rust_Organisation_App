use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

// TODO needs to be kept the same as in staff/models.rs => StaffUserCompanyFlattened
// TODO needs to be kept the same as in employment/models.rs => EmploymentUserCompanyFlattened
#[derive(Debug, FromRow, Clone)]
pub struct Company {
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
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow, Clone)]
pub struct CompanyExtended {
    pub company_id: Uuid,
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
    pub deleted_at: Option<NaiveDateTime>,
    // address
    pub country: String,
    pub region: String,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub street_number: String,
}

#[derive(Debug, Clone)]
pub struct NewCompany {
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CompanyFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, FromRow, Clone)]
pub struct Address {
    pub company_id: Uuid,
    pub country: String,
    pub region: String,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub street_number: String,
}

#[derive(Debug, Clone)]
pub struct AddressData {
    pub country: String,
    pub region: String,
    pub city: String,
    pub postal_code: String,
    pub street: String,
    pub street_number: String,
}

#[derive(Debug, Clone)]
pub struct AddressUpdateData {
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
    pub street_number: Option<String>,
}
