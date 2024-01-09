use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    models::TaskPriority,
    repositories::task::{
        models::{NewTask, TaskData, TaskFilter},
        task_repo::TaskRepository,
    },
    templates::task::{TaskTemplate, TasksTemplate},
};

#[derive(Deserialize)]
pub struct NewEventTaskData {
    creator_id: Uuid,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
}

#[get("/event/{event_id}/task")]
pub async fn get_event_tasks(
    event_id: web::Path<String>,
    query: web::Query<TaskFilter>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() <= 0)
        || (query.offset.is_some() && query.offset.clone().unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");
    let result = task_repo
        .read_all_for_event(parsed_id, query.into_inner())
        .await;

    if let Ok(tasks) = result {
        let task_vector: Vec<TaskTemplate> = tasks.into_iter().map(|task| task.into()).collect();
        let template = TasksTemplate { tasks: task_vector };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
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
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[post("/event/{event_id}/task")]
pub async fn create_task(
    event_id: web::Path<String>,
    new_task: web::Json<NewEventTaskData>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");
    let data = NewTask {
        event_id: parsed_id,
        creator_id: new_task.creator_id,
        title: new_task.title.clone(),
        description: new_task.description.clone(),
        priority: new_task.priority.clone(),
    };
    let result = task_repo.create(data).await;
    if let Ok(task) = result {
        let template: TaskTemplate = task.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

fn is_data_invalid(data: TaskData) -> bool {
    (data.title.is_none() || (data.title.is_some() && data.title.unwrap().is_empty()))
        && (data.description.is_none()
            || (data.description.is_some() && data.description.unwrap().is_empty()))
        && data.finished_at.is_none()
        && data.priority.is_none()
        && data.accepts_staff.is_none()
}

#[patch("/event/task/{task_id}")]
pub async fn update_task(
    task_id: web::Path<String>,
    task_data: web::Json<TaskData>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    if is_data_invalid(task_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let result = task_repo.update(parsed_id, task_data.into_inner()).await;
    if let Ok(task) = result {
        let template: TaskTemplate = task.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.err().expect("Should be error."))
}

#[delete("/event/task/{task_id}")]
pub async fn delete_task(
    task_id: web::Path<String>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");
    let result = task_repo.delete(parsed_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}
