mod auth;
mod common;
mod configs;
mod errors;
mod handlers;
mod models;
mod repositories;
mod templates;
mod utils;

use actix_web::http::header::{CONTENT_TYPE, HeaderValue};
use askama::Template;
use chrono::NaiveDate;
use organization::errors::handle_database_error;
use reqwest::Client;
use crate::auth::models::{Login, Register};
use crate::auth::openid::get_token;
use crate::errors::parse_error;
use crate::models::{Gender, UserRole};
use crate::repositories::user::models::NewUser;
use crate::templates::common::IndexTemplate;

use actix_files::Files as ActixFiles;
use actix_web::{middleware::Logger, App, HttpServer};
use auth::models::{Token, AccessToken};
use dotenv::dotenv;
use env_logger::Env;
use serde_json::json;
use sqlx::{Pool, Postgres};

use std::sync::Arc;

use crate::configs::assigned_staff_config::configure_assigned_staff_endpoints;
use crate::configs::associated_company_config::configure_associated_company_endpoints;
use crate::configs::comment_config::configure_comment_endpoints;
use crate::configs::company_config::configure_company_endpoints;
use crate::configs::employment_config::configure_employment_endpoints;
use crate::configs::event_config::configure_event_endpoints;
use crate::configs::staff_config::configure_staff_endpoints;
use crate::configs::task_config::configure_task_endpoints;
use crate::configs::timesheet_config::configure_timesheet_endpoints;
use crate::configs::user_config::configure_user_endpoints;

use crate::handlers::index::index;
use crate::repositories::associated_company::associated_company_repo::AssociatedCompanyRepository;
use crate::repositories::comment::comment_repo::CommentRepository;
use crate::repositories::company::company_repo::CompanyRepository;
use crate::repositories::employment::employment_repo::EmploymentRepository;
use crate::repositories::event::event_repo::EventRepository;
use crate::repositories::event_staff::event_staff_repo::StaffRepository;
use crate::repositories::repository::DbRepository;
use crate::repositories::task::task_repo::TaskRepository;
use crate::repositories::timesheet::timesheet_repo::TimesheetRepository;
use crate::repositories::user::user_repo::UserRepository;
use crate::{
    handlers::user::get_users_login,
    repositories::assigned_staff::assigned_staff_repo::AssignedStaffRepository,
};
use actix_web::{web, post, HttpResponse, Responder, http};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("HTTP: missing environment variables")
    }
}


#[post("/register")]
async fn register(
    web::Form(form): web::Form<Register>,
    user_repository: web::Data<UserRepository>
) -> HttpResponse {
    // Get admin console token for registration purposes.
    let path = "http://localhost:8080/realms/master/protocol/openid-connect/token";

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
        return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Created().body(body.expect("Should be valid"))
}

#[post("/login")]
async fn login(web::Form(form): web::Form<Login>) -> impl Responder  {
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


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to load .env file");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = Config::default();

    let pool = setup_db_pool().await;
    let arc_pool = Arc::new(pool);
    let user_repository = UserRepository::new(arc_pool.clone());
    let company_repository = CompanyRepository::new(arc_pool.clone());
    let event_repository = EventRepository::new(arc_pool.clone());
    let employment_repository = EmploymentRepository::new(arc_pool.clone());
    let event_staff_repository = StaffRepository::new(arc_pool.clone());
    let task_repository = TaskRepository::new(arc_pool.clone());
    let assigned_staff_repository = AssignedStaffRepository::new(arc_pool.clone());
    let associated_company_repository = AssociatedCompanyRepository::new(arc_pool.clone());
    let timesheet_repository = TimesheetRepository::new(arc_pool.clone());
    let comment_repository = CommentRepository::new(arc_pool.clone());

    let user_repo = web::Data::new(user_repository);
    let company_repo = web::Data::new(company_repository);
    let event_repo = web::Data::new(event_repository);
    let employment_repo = web::Data::new(employment_repository);
    let event_staff_repo = web::Data::new(event_staff_repository);
    let task_repo = web::Data::new(task_repository);
    let assigned_staff_repo = web::Data::new(assigned_staff_repository);
    let associated_company_repo = web::Data::new(associated_company_repository);
    let timesheet_repo = web::Data::new(timesheet_repository);
    let comment_repo = web::Data::new(comment_repository);

    println!("Starting server on http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(user_repo.clone())
            .app_data(company_repo.clone())
            .app_data(event_repo.clone())
            .app_data(employment_repo.clone())
            .app_data(event_staff_repo.clone())
            .app_data(task_repo.clone())
            .app_data(assigned_staff_repo.clone())
            .app_data(associated_company_repo.clone())
            .app_data(timesheet_repo.clone())
            .app_data(comment_repo.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(login)
            .service(register)
            .configure(configure_user_endpoints)
            .configure(configure_company_endpoints)
            .configure(configure_event_endpoints)
            .configure(configure_employment_endpoints)
            .configure(configure_assigned_staff_endpoints)
            .configure(configure_task_endpoints)
            .configure(configure_staff_endpoints)
            .configure(configure_associated_company_endpoints)
            .configure(configure_comment_endpoints)
            .configure(configure_timesheet_endpoints)
            // Temporary
            .service(get_users_login)
            // For serving css and static files overall
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

async fn setup_db_pool() -> Pool<Postgres> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database.")
}
