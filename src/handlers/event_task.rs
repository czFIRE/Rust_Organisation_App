use std::str::FromStr;

use actix_web::{delete, get, patch, post, web, HttpResponse, http};
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::TaskPriority, repositories::task::{task_repo::TaskRepository, models::TaskFilter}, errors::parse_error, templates::task::{TaskTemplate, TasksTemplate}};

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
pub async fn get_event_tasks(event_id: web::Path<String>, query: web::Query<TaskFilter>, task_repo: web::Data<TaskRepository>) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() <= 0)
        || (query.offset.is_some() && query.offset.clone().unwrap() <= 0) {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");
    let result = task_repo.read_all_for_event(parsed_id, query.into_inner()).await;

    if let Ok(tasks) = result {
        let task_vector: Vec<TaskTemplate> = tasks.into_iter().map(|task| task.into()).collect(); 
        let template = TasksTemplate {
            tasks: task_vector,
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        return HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."));
    }
    
    HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
}

#[get("/event/task/{task_id}")]
pub async fn get_event_task(
    task_id: web::Path<String>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let result = task_repo.read_one(parsed_id).await;
    if let Ok(task) = result {
        let template: TaskTemplate = task.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."));
    }
    
    let error = result.err().expect("Should be error.");
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND))
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation() || err.is_foreign_key_violation() {
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
