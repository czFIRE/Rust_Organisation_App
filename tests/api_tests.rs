#[cfg(test)]
mod api_tests {
    use std::borrow::Borrow;
    use std::sync::Arc;

    use actix_web::http::header::ContentType;
    use actix_web::{http, web};
    use actix_web::{test, App};
    use chrono::NaiveDate;
    use organization::common::DbResult;
    use organization::handlers::associated_company::get_all_associated_companies_per_event_and_user;
    use organization::models::{EmployeeLevel, EmploymentContract};
    use organization::repositories::assigned_staff::assigned_staff_repo::AssignedStaffRepository;
    use organization::repositories::associated_company::associated_company_repo::AssociatedCompanyRepository;
    use organization::repositories::comment::comment_repo::CommentRepository;
    use organization::repositories::company::company_repo::CompanyRepository;
    use organization::repositories::employment::employment_repo::EmploymentRepository;
    use organization::repositories::event::event_repo::EventRepository;
    use organization::repositories::event_staff::event_staff_repo::StaffRepository;
    use organization::repositories::repository::DbRepository;
    use organization::repositories::task::models::TaskFilter;
    use organization::repositories::task::task_repo::TaskRepository;
    use organization::repositories::timesheet::timesheet_repo::TimesheetRepository;
    use organization::repositories::user::user_repo::UserRepository;

    use organization::handlers::{
        assigned_staff::{
            create_assigned_staff, delete_assigned_staff, get_all_assigned_staff,
            get_assigned_staff, update_assigned_staff,
        },
        associated_company::{
            create_associated_company, delete_associated_company, get_all_associated_companies,
            update_associated_company,
        },
        comment::{
            create_event_comment, create_task_comment, delete_comment,
            open_event_comments_for_user, open_task_comments_for_user, update_comment,
        },
        company::{create_company, delete_company, get_all_companies, get_company, update_company},
        employment::{
            create_employment, delete_employment, get_employment, get_employments_per_user,
            get_subordinates, update_employment,
        },
        event::{create_event, delete_event, get_event, get_events, update_event},
        event_staff::{
            create_event_staff, delete_event_staff, get_all_event_staff, get_event_staff,
            update_event_staff,
        },
        event_task::{create_task, delete_task, get_event_tasks, update_task},
        index::index,
        timesheet::{
            create_timesheet, get_all_timesheets_for_employment, get_timesheet, update_timesheet,
        },
        user::{create_user, delete_user, get_user, update_user},
    };

    use regex::Regex;
    use serde_json::json;
    use sqlx::PgPool;
    use std::str::{self, FromStr};
    use uuid::Uuid;

    #[actix_web::test]
    async fn index_get() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_patch_delete_user_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app = test::init_service(
            App::new()
                .app_data(user_repo.clone())
                .service(create_user)
                .service(update_user)
                .service(delete_user),
        )
        .await;

