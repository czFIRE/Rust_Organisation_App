use std::str::FromStr;

use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::{EventRole, TaskPriority},
    repositories::{
        assigned_staff::assigned_staff_repo::AssignedStaffRepository,
        event_staff::event_staff_repo::StaffRepository,
        task::{
            models::{NewTask, TaskData, TaskExtended, TaskFilter},
            task_repo::TaskRepository,
        },
    },
    templates::task::{
        EventTask, TaskCreationTemplate, TaskEditTemplate, TaskPanelTemplate, TasksPanelTemplate,
        TasksTemplate,
    },
};

#[derive(Deserialize)]
pub struct NewEventTaskData {
    creator_id: Uuid,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
}

async fn get_tasks_per_event(
    event_id: Uuid,
    query: TaskFilter,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    let result = task_repo.read_all_for_event(event_id, query).await;

    if let Ok(tasks) = result {
        let task_vector: Vec<EventTask> = tasks.into_iter().map(|task| task.into()).collect();
        let template = TasksTemplate { tasks: task_vector };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/event/{event_id}/task")]
pub async fn get_event_tasks(
    event_id: web::Path<String>,
    query: web::Query<TaskFilter>,
    task_repo: web::Data<TaskRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.unwrap() <= 0)
        || (query.offset.is_some() && query.offset.unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");
    get_tasks_per_event(parsed_id, query.into_inner(), task_repo).await
}

// #[get("/event/task/{task_id}")]
// pub async fn get_event_task(
//     task_id: web::Path<String>,
//     task_repo: web::Data<TaskRepository>,
// ) -> HttpResponse {
//     let id_parse = Uuid::from_str(task_id.into_inner().as_str());
//     if id_parse.is_err() {
//         return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
//     }
//     let parsed_id = id_parse.expect("Should be valid.");

//     let result = task_repo.read_one(parsed_id).await;
//     if let Ok(task) = result {
//         let template: TaskTemplate = task.into();
//         let body = template.render();
//         if body.is_err() {
//             return HttpResponse::InternalServerError()
//                 .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
//         }
//         return HttpResponse::Ok()
//             .content_type("text/html")
//             .body(body.expect("Should be valid now."));
//     }

//     handle_database_error(result.expect_err("Should be error."))
// }

#[post("/event/{event_id}/task")]
pub async fn create_task(
    event_id: web::Path<String>,
    new_task: web::Json<NewEventTaskData>,
    task_repo: web::Data<TaskRepository>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    if (new_task.title.is_empty())
        || (new_task.description.is_some() && new_task.description.clone().unwrap().is_empty())
    {
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
        return open_task_panel(task.creator_id, task, assigned_repo).await;
    }
    handle_database_error(result.expect_err("Should be error."))
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
    assigned_repo: web::Data<AssignedStaffRepository>,
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
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }
    let task = result.expect("Should be valid.");

    open_task_panel(task.creator_id, task, assigned_repo).await
}

#[patch("/event/task/{task_id}/completion")]
pub async fn update_task_completion(
    task_id: web::Path<String>,
    task_repo: web::Data<TaskRepository>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let task_data = TaskData {
        title: None,
        finished_at: Some(Utc::now().naive_local()),
        description: None,
        priority: None,
        accepts_staff: None,
    };

    let result = task_repo.update(parsed_id, task_data).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }
    let task = result.expect("Should be valid.");

    open_task_panel(task.creator_id, task, assigned_repo).await
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

    let query = TaskFilter {
        limit: None,
        offset: None,
    };

    get_tasks_per_event(parsed_id, query, task_repo).await
}

/* It's difficult to decide whether this should be in event-staff
 * or event_task. Ultimately it's an operation that bridges staff
 * interaction with tasks, so we put it here.
 */
#[get("/event/staff/{staff_id}/tasks-panel")]
pub async fn open_tasks_panel(
    staff_id: web::Path<String>,
    staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let staff_res = staff_repo.read_one(parsed_id).await;
    if staff_res.is_err() {
        return handle_database_error(staff_res.expect_err("Should be an error."));
    }

    let template = TasksPanelTemplate {
        requester: staff_res.expect("Should be valid.").into(),
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid."))
}

#[get("/event/staff/{staff_id}/task-creation")]
pub async fn open_task_creation_panel(
    staff_id: web::Path<String>,
    staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let parsed_id = id_parse.expect("Should be valid.");

    let staff_res = staff_repo.read_one(parsed_id).await;
    if staff_res.is_err() {
        return handle_database_error(staff_res.expect_err("Should be an error."));
    }

    let staff = staff_res.expect("Should be valid.");
    if staff.role != EventRole::Organizer {
        return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
    }

    let template = TaskCreationTemplate {
        creator_id: staff.id,
        event_id: staff.event_id,
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid here"))
}

async fn open_task_panel(
    staff_id: Uuid,
    task: TaskExtended,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let result = assigned_repo.read_one(task.task_id, staff_id).await;
    if result.is_err() {
        let error = result.expect_err("Should be an error.");
        return match error {
            sqlx::Error::RowNotFound => {
                let template = TaskPanelTemplate {
                    requester_id: staff_id,
                    assigned_staff: None,
                    task: task.into(),
                };
                let body = template.render();
                if body.is_err() {
                    return HttpResponse::InternalServerError()
                        .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
                }
                HttpResponse::Ok().body(body.expect("Should be valid."))
            }
            _ => handle_database_error(error),
        };
    }

    let template = TaskPanelTemplate {
        requester_id: staff_id,
        assigned_staff: Some(result.expect("Should be valid").into()),
        task: task.into(),
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid."))
}

#[get("/event/staff/{staff_id}/task/{task_id}")]
pub async fn open_single_task_panel(
    path: web::Path<(String, String)>,
    task_repo: web::Data<TaskRepository>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (staff_id, task_id) = parsed_ids.unwrap();

    let result = task_repo.read_one(task_id).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }

    let task = result.expect("Should be okay now.");
    open_task_panel(staff_id, task, assigned_repo).await
}

#[get("/event/staff/{staff_id}/task-edit/{task_id}")]
pub async fn open_task_edit_panel(
    path: web::Path<(String, String)>,
    task_repo: web::Data<TaskRepository>,
    assigned_repo: web::Data<AssignedStaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (staff_id, task_id) = parsed_ids.unwrap();

    let task_res = task_repo.read_one(task_id).await;
    if task_res.is_err() {
        return handle_database_error(task_res.expect_err("Should be an error."));
    }

    let task = task_res.expect("Should be okay.");

    let staff_res = assigned_repo.read_one(task_id, staff_id).await;
    if staff_res.is_err() {
        return handle_database_error(staff_res.expect_err("Should be an error."));
    }

    let staff = staff_res.expect("Should be okay.");
    if staff.task_id != task_id || staff.staff.id != task.creator_id {
        return HttpResponse::Forbidden().body(parse_error(http::StatusCode::FORBIDDEN));
    }

    let template = TaskEditTemplate {
        editor_id: staff.staff.id,
        task: task.into(),
    };
    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }
    HttpResponse::Ok().body(body.expect("Should be valid."))
}
