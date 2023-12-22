use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

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
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub avatar_url: String,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
}

#[derive(Debug)]
pub struct CompanyFilters {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