        let user = json!({
            "name": "Peepo Happy",
            "email": "peepo@happy.com",
            "birth": "1999-01-01",
            "gender": "Male",
            "role": "User"
        });
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(user.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Peepo Happy"));
        assert!(body.contains("peepo@happy.com"));

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();
        let uuid_caps = uuid_regex.captures(body).unwrap();
        let uuid_str = &uuid_caps[0];

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(user)
            .to_request();
        let res = test::call_service(&app, req).await;
        // Email should be unique.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let user_update = json!({
            "name": "Peepo Sad",
        });

        let req = test::TestRequest::patch()
            .uri(format!("/user/{}", uuid_str).as_str())
            .set_json(user_update)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Peepo Sad"));
        assert!(body.contains("peepo@happy.com"));
        assert!(body.contains("img/default/user.jpg"));

        // Update with no data should fail
        let req = test::TestRequest::patch()
            .uri(format!("/user/{}", uuid_str).as_str())
            .set_json(json!({}))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
            .uri(format!("/user/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::delete()
            .uri(format!("/user/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn user_get_existing(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app =
            test::init_service(App::new().app_data(user_repo.clone()).service(get_user)).await;

        let req = test::TestRequest::get()
            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Dave Null"));
        assert!(body.contains("dave@null.com"));
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn user_get_not_existing(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app =
            test::init_service(App::new().app_data(user_repo.clone()).service(get_user)).await;

        let req = test::TestRequest::get()
            .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn user_get_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app =
            test::init_service(App::new().app_data(user_repo.clone()).service(get_user)).await;

        let req = test::TestRequest::get()
            .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_non_existent_user(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app =
            test::init_service(App::new().app_data(user_repo.clone()).service(update_user)).await;

        let user_update = json!({
            "name": "Dave Nill",
        });

        let req = test::TestRequest::patch()
            .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
            .set_json(user_update)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_user_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app = test::init_service(
            App::new()
                .app_data(user_repo.clone())
                .service(create_user)
                .service(update_user)
                .service(delete_user),
        )
        .await;

        let user_update = json!({
            "name": "Dave Nill",
        });

        let req = test::TestRequest::patch()
            .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
            .set_json(user_update)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn delete_user_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let user_repository = UserRepository::new(arc_pool.clone());
        let user_repo = web::Data::new(user_repository);

        let app = test::init_service(
            App::new()
                .app_data(user_repo.clone())
                .service(create_user)
                .service(update_user)
                .service(delete_user),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_companies_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(get_all_companies),
        )
        .await;

        let req = test::TestRequest::get().uri("/company").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_existing_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool); //get_db_pool().await;
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(get_company),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("AMD"));
        assert!(body.contains("crn_amd"));
        assert!(body.contains("vatin_amd"));
        assert!(body.contains("+1 408-749-4000"));
        assert!(body.contains("info@amd.com"));
        assert!(body.contains("2485"));
        assert!(body.contains("United States"));
        assert!(body.contains("CA"));
        assert!(body.contains("Santa Clara"));
        assert!(body.contains("Augustine Drive"));

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_non_existing_company_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(get_company),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/company/b548eed1-538d-48d4-8cee-498e0971f9f5")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_company_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(get_company),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/company/b548eed1-sleepy-head-123zzz")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool); //get_db_pool().await;
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(create_company)
                .service(update_company)
                .service(delete_company),
        )
        .await;

        let company = json!({
            "name": "Lethal Company",
            "description": "We specialize in TOTALLY SAFE salvaging of abandoned space stations.",
            "website": "https://store.steampowered.com/app/1966720/Lethal_Company/",
            "crn": "1234",
            "vatin": "123456",
            "country": "ctr",
            "region": "reg",
            "city": "city",
            "street": "strt",
            "number": "nmbr",
            "postal_code": "pstl",
            "phone": "+420123456789",
            "email": "meet@the.quota.com",
            "employee_id": "4a799b2c-3b5f-41ec-a6e3-442cef915051",
        });

        let req = test::TestRequest::post()
            .uri("/company")
            .set_json(company.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();
        let uuid_caps = uuid_regex.captures(body).unwrap();
        let uuid_str = &uuid_caps[0];

        assert!(body.contains("Lethal Company"));
        assert!(body.contains("1234"));
        assert!(body.contains("123456"));
        assert!(body.contains("+420123456789"));
        assert!(body.contains("meet@the.quota.com"));
        assert!(body.contains("nmbr"));
        assert!(body.contains("ctr"));
        assert!(body.contains("reg"));
        assert!(body.contains("city"));
        assert!(body.contains("strt"));

        // Attempt to create a duplicate.
        let req = test::TestRequest::post()
            .uri("/company")
            .set_json(company)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "crn": "crn1234",
        });

        let req = test::TestRequest::patch()
            .uri(format!("/company/{}", uuid_str).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("crn1234"));
        assert!(body.contains("Lethal Company"));

        // Empty data body.
        let req = test::TestRequest::patch()
            .uri(format!("/company/{}", uuid_str).as_str())
            .set_json(json!({}))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
            .uri(format!("/company/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::delete()
            .uri(format!("/company/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_non_existent_company(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(update_company),
        )
        .await;

        let data = json!({
            "crn": "amd_crn",
            "vatin": "amd_vatin"
        });

        let req = test::TestRequest::patch()
            .uri("/company/b548eed1-538d-48d4-8cee-498e0971f9f5")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_company_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(update_company),
        )
        .await;

        let data = json!({
            "crn": "amd_crn",
            "vatin": "amd_vatin"
        });

        let req = test::TestRequest::patch()
            .uri("/company/b5188gda-sleepy-head-123zzz")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn delete_company_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let company_repository = CompanyRepository::new(arc_pool.clone());
        let company_repo = web::Data::new(company_repository);

        let app = test::init_service(
            App::new()
                .app_data(company_repo.clone())
                .service(delete_company),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/company/b5188eda-sleepy-head-123zzz")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_events_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app =
            test::init_service(App::new().app_data(event_repo.clone()).service(get_events)).await;

        let req = test::TestRequest::get().uri("/event").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_existing_event(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app =
            test::init_service(App::new().app_data(event_repo.clone()).service(get_event)).await;

        let req = test::TestRequest::get()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Woodstock"));
        assert!(body.contains("https://woodstock.com"));
        assert!(body.contains("A legendary music festival"));
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_non_existent_event(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app =
            test::init_service(App::new().app_data(event_repo.clone()).service(get_event)).await;

        let req = test::TestRequest::get()
            .uri("/event/a71cd75e-a811-410a-9bb4-70fc5c7748f8")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_event_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app =
            test::init_service(App::new().app_data(event_repo.clone()).service(get_event)).await;

        let req = test::TestRequest::get()
            .uri("/event/a71cd75e-sleepy-head-111z3zz")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_event_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);
        let employment_repo = web::Data::new(EmploymentRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(event_repo.clone())
                .app_data(employment_repo.clone())
                .service(create_event)
                .service(update_event)
                .service(delete_event),
        )
        .await;

        let start_date = NaiveDate::from_ymd_opt(2027, 04, 06).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2027, 04, 07).unwrap();

        let data = json!({
            "name": "BitConnect Charitative Concert",
            "description": "Return of the best bitcoin app, BitConneeeeeeeeect!",
            "start_date": start_date.clone().to_string(),
            "end_date": end_date.clone().to_string(),
            "creator_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5"
        });

        let req = test::TestRequest::post()
            .uri("/event")
            .set_json(data.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();
        let uuid_caps = uuid_regex.captures(body).unwrap();
        let uuid_str = &uuid_caps[0];

        assert!(body.contains("BitConnect Charitative Concert"));
        assert!(body.contains("Return of the best bitcoin app, BitConneeeeeeeeect!"));

        let data = json!({
            "name": "BitConnect Charitative Event"
        });

        let req = test::TestRequest::patch()
            .uri(format!("/event/{}", uuid_str).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("BitConnect Charitative Event"));
        assert!(body.contains("Return of the best bitcoin app, BitConneeeeeeeeect!"));

        let req = test::TestRequest::delete()
            .uri(format!("/event/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::delete()
            .uri(format!("/event/{}", uuid_str).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_non_existent_event(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app = test::init_service(
            App::new()
                .app_data(event_repo.clone())
                .service(update_event),
        )
        .await;

        let data = json!({
            "name": "Ironstock"
        });

        let req = test::TestRequest::patch()
            .uri("/event/b71fd7ce-c891-410a-9bba-1aacececc8fa")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_event_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app = test::init_service(
            App::new()
                .app_data(event_repo.clone())
                .service(update_event),
        )
        .await;

        let data = json!({});

        let req = test::TestRequest::patch()
            .uri("/event/b71fd7ce-deaf-listenerz-zz123zy")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_event_empty_data(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app = test::init_service(
            App::new()
                .app_data(event_repo.clone())
                .service(update_event),
        )
        .await;

        let data = json!({});

        let req = test::TestRequest::patch()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn delete_event_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let event_repository = EventRepository::new(arc_pool.clone());
        let event_repo = web::Data::new(event_repository);

        let app = test::init_service(
            App::new()
                .app_data(event_repo.clone())
                .service(delete_event),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/event/b71fd7ce-im-rusty-boizzz-1")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_tasks_per_event(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(get_event_tasks)).await;

        let req = test::TestRequest::get()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Prepare stage for Joe Cocker"));
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_tasks_per_event_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(get_event_tasks)).await;

        let req = test::TestRequest::get()
            .uri("/event/bzz-tasks-boi-they-sure-are-difficult-are-they-notzz-z-z-z-zzz/task")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_task_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let assigned_repo = web::Data::new(AssignedStaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(assigned_repo.clone())
                .service(create_task)
                .service(delete_task)
                .service(update_task),
        )
        .await;
        let data = json!({
            "creator_id": "9281b570-4d02-4096-9136-338a613c71cd",
            "title": "Stock the wood pile.",
            "description": "Get 1 item of wood, put it on the pile of X items of wood. Repeat.",
            "priority": "High",
        });

        let req = test::TestRequest::post()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Stock the wood pile."));
        assert!(body.contains("High"));

        let creator_id =
            Uuid::from_str("9281b570-4d02-4096-9136-338a613c71cd").expect("Valid uuid");
        let event_id = Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").expect("Valid uuid");
        let tasks = repo
            .read_all_for_event(
                event_id,
                TaskFilter {
                    limit: None,
                    offset: None,
                },
            )
            .await;
        assert!(tasks.is_ok());
        let tasks_ok = tasks.expect("Should be okay.");
        let task = tasks_ok.into_iter().find(|t| {
            t.creator_id == creator_id
                && t.title == "Stock the wood pile.".to_string()
                && t.description
                    == Some(
                        "Get 1 item of wood, put it on the pile of X items of wood. Repeat."
                            .to_string(),
                    )
        });

        assert!(task.is_some());

        let task_id = task.expect("Should be some").task_id.to_string();

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
            .uri(format!("/event/task/{}", task_id).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Help do stuff."));

        let req = test::TestRequest::patch()
            .uri(format!("/event/task/{}", task_id).as_str())
            .set_json(json!({}))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
            .uri(format!("/event/task/{}", task_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = test::TestRequest::delete()
            .uri(format!("/event/task/{}", task_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_non_existent_task(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let assigned_repo = web::Data::new(AssignedStaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(assigned_repo.clone())
                .service(update_task),
        )
        .await;

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
            .uri("/event/task/7a201017-aa31-4aac-b767-100d18a8877b")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn patch_task_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let assigned_repo = web::Data::new(AssignedStaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(assigned_repo.clone())
                .service(update_task),
        )
        .await;

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
                            .uri("/event/task/IllhaveyouknowIgraduatedtopofmyclassintheNavySealsandIvebeeninvolvedinnumeroussecretraids")
                            .set_json(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn delete_task_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = TaskRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(App::new().app_data(repo.clone()).service(delete_task)).await;

        let req = test::TestRequest::delete()
            .uri("/event/task/yesofficerIamanUUID.")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn open_event_comments_for_user_test(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let staff_repo = web::Data::new(StaffRepository::new(arc_pool.clone()));

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(staff_repo.clone())
                .service(open_event_comments_for_user),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/comment-panel/35341253-da20-40b6-96d8-ce069b1ba5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn open_event_comments_for_user_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let staff_repo = web::Data::new(StaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(staff_repo.clone())
                .service(open_event_comments_for_user),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/event/INVALIDFORMATZZZYYYXXX/comment-panel/asdasdasdads")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_event_comment(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_event_comment)
                .service(update_comment)
                .service(delete_comment),
        )
        .await;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Cool event, maaaaan!"));

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();

        let uuid_caps = uuid_regex.find(body).unwrap();
        let comment_id = uuid_caps.as_str();

        let data = json!({
            "content": "Chill event, maaaaan!",
        });

        let req = test::TestRequest::patch()
            .uri(format!("/comment/{}", comment_id).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Chill event, maaaaan!"));

        // Empty Data Test
        let data = json!({});

        let req = test::TestRequest::patch()
            .uri(format!("/comment/{}", comment_id).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
            .uri(format!("/comment/{}", comment_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        // OK because of HTMX
        assert_eq!(res.status(), http::StatusCode::OK);

        // Deleting an already deleted comment.
        let req = test::TestRequest::delete()
            .uri(format!("/comment/{}", comment_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_event_comment_non_existent_event(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_event_comment)
                .service(update_comment)
                .service(delete_comment),
        )
        .await;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/event/b554d7ac-cdea-410a-9bb4-70fc5c7748f8/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_event_comment_invalid_uuid_format(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_event_comment)
                .service(update_comment)
                .service(delete_comment),
        )
        .await;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/event/uuidied-writingthis/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn update_comment_invalid_uuid(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(update_comment)).await;
        let data = json!({
            "content": "One of the events of all time, maaaaan!",
        });

        let req = test::TestRequest::patch()
            .uri("/comment/uuidied-writingthis")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_task_comments_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let assigned_repo = web::Data::new(AssignedStaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(assigned_repo.clone())
                .service(open_task_comments_for_user),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/comment-panel/9281b570-4d02-4096-9136-338a613c71cd")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_task_comments_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let assigned_repo = web::Data::new(AssignedStaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(assigned_repo.clone())
                .service(open_task_comments_for_user),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/task/7ae0c01zzzyyy4aac-b767-10zzz877b/comment-panel/3534zzzzzzzzzzZZZ5d4")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_task_comment(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_task_comment)
                .service(update_comment)
                .service(delete_comment),
        )
        .await;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool task, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Cool task, maaaaan!"));

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();

        let uuid_caps = uuid_regex.find(body).unwrap();
        let comment_id = uuid_caps.as_str();

        let data = json!({
            "content": "Chill task, maaaaan!",
        });

        let req = test::TestRequest::patch()
            .uri(format!("/comment/{}", comment_id).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Chill task, maaaaan!"));

        // Empty Data Test
        let data = json!({});

        let req = test::TestRequest::patch()
            .uri(format!("/comment/{}", comment_id).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
            .uri(format!("/comment/{}", comment_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        // Deleting an already deleted comment.
        let req = test::TestRequest::delete()
            .uri(format!("/comment/{}", comment_id).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_task_comment_non_existent_task(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_task_comment),
        )
        .await;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/task/b554d7ac-cdea-410a-9bb4-70fc5c7748f8/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_task_comment_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = CommentRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_task_comment),
        )
        .await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
            .uri("/task/uuidied-writingthis/comment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employments_per_user_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employments_per_user),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/user/0465041f-fe64-461f-9f71-71e3b97ca85f/employment")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("0465041f-fe64-461f-9f71-71e3b97ca85f"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employments_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employments_per_user),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/user/wrongUUIDFormatBois/employment")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employment_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employment),
        )
        .await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("35341253-da20-40b6-96d8-ce069b1ba5d4"));
        assert!(body.contains("b5188eda-528d-48d4-8cee-498e0971f9f5"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employment_non_existent_user(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employment),
        )
        .await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-dade-4ac6-96dc-cede9b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employment_non_existent_company(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employment),
        )
        .await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/b5188eda-5bcd-4eda-8cae-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_employment_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_employment),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(
                "/user/35341y53-BADUUID6d8-ce06zzz/employment/b5188eda-5bcd-4eda-8cae-498e0971f9f5",
            )
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_subordinates_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_subordinates),
        )
        .await;

        let req = test::TestRequest::get()
                                .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/134d5286-5f55-4637-9b98-223a5820a464/subordinates")
                                .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_subordinates_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(get_subordinates),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/user/BADUUID/employment/b5188eda-528d-48d4-8cee-498e0971f9f5/subordinates")
            .to_request();
        let res = test::call_service(&app, req).await;
        // Bad UUID format.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::get()
            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/BADUUID/subordinates")
            .to_request();
        let res = test::call_service(&app, req).await;
        // Bad UUID at company ID
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_employment(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(create_employment)
                .service(update_employment)
                .service(delete_employment),
        )
        .await;

        let data = json!({
            "user_id": "ac9bf689-a713-4b66-a3d0-41faaf0f8d0c",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "hourly_wage": 200.0,
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
            "description": "A person.",
            "employment_type": "Hpp",
            "level": "Basic"
        });

        // Can't check for the internal data here because we return
        // the manager's view instead. This call is purely for the frotend.
        let req = test::TestRequest::post()
            .uri("/employment")
            .set_json(data.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let user_uuid = Uuid::from_str("ac9bf689-a713-4b66-a3d0-41faaf0f8d0c").expect("Valid uuid");
        let company_uuid =
            Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").expect("Valid uuid");
        let employment_res = employment_repo.read_one(user_uuid, company_uuid).await;
        assert!(employment_res.is_ok());
        let employment = employment_res.expect("Should be okay");

        assert!(employment.manager.is_some());
        assert_eq!(employment.level, EmployeeLevel::Basic);
        assert_eq!(employment.employment_type, EmploymentContract::Hpp);

        let req = test::TestRequest::post()
            .uri("/employment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        // Creating a duplicate employment.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "editor_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "level": "Manager",
        });

        let req = test::TestRequest::patch()
                            .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_json(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_json(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        // Patching empty data.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
                            .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = test::TestRequest::delete()
                            .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_employment_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let employment_repository = EmploymentRepository::new(arc_pool.clone());
        let employment_repo = web::Data::new(employment_repository);

        let app = test::init_service(
            App::new()
                .app_data(employment_repo.clone())
                .service(create_employment),
        )
        .await;

        let data = json!({
            "user_id": "0465041f-INVALID4-461f-9f71-71aaagf",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "employment_type": "Hpp",
            "hourly_wage": 200.0,
            "employee_level": "Basic",
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
        });

        let req = test::TestRequest::post()
            .uri("/employment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        // The user ID is invalid.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "employment_type": "Hpp",
            "hourly_rate": 200.0,
            "employee_level": "Basic",
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
        });

        let req = test::TestRequest::post()
            .uri("/employment")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        // Error: No company ID
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_event_staff_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let staff_repository = StaffRepository::new(arc_pool.clone());
        let staff_repo = web::Data::new(staff_repository);

        let app = test::init_service(
            App::new()
                .app_data(staff_repo.clone())
                .service(get_all_event_staff),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("b71fd7ce-c891-410a-9bb4-70fc5c7748f8"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_event_staff_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let staff_repository = StaffRepository::new(arc_pool.clone());
        let staff_repo = web::Data::new(staff_repository);

        let app = test::init_service(
            App::new()
                .app_data(staff_repo.clone())
                .service(get_all_event_staff),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/beezzz-4INVALIDFORMATbBOIYSb4-70fc5c7748f8/staff")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_event_staff_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let staff_repository = StaffRepository::new(arc_pool.clone());
        let staff_repo = web::Data::new(staff_repository);

        let app = test::init_service(
            App::new()
                .app_data(staff_repo.clone())
                .service(get_event_staff),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/staff/9281b570-4d02-4096-9136-338a613c71cd")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("b71fd7ce-c891-410a-9bb4-70fc5c7748f8"));
        assert!(body.contains("9281b570-4d02-4096-9136-338a613c71cd"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_event_staff_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let staff_repository = StaffRepository::new(arc_pool.clone());
        let staff_repo = web::Data::new(staff_repository);

        let app = test::init_service(
            App::new()
                .app_data(staff_repo.clone())
                .service(get_event_staff),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/staff/918ab570-adb3-4c9d-9136-338a613c71cd")
            .to_request();
        let res = test::call_service(&app, req).await;

        // Staff does not exist
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        let req = test::TestRequest::get()
            .uri("/event/staff/9zzzzz0-adb3-4czz36-338az3c71cd")
            .to_request();
        let res = test::call_service(&app, req).await;

        // Invalid UUID format.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_event_staff(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let staff_repository = StaffRepository::new(arc_pool.clone());
        let staff_repo = web::Data::new(staff_repository);

        let timesheet_repo = web::Data::new(TimesheetRepository::new(arc_pool.clone()));
        let event_repo = web::Data::new(EventRepository::new(arc_pool.clone()));
        let associated_repo = web::Data::new(AssociatedCompanyRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(staff_repo.clone())
                .app_data(timesheet_repo.clone())
                .app_data(event_repo.clone())
                .app_data(associated_repo.clone())
                .service(create_event_staff)
                .service(update_event_staff)
                .service(delete_event_staff),
        )
        .await;

        let data = json!({
            "user_id": "51a01dbf-dcd5-43a0-809c-94ed8e61d420",
            "company_id": "71fa27d6-6f00-4ad0-8902-778e298aaed2",
            "role": "Staff"
        });

        let req = test::TestRequest::post()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff")
            .set_json(data.clone())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Your application is still under review."));

        let event_id = Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").expect("Valid uuid");
        let user_id = Uuid::from_str("51a01dbf-dcd5-43a0-809c-94ed8e61d420").expect("Valid uuid");
        let staff_res = staff_repo
            .read_by_event_and_user_id(event_id, user_id)
            .await;
        assert!(staff_res.is_ok());
        let staff = staff_res.expect("Should be some.");
        let staff_id = staff.id;

        // No data.
        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(json!({}))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        // Trying to set status without providing decided_by.
        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(json!({
                "status": "Accepted"
            }))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        // Decider is not an organizer of the event.
        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(json!({
                "status": "Accepted",
                "decided_by": staff_id.to_string(),
            }))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        // Decider is an organizer, but not for this event.
        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(json!({
                "status": "Accepted",
                "decided_by": "aa7f3d0e-ab48-473b-ac69-b84cb74f34f7",
            }))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        // Setting status to accepted with valid decider.
        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(json!({
                "status": "Accepted",
                "decided_by": "9281b570-4d02-4096-9136-338a613c71cd"
            }))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let data = json!({
            "role": "Organizer",
        });

        let req = test::TestRequest::patch()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Organizer"));

        let req = test::TestRequest::delete()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = test::TestRequest::delete()
            .uri(
                format!(
                    "/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}",
                    staff_id.to_string()
                )
                .as_str(),
            )
            .to_request();
        let res = test::call_service(&app, req).await;
        // Duplicate delete
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_assigned_staff_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_assigned_staff),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff")
            .to_request();

        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Dave Null"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_assigned_staff_invalid_uuid(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_assigned_staff),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/task/BADUUIDBOIS/staff")
            .to_request();

        let res = test::call_service(&app, req).await;

        // Invalid uuid
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_assigned_staff_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_assigned_staff),
        )
        .await;
        let req = test::TestRequest::get()
                    .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/9281b570-4d02-4096-9136-338a613c71cd")
                    .to_request();

        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("9281b570-4d02-4096-9136-338a613c71cd"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_assigned_staff_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_assigned_staff),
        )
        .await;

        let req = test::TestRequest::get()
                    .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/9281b570-4d02-4ab6-9cd6-3e8a613c71cd")
                    .to_request();

        let res = test::call_service(&app, req).await;
        // Non-existent staff
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        let req = test::TestRequest::get()
                    .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/INVALIDUUIDZZ-4ab6-9cd6-3e8a613c71cd")
                    .to_request();

        let res = test::call_service(&app, req).await;
        // Invalid UUID format
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_assigned_staff(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let staff_repo = web::Data::new(StaffRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(staff_repo.clone())
                .service(create_assigned_staff)
                .service(update_assigned_staff)
                .service(delete_assigned_staff),
        )
        .await;
        let data = json!({
            "staff_id": "a96d1d99-93b5-469b-ac62-654b0cf7ebd3"
        });

        let req = test::TestRequest::post()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff")
            .set_json(data)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("7ae0c017-fe31-4aac-b767-100d18a8877b"));
        assert!(body.contains("Pending") || body.contains("pending"));

        let data = json!({
            "status": "Rejected",
            "decided_by": "9281b570-4d02-4096-9136-338a613c71cd"
        });

        let req = test::TestRequest::patch()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/a96d1d99-93b5-469b-ac62-654b0cf7ebd3")
            .set_json(data)
            .to_request();

        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("7ae0c017-fe31-4aac-b767-100d18a8877b"));
        assert!(body.contains("Rejected") || body.contains("rejected"));

        let data = json!({
            "status": "Accepted",
        });

        let req = test::TestRequest::patch()
            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/a96d1d99-93b5-469b-ac62-654b0cf7ebd3")
            .set_json(data)
            .to_request();

        let res = test::call_service(&app, req).await;
        // Data without decided_by
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
        .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/a96d1d99-93b5-469b-ac62-654b0cf7ebd3")
        .to_request();

        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        // OK because of HTMX. If we put no-content, no frontend updates would happen.
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = test::TestRequest::delete()
        .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/staff/a96d1d99-93b5-469b-ac62-654b0cf7ebd3")
        .to_request();

        let res = test::call_service(&app, req).await;
        // Trying to delete a non-existing entry.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_assigned_staff_errors(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssignedStaffRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_assigned_staff)
                .service(update_assigned_staff)
                .service(delete_assigned_staff),
        )
        .await;

        let data = json!({
            "staff_id": "a96d1d99-93b5-469b-ac62-654b0cf7ebd3"
        });

        let req = test::TestRequest::post()
            .uri("/task/7aey-FEELZ-INVALIDUUIDUDE767-100d18a8877b/staff")
            .set_json(data)
            .to_request();

        let res = test::call_service(&app, req).await;

        // Invalid UUID
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({});

        let req = test::TestRequest::post()
            .uri("/task/7ae0c017-fe31-4dde-b653-1acd18a8877b/staff")
            .set_json(data)
            .to_request();

        let res = test::call_service(&app, req).await;

        // Empty data
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_associated_companies_per_event(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssociatedCompanyRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_associated_companies),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("AMD"));
        assert!(body.contains("Prusa Research"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_associated_comapnies_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssociatedCompanyRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_associated_companies),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/BADUUIDZZZZZZZZZc7748f8/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_associated_companies_per_event_and_user_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssociatedCompanyRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let emp_repo = web::Data::new(EmploymentRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(emp_repo.clone())
                .service(get_all_associated_companies_per_event_and_user),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/3f152d12-0bbd-429a-a9c5-28967d6370cc/user/0465041f-fe64-461f-9f71-71e3b97ca85f/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("134d5286-5f55-4637-9b98-223a5820a464"));

        let req = test::TestRequest::get()
            .uri("/event/3f152dad-0bbd-4e9a-aec5-2a567d6370cc/user/0465041f-fe64-461f-9f71-71e3b97ca85f/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(!body.contains("134d5286-5f55-4637-9b98-223a5820a464"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_associated_companies_per_event_and_user_errors_test(
        pool: PgPool,
    ) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssociatedCompanyRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let emp_repo = web::Data::new(EmploymentRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(emp_repo.clone())
                .service(get_all_associated_companies_per_event_and_user),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/event/3f152fds-asddasc5-zzz/user/0465041f-fe64-461f-9f71-71e3b97ca85f/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::get()
            .uri("/event/3f152d12-0bbd-429a-a9c5-28967d6370cc/user/zzzyyy-71zzzcooo7ca85f/company")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_delete_associated_company(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = AssociatedCompanyRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(create_associated_company)
                .service(update_associated_company)
                .service(delete_associated_company),
        )
        .await;

        let data = json!({
          "company_id": "134d5286-5f55-4637-9b98-223a5820a464",
          "association_type": "Sponsor",
        });

        let req = test::TestRequest::post()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company")
            .set_json(data.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Sponsor"));
        assert!(body.contains("ReportLab"));

        //Duplicate creation should fail
        let req = test::TestRequest::post()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company")
            .set_json(data.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        // Invalid UUID should fail
        let req = test::TestRequest::post()
            .uri("/event/BADUUIDFORMATZZZ/company")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "association_type": "Other",
        });

        let req = test::TestRequest::patch()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company/134d5286-5f55-4637-9b98-223a5820a464")
                    .set_json(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Other"));
        assert!(body.contains("ReportLab"));

        let data = json!({});

        let req = test::TestRequest::patch()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company/134d5286-5f55-4637-9b98-223a5820a464")
                    .set_json(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "association_type": "Other",
        });

        let req = test::TestRequest::patch()
            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company/INVALIDUUID")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company/134d5286-5f55-4637-9b98-223a5820a464")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = test::TestRequest::delete()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/company/134d5286-5f55-4637-9b98-223a5820a464")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_timesheets_for_employment_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_timesheets_for_employment),
        )
        .await;

        let req = test::TestRequest::get()
                    .uri("/user/51a01dbf-dcd5-43a0-809c-94ed8e61d420/employment/71fa27d6-6f00-4ad0-8902-778e298aaed2/sheet")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Darkness 2024"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_all_timesheets_for_employment_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .service(get_all_timesheets_for_employment),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/user/3aZZZBADUUIDY/employment/b5188eda-528d-48d4-8cee-498e0971f9f5/sheet")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_timesheet_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(get_timesheet)).await;
        let req = test::TestRequest::get()
            .uri("/timesheet/c51e77aa-bd80-42c7-8b8a-003f018328f6")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();

        assert!(body.contains("Darkness 2024"));
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_non_existent_timesheet(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(get_timesheet)).await;
        let req = test::TestRequest::get()
            .uri("/timesheet/dabe8141-a27e-4c55-a2d5-4a77de24b6d0")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn get_timesheet_invalid_uuid_format(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);

        let app =
            test::init_service(App::new().app_data(repo.clone()).service(get_timesheet)).await;
        let req = test::TestRequest::get()
            .uri("/timesheet/BADFORMATZ12")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create_update_timesheet(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);
        let repository = TimesheetRepository::new(arc_pool.clone());
        let repo = web::Data::new(repository);
        let employment_repo = web::Data::new(EmploymentRepository::new(arc_pool.clone()));
        let app = test::init_service(
            App::new()
                .app_data(repo.clone())
                .app_data(employment_repo.clone())
                .service(create_timesheet)
                .service(update_timesheet),
        )
        .await;

        let data = json!({
             "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
             "company_id": "134d5286-5f55-4637-9b98-223a5820a464",
             "event_id": "3f152d12-0bbd-429a-a9c5-28967d6370cc",
             "start_date": "1969-08-15",
             "end_date": "1969-08-18"
        });

        let req = test::TestRequest::post()
            .uri("/timesheet")
            .set_json(data.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        assert!(body.contains("Darkness 2024"));

        let uuid_regex = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();
        let uuid_caps = uuid_regex.find(body).unwrap();
        let timesheet_id = uuid_caps.as_str();

        let data = json!({
            "manager_note": "Hey, fill out your sheet.",
        });
        let req = test::TestRequest::patch()
            .uri(format!("/timesheet/{}", timesheet_id.to_string()).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let data = json!({});
        let req = test::TestRequest::patch()
            .uri(format!("/timesheet/{}", timesheet_id.to_string()).as_str())
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }
}
