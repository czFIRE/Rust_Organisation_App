use actix_web::web;

use crate::handlers::employment::{
    create_employment, delete_employment, get_employment, get_employments_per_user,
    get_subordinates, toggle_employment_create, toggle_employment_edit, update_employment,
};

pub fn configure_employment_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_employment)
        .service(get_employments_per_user)
        .service(get_subordinates)
        .service(create_employment)
        .service(update_employment)
        .service(delete_employment)
        .service(toggle_employment_edit)
        .service(toggle_employment_create);
}
