use actix_web::web::{self, scope};

use crate::handlers::user::{
    create_user, delete_user, get_user, get_users, open_admin_panel, remove_user_avatar,
    toggle_user_edit, update_user, upload_user_avatar,
};

// ToDo: Move open admin panel to a separate config
pub fn configure_user_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_user)
        .service(get_users)
        .service(toggle_user_edit)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
        .service(upload_user_avatar)
        .service(remove_user_avatar)
        .service(open_admin_panel);
}
