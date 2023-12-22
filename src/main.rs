mod templates;
mod models;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::Postgres;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")?;

    Postgres::drop_database(&database_url).await?;
    Postgres::create_database(&database_url).await?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    println!("All migrations were successfully applied \\o/");

    Ok(())
}
