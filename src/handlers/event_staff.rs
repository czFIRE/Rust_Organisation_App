use std::str::FromStr;

use crate::repositories::associated_company::associated_company_repo::AssociatedCompanyRepository;
use crate::repositories::timesheet::models::TimesheetCreateData;
use crate::templates::staff::{EventStaffManagementTemplate, StaffRegisterTemplate};
use crate::{
    common::DbResult,
    errors::handle_database_error,
    handlers::common::extract_path_tuple_ids,
    models::{AcceptanceStatus, EventRole},
    repositories::{
        event::event_repo::EventRepository,
        event_staff::models::{NewStaff, StaffData, StaffFilter},
        timesheet::timesheet_repo::TimesheetRepository,
    },
    templates::staff::AllStaffTemplate,
};
use actix_web::{delete, get, http, patch, post, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::{
    errors::parse_error, repositories::event_staff::event_staff_repo::StaffRepository,
    templates::staff::StaffTemplate,
};

async fn read_all_event_staff(
    event_id: Uuid,
    query: StaffFilter,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let result = event_staff_repo.read_all_for_event(event_id, query).await;
    if let Ok(all_staff) = result {
        let staff_vec = all_staff.into_iter().map(|staff| staff.into()).collect();

        let template = AllStaffTemplate { staff: staff_vec };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/event/{event_id}/staff")]
pub async fn get_all_event_staff(
    event_id: web::Path<String>,
    query: web::Query<StaffFilter>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let query_info = query.into_inner();
    if (query_info.limit.is_some() && query_info.limit.unwrap() <= 0)
        || (query_info.offset.is_some() && query_info.offset.unwrap() <= 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    read_all_event_staff(parsed_id, query_info, event_staff_repo).await
}

#[get("/event/staff/{staff_id}")]
pub async fn get_event_staff(
    staff_id: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(staff_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.read_one(parsed_id).await;
    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[post("/event/{event_id}/staff")]
pub async fn create_event_staff(
    event_id: web::Path<String>,
    new_event_staff: web::Json<NewStaff>,
    event_staff_repo: web::Data<StaffRepository>,
    associated_company_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(event_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let company_id = new_event_staff.company_id;
    let associated_company = associated_company_repo
        .read_one(company_id, parsed_id)
        .await;
    // An error here likely means the company is not associated with the event.
    if associated_company.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let result = event_staff_repo
        .create(parsed_id, new_event_staff.into_inner())
        .await;

    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn is_data_invalid(data: StaffData) -> bool {
    (data.role.is_none() && data.status.is_none() && data.decided_by.is_none())
        || (data.status.is_some()
            && data.status.clone().unwrap() != AcceptanceStatus::Pending
            && data.decided_by.is_none())
        || (data.status.is_none() && data.decided_by.is_some())
}

async fn create_timesheet_for_user(
    user_id: Uuid,
    company_id: Uuid,
    event_id: Uuid,
    timesheet_repo: web::Data<TimesheetRepository>,
    event_repo: web::Data<EventRepository>,
) -> DbResult<()> {
    let event = event_repo.read_one(event_id).await?;

    let timesheet_data = TimesheetCreateData {
        start_date: event.start_date,
        end_date: event.end_date,
        user_id,
        company_id,
        event_id,
    };

    timesheet_repo.create(timesheet_data).await?;
    Ok(())
}

#[patch("/event/{event_id}/staff/{staff_id}")]
pub async fn update_event_staff(
    path: web::Path<(String, String)>,
    event_staff_data: web::Json<StaffData>,
    event_staff_repo: web::Data<StaffRepository>,
    timesheet_repo: web::Data<TimesheetRepository>,
    event_repo: web::Data<EventRepository>,
) -> HttpResponse {
    if is_data_invalid(event_staff_data.clone()) {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, staff_id) = parsed_ids.unwrap();

    // Extract the old and new status of the event staff to check if status really changes.
    let current_staff = event_staff_repo.read_one(staff_id).await;
    if current_staff.is_err() {
        return HttpResponse::NotFound().body(parse_error(http::StatusCode::NOT_FOUND));
    }
    let old_staff = current_staff.expect("Should be valid now.");
    let status_change = event_staff_data.status.clone();

    // Make sure the decider is a valid entity in the system.
    if event_staff_data.decided_by.is_some() {
        let decider_id = event_staff_data.decided_by.unwrap();
        let decider = event_staff_repo.read_one(decider_id).await;
        if decider.is_err() {
            // Might specify this error further. But the decider needs to exist in the request, so it's a bad request.
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
        let decider_unwrapped = decider.expect("Should be valid here.");
        // Decider is not from this event. Whoops.
        if decider_unwrapped.event_id != event_id || decider_unwrapped.role != EventRole::Organizer
        {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }
    }

    let result = event_staff_repo
        .update(staff_id, event_staff_data.into_inner())
        .await;

    if let Ok(staff) = result {
        // If a status change occured, and everything else went well, then we
        // need to create the timesheet.
        if status_change.is_some()
            && status_change.unwrap() == AcceptanceStatus::Accepted
            && old_staff.status != AcceptanceStatus::Accepted
        {
            let timesheet_res = create_timesheet_for_user(
                staff.user.id,
                staff.company.id,
                event_id,
                timesheet_repo,
                event_repo,
            )
            .await;
            if timesheet_res.is_err() {
                return handle_database_error(timesheet_res.expect_err("Should be err."));
            }
        }

        // Since changes are performed by the manager, we re-fetch all staff to refresh their view.
        return read_all_event_staff(
            staff.event_id,
            StaffFilter {
                limit: None,
                offset: None,
            },
            event_staff_repo,
        )
        .await;
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[delete("/event/{event_id}/staff")]
pub async fn delete_all_rejected_event_staff(
    path: web::Path<String>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(path.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = event_staff_repo.delete_rejected(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    read_all_event_staff(
        parsed_id,
        StaffFilter {
            limit: None,
            offset: None,
        },
        event_staff_repo,
    )
    .await
}

#[delete("/event/{event_id}/staff/{staff_id}")]
pub async fn delete_event_staff(
    path: web::Path<(String, String)>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let (event_id, staff_id) = parsed_ids.unwrap();
    let result = event_staff_repo.delete(staff_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    read_all_event_staff(
        event_id,
        StaffFilter {
            limit: None,
            offset: None,
        },
        event_staff_repo,
    )
    .await
}

async fn prepare_staff_registration_panel(
    user_id: Uuid,
    event_id: Uuid,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    let result = associated_repo
        .get_all_associated_companies_for_event_and_user(event_id, user_id)
        .await;

    if let Ok(companies) = result {
        let template = StaffRegisterTemplate {
            user_id,
            event_id,
            companies,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid here."));
    }

    let error = result.expect_err("Should be an error here.");
    handle_database_error(error)
}

/*
 * This request serves for serving the event staff
 * panel on the frontend. It's purpose is to check if the user is already staff for the event
 * and decide on what content will be served based on the
 * outcome of that decision.
 */
#[get("/event/{event_id}/staff-panel/{user_id}")]
pub async fn initialize_staff_panel(
    path: web::Path<(String, String)>,
    event_staff_repo: web::Data<StaffRepository>,
    associated_repo: web::Data<AssociatedCompanyRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let (event_id, user_id) = parsed_ids.expect("Should be okay");

    // Try to retrieve the staff. Every user should only have one staff relationship for a given event.
    let result = event_staff_repo
        .read_by_event_and_user_id(event_id, user_id)
        .await;

    // If staff exists, we render the regular staff panel.
    if let Ok(staff) = result {
        let template: StaffTemplate = staff.into();
        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    let error = result.expect_err("Should be error.");

    match error {
        sqlx::Error::RowNotFound => {
            // If staff wasn't found, we render a form for staff creation.
            prepare_staff_registration_panel(user_id, event_id, associated_repo).await
        }
        sqlx::Error::Database(err) => {
            if err.is_check_violation()
                || err.is_foreign_key_violation()
                || err.is_unique_violation()
            {
                HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST))
            } else {
                HttpResponse::InternalServerError()
                    .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
        _ => HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[get("/event/{event_id}/staff/{staff_id}/management")]
pub async fn initialize_staff_management_panel(
    path: web::Path<(String, String)>,
    event_staff_repo: web::Data<StaffRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }
    let (event_id, staff_id) = parsed_ids.expect("Should be okay");

    let result = event_staff_repo.read_one(staff_id).await;
    if let Ok(staff) = result {
        if staff.event_id != event_id
            || staff.role != EventRole::Organizer
            || staff.status != AcceptanceStatus::Accepted
        {
            return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
        }

        let template = EventStaffManagementTemplate {
            requester: staff.into(),
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be an error."))
}
