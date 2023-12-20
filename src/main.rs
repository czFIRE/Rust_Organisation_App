use crate::handlers::index::index;
use crate::repositories::company::CompanyRepository;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::Postgres;
use std::path::Path;
use std::sync::Arc;

mod common;
mod handlers;
mod repositories;

const HOST: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    restore_db(&database_url)
        .await
        .expect("Failed to restore database");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let arc_pool = Arc::new(pool);

    println!("API is running on http://{}/api", HOST);

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(web::Data::new(CompanyRepository::new(arc_pool.clone())))
                .service(index),
        )
    })
    .bind(HOST)?
    .run()
    .await
}

async fn restore_db(url: &str) -> anyhow::Result<()> {
    Postgres::drop_database(url).await?;
    println!("Database was successfully dropped");

    Postgres::create_database(url).await?;
    println!("Database was successfully created");

    let pool = sqlx::PgPool::connect(url).await?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    println!("Migrations were successfully applied");

    Ok(())
}
