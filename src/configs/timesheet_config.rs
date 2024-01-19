use actix_web::web;

use crate::handlers::timesheet::{
    create_timesheet, get_all_timesheets_for_employment, get_expected_wage_calculation,
    get_timesheet, get_timesheets_for_review, get_work_day, open_sheet_submit_page,
    open_timesheet_for_review, reset_timesheet_data, toggle_work_day_edit_mode, update_timesheet,
    update_work_day,
};

pub fn configure_timesheet_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(get_all_timesheets_for_employment)
        .service(get_timesheet)
        .service(create_timesheet)
        .service(update_timesheet)
        .service(reset_timesheet_data)
        .service(toggle_work_day_edit_mode)
        .service(update_work_day)
        .service(get_work_day)
        .service(open_timesheet_for_review)
        .service(get_timesheets_for_review)
        .service(get_expected_wage_calculation)
        .service(open_sheet_submit_page);
}
