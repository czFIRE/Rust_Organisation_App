use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse, http};
use askama::Template;
use crate::templates::comment::CommentTemplate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{handlers::common::QueryParams, repositories::comment::{comment_repo::CommentRepository, models::CommentFilter}, errors::parse_error, templates::comment::CommentsTemplate};

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
    event_id: web::Path<String>,
    query: web::Query<CommentFilter>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.clone().unwrap() <= 0)
       || (query.offset.is_some() && query.offset.clone().unwrap() <= 0) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = comment_repo.read_all_per_event(parsed_id, query.into_inner()).await;
    if let Ok(comment) = result {
        let comments: Vec<CommentTemplate> = comment.into_iter().map(|comment| comment.into()).collect();
        let template = CommentsTemplate {
            comments,
        };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok().content_type("text/html").body(body.expect("Should be valid now."));
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

#[post("/event/{event_id}/comment")]
pub async fn create_event_comment(
    _event_id: web::Path<Uuid>,
    _new_comment: web::Form<NewCommentData>,
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

#[put("/comment/{comment_id}")]
pub async fn update_comment(
    _comment_id: web::Path<Uuid>,
    _comment_data: web::Form<CommentData>,
) -> HttpResponse {
    todo!()
}

#[delete("/comment/{comment_id}")]
pub async fn delete_comment(_comment_id: web::Path<Uuid>) -> HttpResponse {
    todo!()
}
