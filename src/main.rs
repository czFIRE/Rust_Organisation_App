mod common;
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

use crate::handlers::company::get_company_edit_mode;
use crate::handlers::employment::toggle_employment_create;
use crate::handlers::employment::toggle_employment_edit;
use crate::handlers::event::switch_event_accepts_staff;
use crate::handlers::event::toggle_event_creation_mode;
use crate::handlers::event::toggle_event_edit_mode;
use crate::handlers::event_staff::initialize_staff_management_panel;
use crate::handlers::event_staff::initialize_staff_panel;
use crate::handlers::timesheet::get_work_day;
use crate::handlers::timesheet::toggle_work_day_edit_mode;
use crate::handlers::timesheet::update_work_day;
use crate::handlers::user::get_users;
use crate::handlers::user::toggle_user_edit;
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

use crate::handlers::{
    assigned_staff::{
        create_assigned_staff, delete_all_rejected_assigned_staff, delete_assigned_staff,
        get_all_assigned_staff, get_assigned_staff, update_assigned_staff,
    },
    associated_company::{
        create_associated_company, delete_associated_company, get_all_associated_companies,
        get_all_associated_companies_per_event_and_user, update_associated_company,
    },
    comment::{
        create_event_comment, create_task_comment, delete_comment, get_all_event_comments,
        get_all_task_comments, update_comment,
    },
    company::{
        create_company, delete_company, get_all_companies, get_company, get_company_avatar,
        remove_company_avatar, update_company, upload_company_avatar,
    },
    employment::{
        create_employment, delete_employment, get_employment, get_employments_per_user,
        get_subordinates, update_employment,
    },
    event::{
        create_event, delete_event, get_event, get_event_avatar, get_events, remove_event_avatar,
        update_event, upload_event_avatar,
    },
    event_staff::{
        create_event_staff, delete_all_rejected_event_staff, delete_event_staff,
        get_all_event_staff, get_event_staff, update_event_staff,
    },
    event_task::{create_task, delete_task, get_event_task, get_event_tasks, update_task},
    index::index,
    timesheet::{
        create_timesheet, get_all_timesheets_for_employment, get_timesheet, reset_timesheet_data,
        update_timesheet,
    },
    user::{
        create_user, delete_user, get_user, get_user_avatar, remove_user_avatar, update_user,
        upload_user_avatar,
    },
};

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
            .service(get_user)
            .service(get_users)
            .service(toggle_user_edit)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_user_avatar)
            .service(upload_user_avatar)
            .service(remove_user_avatar)
            .service(get_company)
            .service(get_all_companies)
            .service(create_company)
            .service(update_company)
            .service(delete_company)
            .service(get_company_edit_mode)
            .service(get_company_avatar)
            .service(upload_company_avatar)
            .service(remove_company_avatar)
            .service(get_events)
            .service(get_event)
            .service(create_event)
            .service(update_event)
            .service(delete_event)
            .service(get_event_avatar)
            .service(upload_event_avatar)
            .service(remove_event_avatar)
            .service(toggle_event_edit_mode)
            .service(toggle_event_creation_mode)
            .service(switch_event_accepts_staff)
            .service(get_employment)
            .service(get_employments_per_user)
            .service(get_subordinates)
            .service(create_employment)
            .service(update_employment)
            .service(delete_employment)
            .service(toggle_employment_edit)
            .service(toggle_employment_create)
            .service(get_all_assigned_staff)
            .service(get_assigned_staff)
            .service(create_assigned_staff)
            .service(update_assigned_staff)
            .service(delete_all_rejected_assigned_staff)
            .service(delete_assigned_staff)
            .service(get_event_tasks)
            .service(get_event_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            .service(get_all_event_staff)
            .service(get_event_staff)
            .service(create_event_staff)
            .service(update_event_staff)
            .service(delete_all_rejected_event_staff)
            .service(delete_event_staff)
            .service(initialize_staff_panel)
            .service(initialize_staff_management_panel)
            .service(get_all_associated_companies)
            .service(get_all_associated_companies_per_event_and_user)
            .service(create_associated_company)
            .service(update_associated_company)
            .service(delete_associated_company)
            .service(get_all_event_comments)
            .service(create_event_comment)
            .service(get_all_task_comments)
            .service(create_task_comment)
            .service(update_comment)
            .service(delete_comment)
            .service(get_all_timesheets_for_employment)
            .service(get_timesheet)
            .service(create_timesheet)
            .service(update_timesheet)
            .service(reset_timesheet_data)
            .service(toggle_work_day_edit_mode)
            .service(update_work_day)
            .service(get_work_day)
            // Temporary
            .service(get_users_login)
            // For serving css
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
