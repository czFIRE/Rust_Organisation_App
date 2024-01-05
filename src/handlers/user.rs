use std::str::FromStr;

use actix_web::{delete, get, patch, post, put, web, HttpResponse, http};
use askama::Template;
use chrono::Utc;
use crate::{templates::user::UserTemplate, errors::parse_error};
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::{Gender, UserRole}, repositories::user::user_repo::UserRepository};

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
pub async fn get_user(user_id: web::Path<String>, user_repo: web::Data<UserRepository>) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        let error = parse_error(http::StatusCode::BAD_REQUEST);
        return HttpResponse::BadRequest().body(error)
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let query_result = user_repo.read_one(parsed_id).await;

    if let Ok(user) = query_result {
        let template = UserTemplate {
            id: user.id,
            name: user.name,
            email: user.email,
            birth: user.birth,
            avatar_url: user.avatar_url,
            role: user.role,
            status: user.status,
            gender: user.gender,
            created_at: user.created_at
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(body.expect("Should be valid"));
    }

    let error = query_result.err().expect("Should be an error");
    match (error) {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
    }
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
