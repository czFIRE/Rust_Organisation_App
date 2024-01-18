use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::{
    models::{Gender, UserRole, UserStatus},
    repositories::user::models::{User, UserLite},
};

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
    pub edited_at: NaiveDateTime,
}

impl From<User> for UserTemplate {
    fn from(user: User) -> Self {
        UserTemplate {
            id: user.id,
            name: user.name,
            email: user.email,
            birth: user.birth,
            avatar_url: user.avatar_url,
            role: user.role,
            status: user.status,
            gender: user.gender,
            created_at: user.created_at,
            edited_at: user.edited_at,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "user/user-lite.html")]
pub struct UserLiteTemplate {
    pub id: Uuid,
    pub name: String,
    pub status: UserStatus,
    pub age: u32,
    pub gender: Gender,
    pub avatar_url: String,
}

impl From<UserLite> for UserLiteTemplate {
    fn from(value: UserLite) -> Self {
        UserLiteTemplate {
            id: value.id,
            name: value.name,
            status: value.status,
            age: chrono::offset::Local::now()
                .naive_local()
                .date()
                .years_since(value.birth)
                .expect("Should be valid"),
            gender: value.gender,
            avatar_url: value.avatar_url,
        }
    }
}

impl From<User> for UserLiteTemplate {
    fn from(user: User) -> UserLiteTemplate {
        UserLiteTemplate {
            id: user.id,
            name: user.name,
            status: user.status,
            age: chrono::offset::Local::now()
                .naive_local()
                .date()
                .years_since(user.birth)
                .expect("Should be valid"),
            gender: user.gender,
            avatar_url: user.avatar_url,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "user/users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserLiteTemplate>,
}

#[derive(Template)]
#[template(path = "user/user-edit.html")]
pub struct UserEditTemplate {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub birth: NaiveDate,
    pub gender: Gender,
}

pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        UserInfo {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

#[derive(Template)]
#[template(path = "user/user-info.html")]
pub struct UserInfoTemplate {
    pub user_info_vec: Vec<UserInfo>,
}

#[derive(Template)]
#[template(path = "user/admin/admin.html")]
pub struct AdminTemplate {
    pub title: String,
}
