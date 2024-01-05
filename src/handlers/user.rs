use std::str::FromStr;

use crate::{
    errors::parse_error,
    repositories::user::models::{NewUser, UserData},
    templates::user::UserTemplate,
};
use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{Gender, UserRole},
    repositories::user::user_repo::UserRepository,
};

#[derive(Deserialize)]
pub struct NewUserData {
    name: String,
    email: String,
    birth: NaiveDate,
    gender: Gender,
    role: UserRole,
}

#[get("/user/{user_id}")]
pub async fn get_user(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
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
            created_at: user.created_at,
            edited_at: user.edited_at,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid"));
    }

    let error = query_result.err().expect("Should be an error");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[post("/user")]
pub async fn create_user(
    new_user: web::Form<NewUserData>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let user_data = NewUser {
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        birth: new_user.birth.clone(),
        gender: new_user.gender.clone(),
        role: new_user.role.clone(),
    };

    let created_user = user_repo.create(user_data).await;

    if let Ok(user) = created_user {
        let template = UserTemplate {
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
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        let unwrapped_body = body.unwrap();

        println!("{}", unwrapped_body);

        return HttpResponse::Created()
            .content_type("text/html")
            .body(unwrapped_body);
    }

    let error = created_user.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[patch("/user/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    user_data: web::Form<UserData>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    if user_data.name.is_none()
        && user_data.email.is_none()
        && user_data.birth.is_none()
        && user_data.avatar_url.is_none()
        && user_data.role.is_none()
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let updated_user = user_repo
        .update_user(parsed_id, user_data.into_inner())
        .await;

    if let Ok(user) = updated_user {
        let template = UserTemplate {
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
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be okay."));
    }

    let error = updated_user.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[delete("/user/{user_id}")]
pub async fn delete_user(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = user_repo.delete_user(parsed_id).await;

    if let Err(error) = result {
        return match error {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
    }

    HttpResponse::NoContent().body("Deleted.")
}

//TODO: Once file store/load is done.
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
