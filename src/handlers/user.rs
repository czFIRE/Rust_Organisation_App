use std::str::FromStr;

use crate::{
    errors::{handle_database_error, parse_error},
    repositories::user::models::{NewUser, UserData, UsersQuery},
    templates::user::{AdminTemplate, UserEditTemplate, UserInfo, UserInfoTemplate, UserTemplate},
    utils::{
        format_check::check::check_email_validity,
        image_storage::{
            img_manipulation::{remove_image, store_image},
            models::{ImageCategory, UploadForm, DEFAULT_USER_IMAGE, MAX_FILE_SIZE},
        },
    },
};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, http, patch, post, put, web, HttpResponse};
use askama::Template;
use chrono::Utc;
use uuid::Uuid;

use crate::repositories::user::user_repo::UserRepository;

#[get("/user")]
pub async fn get_users(
    query: web::Query<UsersQuery>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let result = user_repo._read_all(query.into_inner()).await;
    if let Ok(users) = result {
        let user_info_vec: Vec<UserInfo> = users.into_iter().map(|user| user.into()).collect();
        let template = UserInfoTemplate { user_info_vec };

        let body = template.render();
        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok().body(body.expect("Should be valid now."));
    }
    handle_database_error(result.expect_err("Should be error."))
}
#[get("/user/{user_id}")]
pub async fn get_user(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = user_repo.read_one(parsed_id).await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid"));
    }

    handle_database_error(result.expect_err("Should be error."))
}

// For switching the user view into edit mode.
#[get("/user/{user_id}/mode")]
pub async fn toggle_user_edit(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");
    let result = user_repo.read_one(parsed_id).await;

    if let Ok(user) = result {
        let template: UserEditTemplate = UserEditTemplate {
            id: user.id,
            name: user.name,
            email: user.email,
            birth: user.birth,
            gender: user.gender,
        };

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error.");
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be valid"));
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_new_user(new_user: NewUser) -> Result<(), String> {
    if new_user.name.trim().is_empty() || new_user.email.trim().is_empty() {
        return Err("Username or Email empty.".to_string());
    }

    if new_user.birth >= Utc::now().date_naive() {
        return Err("You can't be younger than today!".to_string());
    }

    if !check_email_validity(new_user.email) {
        return Err("Invalid Email format.".to_string());
    }
    Ok(())
}

#[post("/user")]
pub async fn create_user(
    new_user: web::Json<NewUser>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let validation_res = validate_new_user(new_user.clone());
    if let Err(err_msg) = validation_res {
        return HttpResponse::BadRequest().body(err_msg);
    }
    let result = user_repo.create(new_user.into_inner()).await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();
        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError()
                .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        let unwrapped_body = body.unwrap();

        return HttpResponse::Created()
            .content_type("text/html")
            .body(unwrapped_body);
    }

    handle_database_error(result.expect_err("Should be error."))
}

fn validate_edit_data(user_data: UserData) -> Result<(), String> {
    if user_data.name.is_none()
        && user_data.email.is_none()
        && user_data.birth.is_none()
        && user_data.avatar_url.is_none()
        && user_data.role.is_none()
    {
        return Err("No data provided.".to_string());
    }

    if user_data.name.is_some() && user_data.name.unwrap().trim().is_empty() {
        return Err("Username empty.".to_string());
    }

    if user_data.email.is_some() && !check_email_validity(user_data.email.clone().unwrap()) {
        return Err("Invalid email format.".to_string());
    }

    if user_data.avatar_url.is_some() && user_data.avatar_url.unwrap().trim().is_empty() {
        return Err("Empty avatar url.".to_string());
    }

    if user_data.birth.is_some() && user_data.birth.unwrap() >= Utc::now().date_naive() {
        return Err("Can't be older than today!".to_string());
    }

    Ok(())
}

#[patch("/user/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    user_data: web::Json<UserData>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let validation_res = validate_edit_data(user_data.clone());
    if let Err(err_msg) = validation_res {
        return HttpResponse::BadRequest().body(err_msg);
    }

    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = user_repo
        .update_user(parsed_id, user_data.into_inner())
        .await;

    if let Ok(user) = result {
        let template: UserTemplate = user.into();

        let body = template.render();

        if body.is_err() {
            return HttpResponse::InternalServerError().body("Internal server error.".to_string());
        }

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(body.expect("Should be okay."));
    }

    handle_database_error(result.expect_err("Should be error."))
}

#[delete("/user/{user_id}")]
pub async fn delete_user(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body("Incorrect ID format.".to_string());
    }

    let parsed_id = id_parse.expect("Should be valid.");

    let result = user_repo.delete_user(parsed_id).await;

    if let Err(error) = result {
        return handle_database_error(error);
    }

    HttpResponse::NoContent().finish()
}

#[get("/admin")]
pub async fn open_admin_panel() -> HttpResponse {
    let template = AdminTemplate {
        title: "Admin Panel".to_string(),
    };

    let body = template.render();
    if body.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    HttpResponse::Ok().body(body.expect("Should be valid."))
}

#[put("/user/{user_id}/avatar")]
pub async fn upload_user_avatar(
    user_id: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
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
        return HttpResponse::BadRequest().body("Invalid file type.");
    }

    let image_res = store_image(parsed_id, ImageCategory::User, form.file);
    if image_res.is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }
    let image_path = image_res.expect("Should be valid.");
    let data = UserData {
        name: None,
        email: None,
        birth: None,
        gender: None,
        role: None,
        avatar_url: Some(image_path),
    };
    let user_res = user_repo.update_user(parsed_id, data).await;
    if user_res.is_err() {
        return handle_database_error(user_res.expect_err("Should be an error."));
    }
    HttpResponse::Ok().body("New image uploaded!")
}

#[delete("/user/{user_id}/avatar")]
pub async fn remove_user_avatar(
    user_id: web::Path<String>,
    user_repo: web::Data<UserRepository>,
) -> HttpResponse {
    let id_parse = Uuid::from_str(user_id.into_inner().as_str());
    if id_parse.is_err() {
        return HttpResponse::BadRequest().body(parse_error(http::StatusCode::BAD_REQUEST));
    }

    let parsed_id = id_parse.expect("Should be okay.");
    if remove_image(parsed_id, ImageCategory::User).is_err() {
        return HttpResponse::InternalServerError()
            .body(parse_error(http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    let data = UserData {
        name: None,
        email: None,
        birth: None,
        gender: None,
        role: None,
        avatar_url: Some(DEFAULT_USER_IMAGE.to_string()),
    };

    let res = user_repo.update_user(parsed_id, data).await;
    if res.is_err() {
        return handle_database_error(res.expect_err("Should be an error."));
    }
    HttpResponse::Ok().body("Profile image deleted.")
}
