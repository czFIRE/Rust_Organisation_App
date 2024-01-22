use actix_web::{get, http, HttpResponse};
use askama::Template;

use crate::{
    errors::parse_error,
    templates::common::{IndexTemplate, LoginTemplate, RegistrationTemplate},
};

#[get("/")]
pub async fn index() -> HttpResponse {
    let template = IndexTemplate {
        landing_title: "Organize events with us!".to_string(),
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.expect("Should be valid now."))
}

#[get("/registration")]
pub async fn registration_page() -> HttpResponse {
    let template = RegistrationTemplate {};
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.");
    }
    HttpResponse::Ok().body(body.expect("Should be some."))
}

#[get("/login")]
pub async fn login_page() -> HttpResponse {
    let template = LoginTemplate {};
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.");
    }
    HttpResponse::Ok().body(body.expect("Should be some."))
}
