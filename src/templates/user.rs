use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::{Gender, UserRole, UserStatus};

#[derive(Template, Deserialize)]
#[template(path = "user/user.html")]
pub struct UserTemplate {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub birth: NaiveDate,
    pub avatar_url: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "user/user-lite.html")]
pub struct UserLiteTemplate {
    pub id: Uuid,
    pub name: String,
    pub status: UserStatus,
    pub age: u8,
    pub gender: Gender,
    pub avatar_url: String,
}
