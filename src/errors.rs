use actix_web::{http, HttpResponse};

pub fn parse_error(code: http::StatusCode) -> String {
    match code {
        http::StatusCode::BAD_REQUEST => "Bad request".to_string(),
        http::StatusCode::NOT_FOUND => "Not found".to_string(),
        http::StatusCode::FORBIDDEN => "Forbidden".to_string(),
        http::StatusCode::INTERNAL_SERVER_ERROR => "Internal error.".to_string(),
        _ => "Unknown error".to_string(),
    }
}

pub fn handle_database_error(error: sqlx::Error) -> HttpResponse {
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
