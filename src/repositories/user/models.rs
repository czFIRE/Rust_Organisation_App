use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::{Gender, UserRole, UserStatus};

#[derive(Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub birth: NaiveDate,
    pub gender: Gender,
    pub role: UserRole,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub birth: NaiveDate,
    pub avatar_url: String,
    pub gender: Gender,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct UserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub birth: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub role: Option<UserRole>,
    pub avatar_url: Option<String>,
}
