use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewEventData {
    name: String,
    description: Option<String>,
    website: Option<String>,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct EventData {
    name: Option<String>,
    description: Option<String>,
    website: Option<String>,
    start_date: Option<chrono::DateTime<Utc>>,
    end_date: Option<chrono::DateTime<Utc>>,
    accepts_staff: Option<bool>,
}

#[get("/event")]
pub async fn get_events() -> HttpResponse {
    todo!()
}

#[get("/event/{event_id}")]
pub async fn get_event(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[post("/event")]
pub async fn create_event(_new_event: web::Form<NewEventData>) -> HttpResponse {
    todo!()
}

#[patch("/event/{event_id}")]
pub async fn update_event(
    _id: web::Path<String>,
    _event_data: web::Form<EventData>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}")]
pub async fn delete_event(_id: web::Path<String>) -> HttpResponse {
    todo!()
}

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
