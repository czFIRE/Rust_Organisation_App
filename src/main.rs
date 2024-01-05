mod common;
mod errors;
mod handlers;
mod models;
mod repositories;
mod templates;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use organization::initialize::configure_app;
use std::io::Result;

const HOST: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("Starting server on {HOST}");
    HttpServer::new(move || App::new().configure(configure_app))
        .bind(HOST)?
        .run()
        .await
}
