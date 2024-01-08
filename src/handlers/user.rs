use std::str::FromStr;

use crate::{
    errors::{handle_database_error, parse_error},
    repositories::user::models::{NewUser, UserData},
    templates::user::UserTemplate,
};
use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
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

    handle_database_error(result.err().expect("Should be error."))
}

#[post("/user")]
pub async fn create_user(
    new_user: web::Form<NewUser>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    if new_user.name.len() == 0 || new_user.email.len() == 0 {
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

    handle_database_error(result.err().expect("Should be error."))
}

fn is_data_invalid(user_data: UserData) -> bool {
    (user_data.name.is_none()
        && user_data.email.is_none()
        && user_data.birth.is_none()
        && user_data.avatar_url.is_none()
        && user_data.role.is_none())
        || (user_data.name.is_some() && user_data.name.unwrap().len() == 0)
        || (user_data.email.is_some() && user_data.email.unwrap().len() == 0)
        || (user_data.avatar_url.is_some() && user_data.avatar_url.unwrap().len() == 0)
}

#[patch("/user/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    user_data: web::Form<UserData>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    if is_data_invalid(user_data.clone()) {
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

    handle_database_error(result.err().expect("Should be error."))
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
