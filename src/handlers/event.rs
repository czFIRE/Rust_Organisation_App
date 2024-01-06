use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    repositories::event::{
        event_repo::EventRepository,
        models::{EventData, EventFilter, NewEvent},
    },
    templates::event::{EventLiteTemplate, EventTemplate, EventsTemplate},
};

#[get("/event")]
pub async fn get_events(
    params: web::Query<EventFilter>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.clone().unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.clone().unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = event_repo.read_all(query_params).await;

    if let Ok(events) = result {
        let lite_events = events
            .into_iter()
            .map(|event| EventLiteTemplate {
                id: event.id,
                avatar_url: event.avatar_url,
                name: event.name,
                accepts_staff: event.accepts_staff,
                start_date: event.start_date,
                end_date: event.end_date,
            })
            .collect();

        let template = EventsTemplate {
            events: lite_events,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be okay now."));
    }

    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

#[get("/event/{event_id}")]
pub async fn get_event(
    event_id: web::Path<String>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = event_repo.read_one(parsed_id).await;

    if let Ok(event) = result {
        let template = EventTemplate {
            id: event.id,
            avatar_url: event.avatar_url,
            name: event.name,
            description: event
                .description
                .unwrap_or("No description set.".to_string()),
            website: event.website.unwrap_or("No website set.".to_string()),
            accepts_staff: event.accepts_staff,
            start_date: event.start_date,
            end_date: event.end_date,
            created_at: event.created_at,
            edited_at: event.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    let error = result.err().expect("Should be an error");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[post("/event")]
pub async fn create_event(
    new_event: web::Form<NewEvent>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    if (new_event.website.is_some() && new_event.website.as_ref().unwrap().len() == 0)
    || (new_event.description.is_some() && new_event.description.as_ref().unwrap().len() == 0) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = event_repo.create(new_event.into_inner()).await;

    if let Ok(event) = result {
        let template = EventTemplate {
            id: event.id,
            avatar_url: event.avatar_url,
            name: event.name,
            description: event
                .description
                .unwrap_or("No description set.".to_string()),
            website: event.website.unwrap_or("No website set.".to_string()),
            accepts_staff: event.accepts_staff,
            start_date: event.start_date,
            end_date: event.end_date,
            created_at: event.created_at,
            edited_at: event.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    let error = result.err().expect("Should be error.");
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

fn is_update_data_empty(event_data: EventData) -> bool {
    (event_data.name.is_none()
        && event_data.description.is_none()
        && event_data.website.is_none()
        && event_data.start_date.is_none()
        && event_data.end_date.is_none()
        && event_data.avatar_url.is_none())
        || (event_data.avatar_url.is_some() && event_data.avatar_url.unwrap().len() == 0)
        || (event_data.website.is_some() && event_data.website.unwrap().len() == 0)
        || (event_data.description.is_some() && event_data.description.unwrap().len() == 0)

}

#[patch("/event/{event_id}")]
pub async fn update_event(
    event_id: web::Path<String>,
    event_data: web::Form<EventData>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    if is_update_data_empty(event_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_repo.update(parsed_id, event_data.into_inner()).await;

    if let Ok(event) = result {
        let template = EventTemplate {
            id: event.id,
            avatar_url: event.avatar_url,
            name: event.name,
            description: event
                .description
                .unwrap_or("No description set.".to_string()),
            website: event.website.unwrap_or("No website set.".to_string()),
            accepts_staff: event.accepts_staff,
            start_date: event.start_date,
            end_date: event.end_date,
            created_at: event.created_at,
            edited_at: event.edited_at,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    let error = result.err().expect("Should be error.");
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

#[delete("/event/{event_id}")]
pub async fn delete_event(
    event_id: web::Path<String>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_repo.delete(parsed_id).await;

    if let Err(error) = result {
        return match error {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
            }
            _ => HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
        };
    }

    HttpResponse::NoContent().finish()
}

//TODO: Once file store/load is done.
#[get("/event/{event_id}/avatar")]
pub async fn get_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[put("/event/{event_id}/avatar")]
pub async fn upload_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/avatar")]
pub async fn remove_event_avatar(_id: web::Path<String>) -> HttpResponse {
    todo!()
}
