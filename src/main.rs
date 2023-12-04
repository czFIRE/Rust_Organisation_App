use dotenv::dotenv;
use redis::{Client, ConnectionLike};
use sqlx::{postgres::PgConnection, Connection};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let redis_url = dotenv::var("REDIS_URL").unwrap();

    let connection = PgConnection::connect(&database_url).await;
    match connection {
        Ok(_) => {
            println!("Successfully connected to postgres");
        }
        Err(_) => {
            println!("Failed to connect to postgres");
        }
    }

    let mut client = Client::open(redis_url).unwrap();
    match client.check_connection() {
        true => {
            println!("Successfully connected to redis");
        }
        false => {
            println!("Failed to connect to redis");
        }
    }
}
