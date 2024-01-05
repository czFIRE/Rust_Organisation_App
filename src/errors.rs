use actix_web::http;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub fn parse_error(code: http::StatusCode) -> String {
    let response = ErrorResponse {
        error: match code {
            http::StatusCode::BAD_REQUEST => "Bad request".to_string(),
            http::StatusCode::NOT_FOUND => "Not found".to_string(),
            http::StatusCode::FORBIDDEN => "Forbidden".to_string(),
            http::StatusCode::INTERNAL_SERVER_ERROR => "Internal error.".to_string(),
            _ => "Unknown error".to_string(),
        },
    };

    serde_json::to_string(&response).expect("Should be parsed correctly.")
}
