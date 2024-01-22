use crate::auth::models::{AccessToken, Login, Register, Token};
use crate::auth::openid::get_token;
use crate::errors::{handle_database_error, parse_error};
use crate::models::UserRole;
use crate::repositories::user::models::NewUser;
use crate::repositories::user::user_repo::UserRepository;
use crate::templates::common::IndexTemplate;
use crate::templates::user::UserTemplate;
use actix_web::cookie::Cookie;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use askama::Template;
use reqwest::{Client, StatusCode};

use serde_json::json;

use actix_web::{http, post, web, HttpResponse};

fn build_path(suffix: &str) -> Result<String, ()> {
    let docker_env = std::env::var("DOCKER");

    let port: i32 = if docker_env.is_err() || docker_env.clone().unwrap() == "0" {
        let tmp = std::env::var("KC_PORT");
        if tmp.is_err() {
            return Err(());
        }
        Some(tmp.unwrap().parse::<i32>().expect("Should be a number."))
    } else {
        Some(8080)
    }
    .expect("Should be set and be a number when using localhost development.");

    let host = if docker_env.is_err() || docker_env.clone().unwrap() == "0" {
        "localhost"
    } else {
        "keycloak"
    };

    log::error!("Host is {}", host);
    log::error!("Port is {}", port.clone());

    let mut path = "http://".to_string();
    path.push_str(host);
    path.push(':');
    path.push_str(port.to_string().as_str());
    path.push_str(suffix);

    log::error!("Path is {}", path.clone());

    Ok(path)
}

#[post("/auth/register")]
pub async fn register(
    web::Form(form): web::Form<Register>,
    user_repository: web::Data<UserRepository>,
) -> HttpResponse {
    let path_res = build_path("/realms/master/protocol/openid-connect/token");
    println!("REE 1");
    if path_res.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.");
    }
    println!("REE 2");
    let path = path_res.expect("Should be some.");
    log::error!("Path is {}", path.clone());
    // let path = "http://localhost:9090/realms/master/protocol/openid-connect/token";
    let payload = json!({
        "username": std::env::var("KEYCLOAK_ADMIN").expect("Should be set"),
        "password": std::env::var("KEYCLOAK_ADMIN_PASSWORD").expect("Should be set"),
        "grant_type": "password",
        "client_id": std::env::var("KEYCLOAK_REG_CLIENT").expect("Should be set"),
    });

    let result = get_token(&path, payload).await;
    log::error!("Result is {:?}", result);
    if result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    println!("REE 3");
    // We kinda juggle the token around to get the data. This doesn't work yet.
    let token = result.expect("Should be okay.");
    let token_json = serde_json::to_string(&token);
    if token_json.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let token_str = token_json.expect("Should be okay");
    let access_json: Result<AccessToken, serde_json::Error> = serde_json::from_str(&token_str);
    if access_json.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let access = access_json.expect("Should be valid");
    println!("REE 4");
    let path_res = build_path("/admin/realms/Orchestrate/users");
    if path_res.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.");
    }
    println!("REE 5");
    let path = path_res.expect("Should be some.");
    // let path = "http://localhost:9090/admin/realms/Orchestrate/users";

    let payload = json!({
        "firstName": form.first_name,
        "lastName": form.last_name,
        "email": form.email,
        "enabled": "true",
        "username": form.email,
        "credentials": [{
            "temporary": false,
            "type": "password",
            "value": form.password
        }]
    });
    println!("REE 6");
    let payload_str = serde_json::to_string(&payload);
    if payload_str.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let request = Client::new()
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(payload_str.expect("Should be valid."))
        .bearer_auth(access.access_token);
    let response = request.send().await;
    println!("REE 7");
    if response.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    println!("REE 8");
    let response_exp = response.expect("Should be valid here.");
    if response_exp.status() != http::StatusCode::CREATED {
        return HttpResponse::BadRequest().finish();
    }
    println!("REE 9");
    let mut full_name = form.first_name;
    full_name.push(' ');
    full_name.push_str(&form.last_name);

    let user_data = NewUser {
        name: full_name,
        email: form.email,
        birth: form.birth,
        gender: form.gender,
        role: UserRole::User,
    };
    let user_res = user_repository.create(user_data).await;
    println!("REE 10");
    if user_res.is_err() {
        return handle_database_error(user_res.expect_err("Should be an error."));
    }
    let template = IndexTemplate {
        landing_title: "Log in to your new account!".to_string(),
    };
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }
    println!("REE 11");
    HttpResponse::Created().body(body.expect("Should be valid"))
}

#[post("/auth/login")]
pub async fn login(
    web::Form(form): web::Form<Login>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let path_res = build_path("/realms/Orchestrate/protocol/openid-connect/token");
    if path_res.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.");
    }
    let path = path_res.expect("Should be some.");
    // The path variable stores the URL of the authentication server
    // let path = "http://localhost:9090/realms/Orchestrate/protocol/openid-connect/token";

    // The payload variable stores the JSON object with the login credentials and the client information
    let payload = json!({
        "username": form.username,
        "password": form.password,
        "client_id": std::env::var("CLIENT_ID").expect("Should be set"),
        "grant_type": "password"
    });

    let client = reqwest::Client::new();
    let res = client
        .post(path)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .form(&payload)
        .send()
        .await;

    if res.is_err() {
        return HttpResponse::BadRequest().body("Bad request".to_string());
    }

    let result = res.expect("Should be some.");

    let result_status = result.status();

    let user_res = user_repo.read_one_with_email(form.username).await;

    if user_res.is_err() {
        return handle_database_error(user_res.expect_err("Should be an error."));
    }

    let template: UserTemplate = user_res.expect("Should be some.").into();
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError().body("Internal server error.".to_string());
    }
    let token = result.json::<Token>().await;

    if token.is_err() {
        return HttpResponse::InternalServerError().body(format!(
            "Internal server error with token: {}.",
            token.err().unwrap()
        ));
    }

    let token = token.expect("Should be some.");

    let tmp = serde_json::to_string(&token);

    if tmp.is_err() {
        return HttpResponse::InternalServerError()
            .body("Internal server error with token serialization.".to_string());
    }

    let serialized_text = tmp.expect("Should be some.");

    match result_status {
        StatusCode::OK => {
            let cookie = Cookie::build("bearer_token", token.access_token)
                .domain("localhost")
                .path("/")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .insert_header(("Authorization", "Bearer"))
                .body(body.expect("Should be some."))
        }
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body(serialized_text),
        StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body(serialized_text),
        _ => HttpResponse::InternalServerError().body("Internal Server Error.".to_string()),
    }
}
