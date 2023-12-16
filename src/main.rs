use crate::repositories::company::CompanyRepository;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::Postgres;
use std::path::Path;
use std::sync::Arc;

mod common;
mod repositories;

const HOST: &str = "localhost:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")?;
    restore_db(&database_url).await?;

    println!("API is running on http://{}/api", HOST);

    let pool = Arc::new(sqlx::PgPool::connect(&database_url).await?);
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api").app_data(web::Data::new(CompanyRepository::new(pool.clone()))),
        )
    })
    .bind(HOST)?
    .run()
    .await?;

    Ok(())
}

async fn restore_db(url: &str) -> Result<()> {
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
