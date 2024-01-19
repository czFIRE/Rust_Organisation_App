use actix_web::web;

use crate::handlers::event_task::{
    create_task, delete_task, get_event_tasks, open_single_task_panel, open_task_creation_panel,
    open_task_edit_panel, open_tasks_panel, update_task, update_task_completion,
};

pub fn configure_task_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(open_tasks_panel)
        .service(open_single_task_panel)
        .service(open_task_creation_panel)
        .service(open_task_edit_panel)
        .service(get_event_tasks)
        .service(create_task)
        .service(update_task)
        .service(update_task_completion)
        .service(delete_task);
}
