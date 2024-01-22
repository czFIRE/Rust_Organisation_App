use std::str::FromStr;

use crate::{
    repositories::employment::employment_repo::EmploymentRepository,
    templates::company::CompaniesInfoTemplate,
    utils::image_storage::{
        img_manipulation::{remove_image, store_image},
        models::{ImageCategory, UploadForm, DEFAULT_COMPANY_IMAGE, MAX_FILE_SIZE},
    },
};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{handle_database_error, parse_error},
    handlers::common::extract_path_tuple_ids,
    models::EmployeeLevel,
    repositories::company::{
        company_repo::CompanyRepository,
        models::{AddressData, AddressUpdateData, CompanyData, CompanyFilter, NewCompany},
    },
    templates::company::{CompaniesTemplate, CompanyEditTemplate, CompanyLite, CompanyTemplate},
    utils::format_check::check::{check_email_validity, check_phone_validity},
};

#[derive(Deserialize, Clone)]
pub struct NewCompanyData {
    name: String,
    description: Option<String>,
    website: Option<String>,
    crn: String,
    vatin: String,
    country: String,
    region: String,
    city: String,
    street: String,
    number: String,
    postal_code: String,
    phone: String,
    email: String,
    employee_id: Uuid,
}

#[derive(Deserialize, Clone)]
pub struct CompanyUpdateData {
    name: Option<String>,
    description: Option<String>,
    website: Option<String>,
    crn: Option<String>,
    vatin: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    street: Option<String>,
    number: Option<String>,
    postal_code: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

async fn get_many_companies(
    filter: CompanyFilter,
    company_repo: web::Data<CompanyRepository>,
    simple_view: bool,
) -> HttpResponse {
    let result = company_repo.read_all(filter).await;

    if let Ok(companies) = result {
        let lite_companies = companies
            .into_iter()
            .map(|company| CompanyLite {
                id: company.id,
                name: company.name,
                avatar_url: company.avatar_url,
            })
            .collect();

        let body: Result<String, askama::Error> = if !simple_view {
            let template = CompaniesTemplate {
                companies: lite_companies,
            };
            template.render()
        } else {
            let template = CompaniesInfoTemplate {
                companies: lite_companies,
            };
            template.render()
        };

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be okay now."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[get("/company")]
pub async fn get_all_companies(
    params: web::Query<CompanyFilter>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    get_many_companies(query_params, company_repo, false).await
}

/* This exists because the function above is used mainly for the layout
 * of companies in the company tab. This function retrieves a much simpler view
 * used by administrators / event organizers for company-related operations.
*/
#[get("/company-info")]
pub async fn get_company_information(
    params: web::Query<CompanyFilter>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let query_params = params.into_inner();

    if (query_params.limit.is_some() && query_params.limit.unwrap() < 0)
        || (query_params.offset.is_some() && query_params.offset.unwrap() < 0)
    {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    get_many_companies(query_params, company_repo, true).await
}

#[get("/company/{company_id}")]
pub async fn get_company(
    company_id: web::Path<String>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = company_repo.read_one_extended(parsed_id).await;

    if let Ok(company) = result {
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_creation_data(data: NewCompanyData) -> Result<(), String> {
    if data.name.trim().is_empty()
        || data.phone.trim().is_empty()
        || data.email.trim().is_empty()
        || data.crn.trim().is_empty()
        || data.vatin.trim().is_empty()
        || data.region.trim().is_empty()
        || data.country.trim().is_empty()
        || data.city.trim().is_empty()
        || data.postal_code.trim().is_empty()
        || data.street.trim().is_empty()
    {
        return Err("No data provided.".to_string());
    }

    if !check_email_validity(data.email) {
        return Err("Invalid email format.".to_string());
    }

    if !check_phone_validity(data.phone) {
        return Err("Invalid phone number format.".to_string());
    }

    Ok(())
}

#[post("/company")]
pub async fn create_company(
    new_company: web::Json<NewCompanyData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let data = new_company.into_inner();
    let validation_res = validate_creation_data(data.clone());
    if let Err(error_msg) = validation_res {
        return HttpResponse::BadRequest().body(error_msg);
    }

    let company_data = NewCompany {
        name: data.name.clone(),
        description: data.description.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        website: data.website.clone(),
        crn: data.crn.clone(),
        vatin: data.vatin.clone(),
    };

    let address = AddressData {
        country: data.country.clone(),
        region: data.region.clone(),
        city: data.city.clone(),
        postal_code: data.postal_code.clone(),
        street: data.street.clone(),
        street_number: data.number.clone(),
    };

    let result = company_repo
        .create(company_data, address, data.employee_id)
        .await;

    if let Ok(company) = result {
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal server error.".to_string());
        }

        return HttpResponse::Created()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn is_data_empty(company_data: &CompanyUpdateData) -> bool {
    company_data.name.is_none()
        && company_data.description.is_none()
        && company_data.website.is_none()
        && company_data.crn.is_none()
        && company_data.vatin.is_none()
        && company_data.country.is_none()
        && company_data.region.is_none()
        && company_data.city.is_none()
        && company_data.street.is_none()
        && company_data.number.is_none()
        && company_data.postal_code.is_none()
        && company_data.phone.is_none()
        && company_data.email.is_none()
}

/*
 * On the semantics of 'formatless'. Some strings here could definitely
 * have some format imposed upon them, but we ultimately decided that it may
 * be wiser to leave this as it is and not bother with restricting format of
 * things like CRN and VATIN.
 * Similarly, postal codes could vary for different locations.
 */
fn any_formatless_string_empty(company_data: &CompanyUpdateData) -> bool {
    (company_data.name.is_some() && company_data.name.as_ref().unwrap().trim().is_empty())
        || (company_data.description.is_some()
            && company_data.description.as_ref().unwrap().trim().is_empty())
        || (company_data.website.is_some()
            && company_data.website.as_ref().unwrap().trim().is_empty())
        || (company_data.crn.is_some() && company_data.crn.as_ref().unwrap().trim().is_empty())
        || (company_data.vatin.is_some() && company_data.vatin.as_ref().unwrap().trim().is_empty())
        || (company_data.country.is_some()
            && company_data.country.as_ref().unwrap().trim().is_empty())
        || (company_data.region.is_some()
            && company_data.region.as_ref().unwrap().trim().is_empty())
        || (company_data.city.is_some() && company_data.city.as_ref().unwrap().trim().is_empty())
        || (company_data.street.is_some()
            && company_data.street.as_ref().unwrap().trim().is_empty())
        || (company_data.number.is_some()
            && company_data.number.as_ref().unwrap().trim().is_empty())
        || (company_data.postal_code.is_some()
            && company_data.postal_code.as_ref().unwrap().trim().is_empty())
}

fn validate_update_data(company_data: CompanyUpdateData) -> Result<(), String> {
    if is_data_empty(&company_data) {
        return Err("No data provided.".to_string());
    }

    if any_formatless_string_empty(&company_data) {
        return Err("An empty field was found.".to_string());
    }

    if company_data.email.is_some() && !check_email_validity(company_data.email.unwrap()) {
        return Err("Invalid email format.".to_string());
    }

    if company_data.phone.is_some() && !check_phone_validity(company_data.phone.unwrap()) {
        return Err("Invalid phone number format.".to_string());
    }

    Ok(())
}

#[patch("/company/{company_id}")]
pub async fn update_company(
    company_id: web::Path<String>,
    company_data: web::Json<CompanyUpdateData>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let data = company_data.into_inner();

    let validation_res = validate_update_data(data.clone());
    if let Err(error_msg) = validation_res {
        return HttpResponse::BadRequest().body(error_msg);
    }

    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body("Incorrect ID format.".to_string());
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let company_update_data = CompanyData {
        name: data.name.clone(),
        description: data.description.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        avatar_url: None, //ToDo: Include avatar in company update?
        website: data.website.clone(),
        crn: data.crn.clone(),
        vatin: data.vatin.clone(),
    };

    let address_update_data = AddressUpdateData {
        country: data.country.clone(),
        city: data.city.clone(),
        region: data.region.clone(),
        postal_code: data.postal_code.clone(),
        street: data.street.clone(),
        street_number: data.number.clone(),
    };

    let result = company_repo
        .update(parsed_id, company_update_data, address_update_data)
        .await;

    if let Ok(company) = result {
        let template: CompanyTemplate = company.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[delete("/company/{company_id}")]
pub async fn delete_company(
    company_id: web::Path<String>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = company_repo.delete(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}

#[get("/company/{company_id}/mode/{user_id}")]
pub async fn get_company_edit_mode(
    path: web::Path<(String, String)>,
    company_repo: web::Data<CompanyRepository>,
    employment_repo: web::Data<EmploymentRepository>,
) -> HttpResponse {
    let parsed_ids = extract_path_tuple_ids(path.into_inner());
    if parsed_ids.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID format.".to_string());
    }

    let (company_id, user_id) = parsed_ids.unwrap();
    let employee_res = employment_repo.read_one(user_id, company_id).await;
    if employee_res.is_err() {
        return handle_database_error(employee_res.expect_err("Should be an error."));
    }

    let employee = employee_res.expect("Should be valid now.");

    if employee.level != EmployeeLevel::CompanyAdministrator {
        return HttpResponse::Forbidden().body("Not a Company Administrator.");
    }

    let result = company_repo.read_one_extended(company_id).await;
    if let Ok(company) = result {
        let template: CompanyEditTemplate = CompanyEditTemplate {
            id: company.company_id,
            user_id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            country: company.country,
            region: company.region,
            city: company.city,
            street: company.street,
            postal_code: company.postal_code,
            address_number: company.street_number,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[put("/company/{company_id}/avatar")]
pub async fn upload_company_avatar(
    company_id: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be okay.");

    if form.file.size == 0 || form.file.size > MAX_FILE_SIZE {
        return HttpResponse::BadRequest().body("Incorrect file size. The limit is 10MB.");
    }

    if form.file.content_type.is_none()
        || form
            .file
            .content_type
            .clone()
            .expect("Should be valid")
            .subtype()
            != "jpeg"
    {
        return HttpResponse::BadRequest().body("Invalid file type, only .jpeg is allowed.");
    }

    let image_res = store_image(parsed_id, ImageCategory::Company, form.file);
    if image_res.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error.".to_string());
    }
    let image_path = image_res.expect("Should be valid.");
    let data = CompanyData {
        name: None,
        email: None,
        website: None,
        description: None,
        phone: None,
        crn: None,
        vatin: None,
        avatar_url: Some(image_path),
    };

    let address = AddressUpdateData {
        country: None,
        region: None,
        city: None,
        postal_code: None,
        street: None,
        street_number: None,
    };

    let result = company_repo.update(parsed_id, data, address).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }
    HttpResponse::Ok().body("New image uploaded!")
}

#[delete("/company/{company_id}/avatar")]
pub async fn remove_company_avatar(
    company_id: web::Path<String>,
    company_repo: web::Data<CompanyRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(company_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be okay.");
    if remove_image(parsed_id, ImageCategory::Company).is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error".to_string());
    }

    let data = CompanyData {
        name: None,
        email: None,
        website: None,
        description: None,
        phone: None,
        crn: None,
        vatin: None,
        avatar_url: Some(DEFAULT_COMPANY_IMAGE.to_string()),
    };

    let address = AddressUpdateData {
        country: None,
        region: None,
        city: None,
        postal_code: None,
        street: None,
        street_number: None,
    };

    let result = company_repo.update(parsed_id, data, address).await;
    if result.is_err() {
        return handle_database_error(result.expect_err("Should be an error."));
    }
    HttpResponse::Ok().body("Company image deleted.")
}
