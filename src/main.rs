mod common;
mod configs;
mod errors;
mod handlers;
mod models;
mod repositories;
mod templates;
mod utils;

use actix_files::Files as ActixFiles;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::{Pool, Postgres};
use std::io::Result;

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
use actix_web::web;

const HOST: &str = "localhost:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("Failed to load .env file");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let pool = setup_db_pool().await;
    let arc_pool = Arc::new(pool);
    // Add repositories here.
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

    println!("Starting server on {}", HOST);
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
    .bind(HOST)?
    .run()
    .await
}

async fn setup_db_pool() -> Pool<Postgres> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database.")
}
