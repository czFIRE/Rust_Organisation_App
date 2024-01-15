use std::str::FromStr;

use crate::{
    errors::{handle_database_error, parse_error},
    repositories::user::models::{NewUser, UserData},
    templates::user::{UserEditTemplate, UserLiteTemplate, UserTemplate, UsersTemplate}, utils::format_check::check::check_email_validity,
};
use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use chrono::Utc;
use uuid::Uuid;

use crate::repositories::user::user_repo::UserRepository;

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
    let result = user_repo.read_one(parsed_id).await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid"));
    }

    handle_database_error(result.expect_err("Should be error."))
}

// Temporary workaround to the lack of auth.
#[get("/user")]
pub async fn get_users(user_repo: web::Data<UserRepository>) -> HttpResponse {
    let result = user_repo._read_all().await;

    if let Ok(users) = result {
        let lite_users: Vec<UserLiteTemplate> = users.into_iter().map(|user| user.into()).collect();
        let template: UsersTemplate = UsersTemplate { users: lite_users };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid"));
    }

    handle_database_error(result.expect_err("Should be error."))
}

// For switching the user view into edit mode.
#[get("/user/{user_id}/mode")]
pub async fn toggle_user_edit(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = user_repo.read_one(parsed_id).await;

    if let Ok(user) = result {
        let template: UserEditTemplate = UserEditTemplate {
            id: user.id,
            name: user.name,
            email: user.email,
            birth: user.birth,
            gender: user.gender,
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

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_new_user(new_user: NewUser) -> bool {
    if new_user.name.is_empty()
        || new_user.email.is_empty()
        || new_user.birth >= Utc::now().date_naive()
    {
        return false;
    }

    check_email_validity(new_user.email)
}

#[post("/user")]
pub async fn create_user(
    new_user: web::Json<NewUser>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    if !validate_new_user(new_user.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let result = user_repo.create(new_user.into_inner()).await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();
        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        let unwrapped_body = body.unwrap();

        return HttpResponse::Created()
            .content_type("text/html")
            .body(unwrapped_body);
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_edit_data(user_data: UserData) -> bool {
    if user_data.name.is_none()
        && user_data.email.is_none()
        && user_data.birth.is_none()
        && user_data.avatar_url.is_none()
        && user_data.role.is_none()
    {
        return false;
    }

    if user_data.name.is_some() && user_data.name.unwrap().is_empty() {
        return false;
    }

    if user_data.email.is_some() && !check_email_validity(user_data.email.clone().unwrap()) {
        return false;
    }

    if user_data.avatar_url.is_some() && user_data.avatar_url.unwrap().is_empty() {
        return false;
    }

    !(user_data.birth.is_some() && user_data.birth.unwrap() >= Utc::now().date_naive())
}

#[patch("/user/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    user_data: web::Json<UserData>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    if !validate_edit_data(user_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = user_repo
        .update_user(parsed_id, user_data.into_inner())
        .await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be okay."));
    }

    handle_database_error(result.expect_err("Should be error."))
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
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
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
