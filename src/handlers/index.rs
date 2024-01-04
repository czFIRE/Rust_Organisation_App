use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Orchestrate. This is just a testing index page.")
}
