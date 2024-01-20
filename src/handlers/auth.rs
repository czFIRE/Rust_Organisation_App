use crate::auth::models::{AccessToken, Login, Register};
use crate::auth::openid::get_token;
use crate::errors::{handle_database_error, parse_error};
use crate::models::{Gender, UserRole};
use crate::repositories::user::models::NewUser;
use crate::repositories::user::user_repo::UserRepository;
use crate::templates::common::IndexTemplate;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use askama::Template;
use chrono::NaiveDate;
use reqwest::Client;

use actix_files::Files as ActixFiles;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use serde_json::json;
use sqlx::{Pool, Postgres};

use std::sync::Arc;

use crate::{
    handlers::user::get_users_login,
    repositories::assigned_staff::assigned_staff_repo::AssignedStaffRepository,
};
use actix_web::{http, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/register")]
async fn register(
    web::Form(form): web::Form<Register>,
    user_repository: web::Data<UserRepository>,
) -> HttpResponse {
    // Get admin console token for registration purposes.
    let path = "http://localhost:9090/realms/master/protocol/openid-connect/token";

    let payload = json!({
        "username": std::env::var("KEYCLOAK_ADMIN").expect("Should be set"),
        "password": std::env::var("KEYCLOAK_PASSWORD").expect("Should be set"),
        "grant_type": "password",
        "client_id": std::env::var("KEYCLOAK_REG_CLIENT").expect("Should be set"),
    });

    let result = get_token(path, payload).await;
    if result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    // We kinda juggle the token around to get the data. This doesn't work yet.
    let token = result.expect("Should be okay.");
    let token_json = serde_json::to_string(&token);
    if token_json.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let token_str = token_json.expect("Should be okay");
    let access_json: Result<AccessToken, serde_json::Error> = serde_json::from_str(&token_str);
    if access_json.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let access = access_json.expect("Should be valid");

    let path = "http://localhost:8080/admin/realms/Orchestrate/users";

    let payload = json!({
        "firstName": form.name,
        "lastName": form.name,
        "email": form.email,
        "enabled": "true",
        "username": form.email
    });

    let payload_str = serde_json::to_string(&payload);
    if payload_str.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let request = Client::new()
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(payload_str.expect("Should be valid."))
        .bearer_auth(access.access_token);
    let response = request.send().await;

    if response.is_err() {
        return HttpResponse::BadRequest().finish();
    }

    let response_exp = response.expect("Should be valid here.");
    if response_exp.status() != http::StatusCode::CREATED {
        return HttpResponse::BadRequest().finish();
    }
    let user_data = NewUser {
        name: form.name,
        email: form.email,
        birth: NaiveDate::from_ymd_opt(1999, 02, 20).expect("Should be some"),
        gender: Gender::Male,
        role: UserRole::User,
    };
    let user_res = user_repository.create(user_data).await;

    if user_res.is_err() {
        return handle_database_error(user_res.expect_err("Should be an error."));
    }
    let template = IndexTemplate {
        landing_title: "Log in to your new account!".to_string(),
    };
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Created().body(body.expect("Should be valid"))
}

#[post("/login")]
async fn login(web::Form(form): web::Form<Login>) -> impl Responder {
    // The path variable stores the URL of the authentication server
    let path = "http://localhost:8080/realms/Orchestrate/protocol/openid-connect/token";

    // The payload variable stores the JSON object with the login credentials and the client information
    let payload = json!({
        "username": form.username,
        "password": form.password,
        "client_id": std::env::var("CLIENT_ID").expect("Should be set"),
        "client_secret": std::env::var("CLIENT_SECRET").expect("Should be set"),
        "grant_type": "password"
    });

    let result = get_token(path, payload).await;

    if result.is_err() {
        return HttpResponse::BadRequest().finish();
    }

    let serialized = serde_json::to_string(&result.expect("Should be valid")).unwrap();

    // The function returns an HTTP response with status code 200 and the serialized result as the body
    HttpResponse::Ok().body(serialized)
}
