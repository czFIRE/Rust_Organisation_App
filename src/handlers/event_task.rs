use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::TaskPriority;

#[derive(Deserialize)]
pub struct NewEventTaskData {
    creator_id: Uuid,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
}

#[derive(Deserialize)]
pub struct EventTaskData {
    title: Option<String>,
    description: Option<String>,
    finished_at: Option<chrono::DateTime<Utc>>,
    priority: Option<TaskPriority>,
    accepts_staff: Option<bool>,
}

#[get("/event/{event_id}/task")]
pub async fn get_event_tasks(_event_id: web::Path<String>) -> HttpResponse {
    todo!()
}

#[get("/event/{event_id}/task/{task_id}")]
pub async fn get_event_task(
    _event_id: web::Path<String>,
    _task_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}

#[post("/event/{event_id}/task")]
pub async fn create_task(
    _event_id: web::Path<String>,
    _new_task: web::Form<NewEventTaskData>,
) -> HttpResponse {
    todo!()
}

#[patch("/event/{event_id}/task/{task_id}")]
pub async fn update_task(
    _event_id: web::Path<String>,
    _task_id: web::Path<String>,
    _task_data: web::Form<EventTaskData>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/task/{task_id}")]
pub async fn delete_task(
    _event_id: web::Path<String>,
    _task_id: web::Path<String>,
) -> HttpResponse {
    todo!()
}
