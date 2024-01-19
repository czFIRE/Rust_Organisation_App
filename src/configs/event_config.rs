use actix_web::web;

use crate::handlers::event::{
    create_event, delete_event, get_event, get_events, remove_event_avatar,
    switch_event_accepts_staff, toggle_event_creation_mode, toggle_event_edit_mode, update_event,
    upload_event_avatar,
};

pub fn configure_event_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_events)
        .service(get_event)
        .service(create_event)
        .service(update_event)
        .service(delete_event)
        .service(upload_event_avatar)
        .service(remove_event_avatar)
        .service(toggle_event_edit_mode)
        .service(toggle_event_creation_mode)
        .service(switch_event_accepts_staff);
}
