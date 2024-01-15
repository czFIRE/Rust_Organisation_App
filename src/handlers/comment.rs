use std::str::FromStr;

use crate::{
    errors::handle_database_error,
    repositories::comment::models::{CommentData, NewComment},
    templates::comment::CommentTemplate,
};
use actix_web::{delete, get, http, post, put, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::parse_error,
    repositories::comment::{comment_repo::CommentRepository, models::CommentFilter},
    templates::comment::CommentsTemplate,
};

#[derive(Deserialize)]
pub struct NewCommentData {
    author_id: Uuid,
    content: String,
}

#[get("/event/{event_id}/comment")]
pub async fn get_all_event_comments(
    event_id: web::Path<String>,
    query: web::Query<CommentFilter>,
    comment_repo: web::Data<CommentRepository>,
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
    let result = comment_repo
        .read_all_per_event(parsed_id, query.into_inner())
        .await;
    if let Ok(comment) = result {
        let comments: Vec<CommentTemplate> =
            comment.into_iter().map(|comment| comment.into()).collect();
        let template = CommentsTemplate { comments };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[post("/event/{event_id}/comment")]
pub async fn create_event_comment(
    event_id: web::Path<String>,
    new_comment: web::Json<NewCommentData>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    if new_comment.content.clone().is_empty() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let data = NewComment {
        author_id: new_comment.author_id,
        event_id: Some(parsed_id),
        task_id: None,
        content: new_comment.content.clone(),
    };

    let result = comment_repo.create(data).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error here."));
    }

    let comments_result = comment_repo
        .read_all_per_event(
            parsed_id,
            CommentFilter {
                limit: None,
                offset: None,
            },
        )
        .await;
    if let Ok(comment) = comments_result {
        let comments: Vec<CommentTemplate> =
            comment.into_iter().map(|comment| comment.into()).collect();
        let template = CommentsTemplate { comments };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be error."))
}

#[get("/task/{task_id}/comment")]
pub async fn get_all_task_comments(
    task_id: web::Path<String>,
    query: web::Query<CommentFilter>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    if (query.limit.is_some() && query.limit.unwrap() <= 0)
        || (query.offset.is_some() && query.offset.unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = comment_repo
        .read_all_per_task(parsed_id, query.into_inner())
        .await;
    if let Ok(comment) = result {
        let comments: Vec<CommentTemplate> =
            comment.into_iter().map(|comment| comment.into()).collect();
        let template = CommentsTemplate { comments };
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[post("/task/{task_id}/comment")]
pub async fn create_task_comment(
    task_id: web::Path<String>,
    new_comment: web::Json<NewCommentData>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(task_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    if new_comment.content.clone().is_empty() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let data = NewComment {
        author_id: new_comment.author_id,
        event_id: None,
        task_id: Some(parsed_id),
        content: new_comment.content.clone(),
    };

    let result = comment_repo.create(data).await;
    if let Ok(comment) = result {
        let template: CommentTemplate = comment.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be error."))
}

#[put("/comment/{comment_id}")]
pub async fn update_comment(
    comment_id: web::Path<String>,
    comment_data: web::Json<CommentData>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    if comment_data.content.clone().is_empty() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(comment_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = comment_repo
        .update(parsed_id, comment_data.into_inner())
        .await;
    if let Ok(comment) = result {
        let template: CommentTemplate = comment.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be error."))
}

#[delete("/comment/{comment_id}")]
pub async fn delete_comment(
    comment_id: web::Path<String>,
    comment_repo: web::Data<CommentRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(comment_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = comment_repo.delete(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}
