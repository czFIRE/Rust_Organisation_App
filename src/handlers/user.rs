use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;

use crate::models::{Gender, UserRole};

#[derive(Deserialize)]
pub struct NewUserData {
    name: String,
    email: String,
    birth: chrono::DateTime<Utc>,
    gender: Gender,
    role: UserRole,
}

#[derive(Deserialize)]
pub struct UserData {
    name: Option<String>,
    email: Option<String>,
    birth: Option<chrono::DateTime<Utc>>,
    gender: Option<Gender>,
    role: Option<UserRole>,
}

#[get("/user/{user_id}")]
pub async fn get_user(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[post("/user")]
pub async fn create_user(_new_user: web::Form<NewUserData>) -> HttpResponse {
    todo!()
}

#[patch("/user/{user_id}")]
pub async fn update_user(_id: web::Path<String>, _user_data: web::Form<UserData>) -> HttpResponse {
    todo!()
}

#[delete("/user/{user_id}")]
pub async fn delete_user(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/user/{user_id}/avatar")]
pub async fn get_user_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[put("/user/{user_id}/avatar")]
pub async fn upload_user_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[delete("/user/{user_id}/avatar")]
pub async fn remove_user_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}
