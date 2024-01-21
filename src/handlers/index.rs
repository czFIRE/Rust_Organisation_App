use actix_web::{get, http, HttpResponse};
use askama::Template;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    templates::common::{IndexTemplate, RegistrationTemplate, LoginTemplate},
};

use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth, KeycloakClaims};

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

// http://localhost:9090/realms/Orchestrate/account/#/
// https://stackoverflow.com/questions/61858077/keycloak-realm-login-page-is-not-appearing

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ClaimsWithEmail {
    // Standard claims, we choose the way they should be deserialized
    sub: Uuid,
    #[serde(with = "ts_seconds")]
    exp: DateTime<Utc>,
    // Custom claims
    company_id: u32,
}

#[get("/private")]
pub async fn protected(claims: KeycloakClaims<ClaimsWithEmail>) -> HttpResponse {
    HttpResponse::Ok().body("You are in a protected area.")
}
