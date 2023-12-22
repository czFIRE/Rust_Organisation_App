/* Note to the person merging: Just ignore this version
 * in favour of the one that is in the master branch.
 * You can add these two lines manually.
 */
mod models;
mod templates;

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
