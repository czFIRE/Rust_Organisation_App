use actix_web::web;

use crate::handlers::associated_company::{
    create_associated_company, delete_associated_company, get_all_associated_companies,
    get_all_associated_companies_per_event_and_user, get_associated_company_edit_form,
    get_editable_associated_companies, get_editable_associated_company,
    open_associated_company_management_panel, update_associated_company,
};

pub fn configure_associated_company_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_all_associated_companies)
        .service(get_all_associated_companies_per_event_and_user)
        .service(create_associated_company)
        .service(update_associated_company)
        .service(delete_associated_company)
        .service(open_associated_company_management_panel)
        .service(get_editable_associated_companies)
        .service(get_editable_associated_company)
        .service(get_associated_company_edit_form);
}
