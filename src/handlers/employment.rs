use std::str::FromStr;

use crate::{
    errors::handle_database_error,
    handlers::common::{extract_path_triple_ids, extract_path_tuple_ids},
    models::{EmployeeLevel, EmploymentContract},
    repositories::employment::models::{EmploymentData, NewEmployment},
    templates::employment::{
        EmploymentCreateTemplate, EmploymentEditTemplate, EmploymentLite, EmploymentTemplate,
        SubordinatesTemplate,
    },
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    repositories::employment::{employment_repo::EmploymentRepository, models::EmploymentFilter},
    templates::employment::EmploymentsTemplate,
};

#[derive(Clone, Debug, Deserialize)]
pub struct EmploymentUpdateData {
    pub editor_id: Uuid,
    pub manager_id: Option<Uuid>,
    pub hourly_wage: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub employment_type: Option<EmploymentContract>,
    pub level: Option<EmployeeLevel>,
}

#[get("/user/{user_id}/employment")]
pub async fn get_employments_per_user(
    user_id: web::Path<String>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();
    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body("Incorrect query parameters.".to_string());
    }

    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format".to_string());
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = employment_repo
        .read_all_for_user(parsed_id, query_params)
        .await;

    if let Ok(employments) = result {
        let employment_vec: Vec<EmploymentLite> = employments
            .into_iter()
            .map(|employment| employment.into())
            .collect();
        let template = EmploymentsTemplate {
            employments: employment_vec,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be error."))
}

async fn get_full_employment(
    user_id: Uuid,
    company_id: Uuid,
    employment_repo: web::Data<EmploymentRepository>,
    is_created: bool,
) -> HttpResponse {
    let result = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = result {
        let template: EmploymentTemplate = employment.into();

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error".to_string());
        }

        return if is_created {
            HttpResponse::Created()
                .content_type("text/html")
                .body(body.expect("Should be valid now."))
        } else {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(body.expect("Should be valid now."))
        };
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/user/{user_id}/employment/{company_id}")]
pub async fn get_employment(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    get_full_employment(user_id, company_id, employment_repo, false).await
}

#[get("/user/{user_id}/employment/{company_id}/subordinates")]
pub async fn get_subordinates(
    path: web::Path<(String, String)>,
    params: web::Query<EmploymentFilter>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body("Incorrect query parameters.".to_string());
    }

    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (user_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo
        .read_subordinates(user_id, company_id, query_params)
        .await;

    if let Ok(subordinates) = result {
        let template = SubordinatesTemplate {
            user_id,
            subordinates,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[post("/employment")]
pub async fn create_employment(
    new_employment: web::Json<NewEmployment>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    println!("Open1");
    let company_id = new_employment.company_id;

    let result = employment_repo.create(new_employment.into_inner()).await;
    println!("Open2");
    if let Err(error) = result {
        return handle_database_error(error);
    }
    println!("Open3");
    let employee = result.expect("Should be valid");

    // We don't want to show the manager the employee's view, so we re-render their view.
    if employee.manager_id.is_some() {
        println!("Open4");
        return get_full_employment(
            employee.manager_id.expect("Should be some"),
            company_id,
            employment_repo,
            true,
        )
        .await;
    }
    println!("Open5");
    // This is for the case when the first employee is created. We don't want to redirect the admin to them.
    HttpResponse::NoContent().finish()
}

#[get("/user/{user_id}/employment/{company_id}/mode/{editor_id}")]
pub async fn toggle_employment_edit(
    path: web::Path<(String, String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_triple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (user_id, company_id, editor_id) = parsed_ids.unwrap();
    let result = employment_repo.read_one(user_id, company_id).await;
    if let Ok(employment) = result {
        // Only the direct manager may edit an employee.
        if employment.manager.is_none()
            || employment.manager.is_some() && employment.manager.unwrap().id != editor_id
        {
            return HttpResponse::Forbidden()
                .body("Not the manager for this employee.".to_string());
        }
        let template: EmploymentEditTemplate = EmploymentEditTemplate {
            editor_id,
            user_id: employment.user_id,
            company_id: employment.company.id,
            employment_type: employment.employment_type,
            hourly_wage: employment.hourly_wage,
            level: employment.level,
            description: employment.description,
            start_date: employment.start_date,
            end_date: employment.end_date,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/user/{user_id}/employment/{company_id}/creation-mode")]
pub async fn toggle_employment_create(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (creator_id, company_id) = parsed_ids.unwrap();
    let result = employment_repo.read_one(creator_id, company_id).await;
    if let Ok(employment) = result {
        // Only a manager or a company admin may create new employments
        if employment.level == EmployeeLevel::Basic {
            return HttpResponse::Forbidden()
                .body("Your employee level is unable to create new employments.".to_string());
        }
        let template: EmploymentCreateTemplate = EmploymentCreateTemplate {
            company_id,
            creator_id,
            creator_level: employment.level,
        };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn is_data_invalid(data: EmploymentUpdateData) -> Option<String> {
    if data.manager_id.is_none()
        && data.hourly_wage.is_none()
        && data.start_date.is_none()
        && data.end_date.is_none()
        && data.description.is_none()
        && data.employment_type.is_none()
        && data.level.is_none()
        && data.start_date.is_none()
        && data.end_date.is_none()
    {
        return Some("No data provided.".to_string());
    }

    if data.start_date.is_some()
        && data.end_date.is_some()
        && data.start_date.unwrap() > data.end_date.unwrap()
    {
        return Some("Start date can't exceed end date.".to_string());
    }

    if data.hourly_wage.is_some() && data.hourly_wage.expect("Should be some.") <= 0.0 {
        return Some("Hourly wage can't be 0 or less.".to_string());
    }

    None
}

#[patch("/user/{user_id}/employment/{company_id}")]
pub async fn update_employment(
    path: web::Path<(String, String)>,
    employment_data: web::Json<EmploymentUpdateData>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let validation_err = is_data_invalid(employment_data.clone());

    if validation_err.is_some() {
        return HttpResponse::BadRequest().body(validation_err.expect("Should be some"));
    }
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (user_id, company_id) = parsed_ids.unwrap();

    // We have to compare these dates against old dates.
    if employment_data.start_date.is_some() || employment_data.end_date.is_some() {
        let current_employment = employment_repo.read_one(user_id, company_id).await;

        if current_employment.is_err() {
            return handle_database_error(current_employment.expect_err("Should be error."));
        }

        let current = current_employment.expect("Should be valid now.");

        if employment_data.start_date.is_some()
            && employment_data.start_date.unwrap() > current.end_date
        {
            return HttpResponse::BadRequest()
                .body("New start date can't be later than the current end date.".to_string());
        }

        if employment_data.end_date.is_some()
            && employment_data.end_date.unwrap() < current.start_date
        {
            return HttpResponse::BadRequest()
                .body("New end date can't be earlier than the current start date.".to_string());
        }
    }

    let data = EmploymentData {
        manager_id: employment_data.manager_id,
        hourly_wage: employment_data.hourly_wage,
        start_date: employment_data.start_date,
        end_date: employment_data.end_date,
        description: employment_data.description.clone(),
        employment_type: employment_data.employment_type.clone(),
        level: employment_data.level.clone(),
    };

    let result = employment_repo.update(user_id, company_id, data).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    // This isn't very pleasant, but it is what it is. Maybe fix later.
    // Editor id because we don't want to render the employee's view.
    get_full_employment(
        employment_data.editor_id,
        company_id,
        employment_repo,
        false,
    )
    .await
}

#[delete("/user/{user_id}/employment/{company_id}")]
pub async fn delete_employment(
    path: web::Path<(String, String)>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (user_id, company_id) = parsed_ids.unwrap();

    let result = employment_repo.delete(user_id, company_id).await;
    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::Ok().finish()
}
