mod auth;
mod common;
mod configs;
mod errors;
mod handlers;
mod models;
mod repositories;
mod templates;
mod utils;

use crate::auth::models::CookieAuthError;
use crate::handlers::auth::{login, register};
use actix_web::dev::Service;
use actix_web::http::header::HeaderValue;
use actix_web::middleware::ErrorHandlers;
use log::trace;
use reqwest::header;

use actix_files::Files as ActixFiles;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::{Pool, Postgres};

use std::sync::Arc;

use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};
use actix_web_httpauth::extractors::AuthenticationError;

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

use crate::handlers::index::{index, registration_page, login_page};
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
use actix_web::{web, HttpResponse, ResponseError};
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to load .env file");
    std::env::set_var("RUST_LOG", "debug,actix_web_middleware_keycloak_auth=trace");
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
        // let cookie_transform = CookieTransform::new_transform()
        let keycloak_auth = KeycloakAuth::default_with_pk(
            DecodingKey::from_rsa_pem(std::fs::read_to_string(".cert.pem").unwrap().as_bytes())
                .expect("Failed to read .cert.pem"),
        );

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
            .service(registration_page)
            .service(login_page)
            .service(login)
            .service(register)
            .service(
                web::scope("/protected")
                    .wrap(keycloak_auth)
                    .wrap_fn(|mut req, service| {
                        trace!("Initialize Cookie Transform Middleware.");
                        let cookie_val = req.cookie("bearer_token").map(|cookie| cookie.value().to_owned()).ok_or_else(|| HttpResponse::Unauthorized().finish());
                        // if cookie_val.is_err() {
                        //     return CookieAuthError{ message: "Failed to parse cookie.".to_string() };
                        // }
                        let cookie = cookie_val.expect("Should be valid");
                        let auth_header_value = format!("Bearer {}", cookie);
                        let header_value = HeaderValue::from_str(auth_header_value.as_str());
                        // if header_value.is_err() {
                        //     return HttpResponse::InternalServerError().finish();
                        // }
                        req.headers_mut().insert(header::AUTHORIZATION, header_value.expect("Should be some."));
                        trace!("Initialize Cookie Transform Middleware.");
                        service.call(req)

                    })
                    .configure(configure_user_endpoints)
                    .configure(configure_company_endpoints)
                    .configure(configure_event_endpoints)
                    .configure(configure_employment_endpoints)
                    .configure(configure_assigned_staff_endpoints)
                    .configure(configure_task_endpoints)
                    .configure(configure_staff_endpoints)
                    .configure(configure_associated_company_endpoints)
                    .configure(configure_comment_endpoints)
                    .configure(configure_timesheet_endpoints),
            )
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
