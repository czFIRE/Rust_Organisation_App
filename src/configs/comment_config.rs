use actix_web::web;

use crate::handlers::comment::{
    create_event_comment, create_task_comment, delete_comment, get_comment,
    open_comment_update_mode, open_event_comments_for_user, open_task_comments_for_user,
    update_comment,
};

pub fn configure_comment_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(open_event_comments_for_user)
        .service(create_event_comment)
        .service(open_task_comments_for_user)
        .service(create_task_comment)
        .service(open_comment_update_mode)
        .service(get_comment)
        .service(update_comment)
        .service(delete_comment);
}
