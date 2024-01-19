use actix_web::web;

use crate::handlers::company::{
    create_company, delete_company, get_all_companies, get_company, get_company_edit_mode,
    get_company_information, remove_company_avatar, update_company, upload_company_avatar,
};

pub fn configure_company_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_company)
        .service(get_all_companies)
        .service(get_company_information)
        .service(create_company)
        .service(update_company)
        .service(delete_company)
        .service(get_company_edit_mode)
        .service(upload_company_avatar)
        .service(remove_company_avatar);
}
