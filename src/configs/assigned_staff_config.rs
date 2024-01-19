use actix_web::web;

use crate::handlers::assigned_staff::{
    create_assigned_staff, delete_all_rejected_assigned_staff, delete_assigned_staff,
    get_all_assigned_staff, get_assigned_staff, initialize_assigned_staff_management_panel,
    update_assigned_staff,
};

pub fn configure_assigned_staff_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_all_assigned_staff)
        .service(get_assigned_staff)
        .service(create_assigned_staff)
        .service(update_assigned_staff)
        .service(delete_all_rejected_assigned_staff)
        .service(delete_assigned_staff)
        .service(initialize_assigned_staff_management_panel);
}
