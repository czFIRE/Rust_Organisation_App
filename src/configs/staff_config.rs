use actix_web::web;

use crate::handlers::event_staff::{
    create_event_staff, delete_all_rejected_event_staff, delete_event_staff, get_all_event_staff,
    get_event_staff, initialize_staff_management_panel, initialize_staff_panel, update_event_staff,
};

pub fn configure_staff_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_all_event_staff)
        .service(get_event_staff)
        .service(create_event_staff)
        .service(update_event_staff)
        .service(delete_all_rejected_event_staff)
        .service(delete_event_staff)
        .service(initialize_staff_panel)
        .service(initialize_staff_management_panel);
}
