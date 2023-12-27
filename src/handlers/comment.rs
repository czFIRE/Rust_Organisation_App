use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::handlers::common::QueryParams;

#[derive(Deserialize)]
pub struct NewCommentData {
    author_id: Uuid,
    content: String,
}

// Might be smart to remove author_id from this.
#[derive(Deserialize)]
pub struct CommentData {
    author_id: Uuid,
    content: String,
}

#[get("/event/{event_id}/comment")]
pub async fn get_all_event_comments(
    _event_id: web::Path<Uuid>,
    _query: web::Query<QueryParams>,
) -> HttpResponse {
    todo!()
}

#[post("/event/{event_id}/comment")]
pub async fn create_event_comment(
    _event_id: web::Path<Uuid>,
    _new_comment: web::Form<NewCommentData>,
) -> HttpResponse {
    todo!()
}

#[put("/event/{event_id}/comment/{comment_id}")]
pub async fn update_event_comment(
    _event_id: web::Path<Uuid>,
    _comment_id: web::Path<Uuid>,
    _comment_data: web::Form<CommentData>,
) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/comment")]
pub async fn delete_all_event_comments(_event_id: web::Path<Uuid>) -> HttpResponse {
    todo!()
}

#[delete("/event/{event_id}/comment/{comment_id}")]
pub async fn delete_event_comment(
    _event_id: web::Path<Uuid>,
    _comment_id: web::Path<Uuid>,
) -> HttpResponse {
    todo!()
}

#[get("/task/{task_id}/comment")]
pub async fn get_all_task_comments(
    _task_id: web::Path<Uuid>,
    _query: web::Query<QueryParams>,
) -> HttpResponse {
    todo!()
}

#[post("/task/{task_id}/comment")]
pub async fn create_task_comment(
    _task_id: web::Path<Uuid>,
    _new_comment: web::Form<NewCommentData>,
) -> HttpResponse {
    todo!()
}

#[put("/task/{task_id}/comment/{comment_id}")]
pub async fn update_task_comment(
    _task_id: web::Path<Uuid>,
    _comment_id: web::Path<Uuid>,
    _comment_data: web::Form<CommentData>,
) -> HttpResponse {
    todo!()
}

#[delete("/task/{task_id}/comment")]
pub async fn delete_all_task_comments(_task_id: web::Path<Uuid>) -> HttpResponse {
    todo!()
}

#[delete("/task/{task_id}/comment/{comment_id}")]
pub async fn delete_task_comment(
    _task_id: web::Path<Uuid>,
    _comment_id: web::Path<Uuid>,
) -> HttpResponse {
    todo!()
}
