use crate::handlers::index::index;
use crate::repositories::company::CompanyRepository;
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

    println!("API is running on http://{}/api", HOST);

    HttpServer::new(move || {
        App::new().route("/api", web::get().to(index)).service(
            web::scope("/api").app_data(web::Data::new(CompanyRepository::new(arc_pool.clone()))),
        )
    })
    .bind(HOST)?
    .run()
    .await
}
