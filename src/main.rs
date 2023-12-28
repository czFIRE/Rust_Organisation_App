use crate::handlers::index::index;
use crate::repositories::assigned_staff::assigned_staff_repo::AssignedStaffRepository;
use crate::repositories::comment::comment_repo::CommentRepository;
use crate::repositories::company::company_repo::CompanyRepository;
use crate::repositories::employment::employment_repo::EmploymentRepository;
use crate::repositories::event::event_repo::EventRepository;
use crate::repositories::event_staff::event_staff_repo::StaffRepository;
use crate::repositories::repository::DbRepository;
use crate::repositories::task::task_repo::TaskRepository;
use crate::repositories::user::user_repo::UserRepository;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io::Result;
use std::sync::Arc;

mod common;
mod handlers;
mod models;
mod repositories;
mod templates;

const HOST: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let arc_pool = Arc::new(pool);

    let company_repo = CompanyRepository::new(arc_pool.clone());
    let user_repo = UserRepository::new(arc_pool.clone());
    let event_repo = EventRepository::new(arc_pool.clone());
    let task_repo = TaskRepository::new(arc_pool.clone());
    let event_staff_repo = StaffRepository::new(arc_pool.clone());
    let employment_repo = EmploymentRepository::new(arc_pool.clone());
    let comment_repo = CommentRepository::new(arc_pool.clone());
    let assigned_staff_repo = AssignedStaffRepository::new(arc_pool.clone());

    println!("API is running on http://{}/api", HOST);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(company_repo.clone()))
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(event_repo.clone()))
            .app_data(web::Data::new(task_repo.clone()))
            .app_data(web::Data::new(event_staff_repo.clone()))
            .app_data(web::Data::new(employment_repo.clone()))
            .app_data(web::Data::new(comment_repo.clone()))
            .app_data(web::Data::new(assigned_staff_repo.clone()))
            .service(index)
    })
    .bind(HOST)?
    .run()
    .await
}
