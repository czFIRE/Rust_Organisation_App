use dotenv::dotenv;
use redis::{Client, ConnectionLike};
use sqlx::migrate::Migrator;

use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")?;
    let redis_url = dotenv::var("REDIS_URL")?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    let pool = sqlx::PgPool::connect(&database_url).await?;

    migrator.run(&pool).await?;
    println!("Successfully ran migrations \\o/");

    let mut client = Client::open(redis_url)?;
    match client.check_connection() {
        true => {
            println!("Successfully connected to redis");
        }
        false => {
            println!("Failed to connect to redis");
        }
    }

    Ok(())
}
