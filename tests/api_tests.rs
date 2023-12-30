#[cfg(test)]
mod api_tests {
    use std::borrow::Borrow;

    use actix_web::http::{Method, self, StatusCode};
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};
    use chrono::{NaiveDate, Utc, TimeZone};
    use organization::models::{UserRole, StaffLevel};
    use organization::templates::comment::{CommentTemplate, CommentsTemplate};
    use organization::templates::company::{CompaniesTemplate, CompanyTemplate};
    use organization::templates::employment::{EmploymentTemplate, EmploymentsTemplate};
    use organization::templates::event::{EventsTemplate, EventTemplate};
    use organization::templates::staff::{AllStaffTemplate, StaffTemplate};
    use organization::templates::task::{TasksTemplate, TaskTemplate};
    use organization::templates::user::UserTemplate;
    use serde_json::json;
    use uuid::Uuid;
    use std::str::{self, FromStr};

    struct Error {
        error: String,
    }

    #[actix_web::test]
    async fn index_get() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let req = test::TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn create_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let user = json!({
            "name": "Peepo Happy",
            "email": "peepo@happy.com",
            "birth": "1999-01-01 00:00:00",
            "gender": "male",
            "role": "user"
        });
        let req = test::TestRequest::post().uri("/user").set_form(user).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let user_template = serde_json::from_str::<UserTemplate>(body).unwrap();
        assert_eq!(user_template.name, "Peepo Happy");
        assert_eq!(user_template.email, "peepo@happy.com");
    }

    #[actix_web::test]
    async fn create_user_duplicate() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let user = json!({
            "name": "Peepo Sad",
            "email": "peepo@sad.com",
            "birth": "1999-01-01 00:00:00",
            "gender": "male",
            "role": "user"
        });
        let req = test::TestRequest::post().uri("/user").set_form(user.clone()).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());

        let req = test::TestRequest::post().uri("/user").set_form(user).to_request();
        let res = test::call_service(&app, req).await;
        // Email should be unique.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn user_get_existing() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let user_template = serde_json::from_str::<UserTemplate>(body).unwrap();
        assert_eq!(user_template.name, "Dave Null");
        assert_eq!(user_template.email, "dave@null.com");
        assert_eq!(user_template.avatar_url, "dave.jpg");
    }

    #[actix_web::test]
    async fn user_get_not_existing() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn user_get_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_existing_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let user_update = json!({
            "name": "Dave Nill",
        });

        let req = test::TestRequest::patch()
                    .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
                    .set_form(user_update)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let user_template = serde_json::from_str::<UserTemplate>(body).unwrap();
        assert_eq!(user_template.name, "Dave Nill");
        assert_eq!(user_template.email, "dave@null.com");
        assert_eq!(user_template.avatar_url, "dave.jpg");
    }

    #[actix_web::test]
    async fn patch_non_existent_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let user_update = json!({
            "name": "Dave Nill",
        });

        let req = test::TestRequest::patch()
                    .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
                    .set_form(user_update)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn patch_user_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let user_update = json!({
            "name": "Dave Nill",
        });

        let req = test::TestRequest::patch()
                    .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
                    .set_form(user_update)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_user_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let user_update = json!({});

        let req = test::TestRequest::patch()
                    .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
                    .set_form(user_update)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn delete_user_exists() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_user_invalid_uuid_format () {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }


    #[actix_web::test]
    async fn get_user_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn upload_user_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn remove_user_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_companies() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let companies_template = serde_json::from_str::<CompaniesTemplate>(body).unwrap();
        assert_eq!(companies_template.companies.len(), 3);
    }

    #[actix_web::test]
    async fn get_existing_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CompanyTemplate>(body).unwrap();
        assert_eq!(out.name, "AMD");
        assert_eq!(out.crn, "crn_amd");
        assert_eq!(out.vatin, "vatin_amd");
        assert_eq!(out.phone, "+1 408-749-4000");
        assert_eq!(out.email, "info@amd.com");
        assert_eq!(out.address.address_number, "2485");
        assert_eq!(out.address.country, "United States");
        assert_eq!(out.address.region, "CA");
        assert_eq!(out.address.city, "Santa Clara");
        assert_eq!(out.address.street, "Augustine Drive");
    }

    #[actix_web::test]
    async fn get_non_existing_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company/b548eed1-538d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company/b548eed1-sleepy-head-123zzz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let company = json!({
            "name": "Pepe Productions",
            "description": "For all your meemz and needz",
            "website": "www.trollfacecomics.com",
            "crn": "pepe_crn",
            "vatin": "pepe_vatin",
            "country": "Landia",
            "region": "Landenten",
            "city": "Citia",
            "street": "Roadton Ave.",
            "number": "69",
            "postal_code": "420 00",
            "phone": "+0 123456789",
            "email": "pepe@products.com"
        });

        let req = test::TestRequest::post()
                            .uri("/company")
                            .set_form(company)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CompanyTemplate>(body).unwrap();
        assert_eq!(out.name, "Pepe Productions");
        assert_eq!(out.crn, "pepe_crn");
        assert_eq!(out.vatin, "pepe_vatin");
        assert_eq!(out.phone, "+0 123456789");
        assert_eq!(out.email, "pepe@products.com");
        assert_eq!(out.address.address_number, "69");
        assert_eq!(out.address.country, "Landia");
        assert_eq!(out.address.region, "Landenten");
        assert_eq!(out.address.city, "Citia");
        assert_eq!(out.address.street, "Roadton Ave.");
    }

    #[actix_web::test]
    async fn create_duplicate_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let company = json!({
            "name": "Lethal Company",
            "description": "We specialize in TOTALLY SAFE salvaging of abandoned space stations.",
            "website": "https://store.steampowered.com/app/1966720/Lethal_Company/",
            "crn": "1234",
            "vatin": "123456",
            "country": "???",
            "region": "???",
            "city": "???",
            "street": "???",
            "number": "???",
            "postal_code": "???",
            "phone": "+0 123456789",
            "email": "meet@the.quota"
        });

        let req = test::TestRequest::post()
                            .uri("/company")
                            .set_form(company.clone())
                            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let req = test::TestRequest::post()
                    .uri("/company")
                    .set_form(company)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({
            "crn": "amd_crn",
            "vatin": "amd_vatin"
        });

        let req = test::TestRequest::patch()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CompanyTemplate>(body).unwrap();
        assert_eq!(out.name, "AMD");
        assert_eq!(out.crn, "amd_crn");
        assert_eq!(out.vatin, "amd_vatin");
        assert_eq!(out.phone, "+1 408-749-4000");
        assert_eq!(out.email, "info@amd.com");
        assert_eq!(out.address.address_number, "2485");
        assert_eq!(out.address.country, "United States");
        assert_eq!(out.address.region, "CA");
        assert_eq!(out.address.city, "Santa Clara");
        assert_eq!(out.address.street, "Augustine Drive");
    }

    #[actix_web::test]
    async fn patch_non_existent_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({
            "crn": "amd_crn",
            "vatin": "amd_vatin"
        });

        let req = test::TestRequest::patch()
                            .uri("/company/b548eed1-538d-48d4-8cee-498e0971f9f5")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn patch_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({
            "crn": "amd_crn",
            "vatin": "amd_vatin"
        });

        let req = test::TestRequest::patch()
                            .uri("/company/b5188gda-sleepy-head-123zzz")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_company_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn delete_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-sleepy-head-123zzz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_company_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn upload_company_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn remove_company_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_events() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EventsTemplate>(body).unwrap();
        assert_eq!(out.events.len(), 1);
    }
    
    #[actix_web::test]
    async fn get_existing_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EventTemplate>(body).unwrap();
        assert_eq!(out.name, "Woodstock");
        assert_eq!(out.website, Some("https://woodstock.com".to_string()));
        assert!(out.accepts_staff);
        assert_eq!(out.description, Some("A legendary music festival".to_string()));
        assert_eq!(out.start_date, NaiveDate::from_ymd_opt(1969, 8, 15).unwrap());
        assert_eq!(out.end_date, NaiveDate::from_ymd_opt(1969, 8, 18).unwrap());
    }

    #[actix_web::test]
    async fn get_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event/a71cd75e-a811-410a-9bb4-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_event_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event/a71cd75e-sleepy-head-111z3zz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let start_date = Utc.with_ymd_and_hms(2027, 04, 06, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 04, 07, 0, 0, 0).unwrap();

        let data = json!({  
            "name": "BitConnect Charitative Concert",
            "description": "Return of the best bitcoin app, BitConneeeeeeeeect!",
            "start_date": start_date.clone().to_string(),
            "end_date": end_date.clone().to_string(),
        });

        let req = test::TestRequest::post()
                            .uri("/event")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EventTemplate>(body).unwrap();
        assert_eq!(out.name, "BitConnect Charitative Concert");
        // Accepts staff should be default true when event is created.
        assert!(out.accepts_staff);
        assert_eq!(out.description, Some("Return of the best bitcoin app, BitConneeeeeeeeect!".to_string()));
        assert_eq!(out.start_date, start_date.date_naive());
        assert_eq!(out.end_date, end_date.date_naive());
    }

    #[actix_web::test]
    async fn create_event_duplicate() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let start_date = Utc.with_ymd_and_hms(2027, 04, 06, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 04, 07, 0, 0, 0).unwrap();

        let data = json!({  
            "name": "BitConnect Charitative Concert",
            "description": "Return of the best bitcoin app, BitConneeeeeeeeect!",
            "start_date": start_date.clone().to_string(),
            "end_date": end_date.clone().to_string(),
        });

        let req = test::TestRequest::post()
                            .uri("/event")
                            .set_form(data.clone())
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let req = test::TestRequest::post()
                            .uri("/event")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "name": "Ironstock"
        });

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EventTemplate>(body).unwrap();
        assert_eq!(out.name, "Ironstock");
        assert_eq!(out.website, Some("https://woodstock.com".to_string()));
        assert!(out.accepts_staff);
        assert_eq!(out.description, Some("A legendary music festival".to_string()));
        assert_eq!(out.start_date, NaiveDate::from_ymd_opt(1969, 8, 15).unwrap());
        assert_eq!(out.end_date, NaiveDate::from_ymd_opt(1969, 8, 18).unwrap());
    }

    #[actix_web::test]
    async fn patch_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "name": "Ironstock"
        });

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bba-1aacececc8fa")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn patch_event_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-deaf-listenerz-zz123zy")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_event_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn delete_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b7afddce-c8fe-45aa-a12c-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_event_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-im-rusty-boizzz-1")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_event_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn upload_event_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn remove_event_avatar() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_tasks_per_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<TasksTemplate>(body).unwrap();
        assert_eq!(out.tasks.len(), 1);
        assert_eq!(out.tasks[0].event_id, Uuid::from_str("b71fd7ce-c891-410a-9bba-1aacececc8fa").unwrap());
    }

    #[actix_web::test]
    async fn get_all_tasks_per_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/ba1cd734-c571-42ea-9bb4-70fc5c7748f8/task")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_all_tasks_per_event_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/ba1cd734-tasks-boi-they-sure-are-difficult-are-they-not?")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_one_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<TaskTemplate>(body).unwrap();
        assert_eq!(out.title, "Prepare stage for Joe Cocker".to_string());
        assert_eq!(out.id, Uuid::from_str("7ae0c017-fe31-4aac-b767-100d18a8877b").unwrap());
        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bba-1aacececc8fa").unwrap());
        assert!(out.accepts_staff);
    }

    #[actix_web::test]
    async fn get_one_task_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/baaadfcf-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_non_existent_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fffe-4aac-b767-1aacca8877b")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_one_task_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                    .uri("/event/sleepy-head-I-am?-70fc5c7748f8/task/nowaythiscanbeavalidUUIDbrotherrr")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "creator_id": "9281b570-4d02-4096-9136-338a613c71cd",
            "title": "Stock the wood pile.",
            "priority": "high"
        });

        let req = test::TestRequest::post()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<TaskTemplate>(body).unwrap();
        assert!(out.accepts_staff);
        assert!(out.finished_at.is_none());
        assert_eq!(out.title, "Stock the wood pile.".to_string());
        assert_eq!(out.creator.id, Uuid::from_str("9281b570-4d02-4096-9136-338a613c71cd").unwrap());
        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
    }

    #[actix_web::test]
    async fn create_task_duplicate() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "creator_id": "9281b570-4d02-4096-9136-338a613c71cd",
            "title": "Unstock the wood pile.",
            "priority": "low"
        });

        let req = test::TestRequest::post()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
                            .set_form(data.clone())
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);

        let req = test::TestRequest::post()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<TaskTemplate>(body).unwrap();
        assert!(out.accepts_staff);
        assert!(out.finished_at.is_none());
        assert_eq!(out.title, "Help do stuff.".to_string());
        assert_eq!(out.creator.id, Uuid::from_str("9281b570-4d02-4096-9136-338a613c71cd").unwrap());
        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
    }

    #[actix_web::test]
    async fn patch_non_existent_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7a201017-aa31-4aac-b767-100d18a8877b")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn patch_task_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "title": "Help do stuff."
        });

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/IllhaveyouknowIgraduatedtopofmyclassintheNavySealsandIvebeeninvolvedinnumeroussecretraids")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_task_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn delete_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d1fa88aac")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_task_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/yesofficerIamanUUID.")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }


    //ToDo: Insert an event comment into the DB :|
    #[actix_web::test]
    async fn get_all_event_comments() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentsTemplate>(body).unwrap();
        assert_eq!(out.comments.len(), 1);
        assert_eq!(out.comments.first().unwrap().parent_category_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
    }

    #[actix_web::test]
    async fn get_all_event_comments_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/event/beefdace-c1a1-410a-9bb4-70fc5c7748f8/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_all_event_comments_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/event/INVALIDFORMATZZZYYYXXX/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_update_delete_event_comment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentTemplate>(body).unwrap();
        assert_eq!(out.author.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
        assert_eq!(out.content, "Cool event, maaaaan!");
        assert_eq!(out.parent_category_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
        let comment_id = out.id;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Chill event, maaaaan!",
        });

        let req = test::TestRequest::put()
                    .uri(format!("/comment/{}", comment_id.clone().to_string()).as_str())
                    .set_form(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentTemplate>(body).unwrap();
        assert_eq!(out.author.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
        assert_eq!(out.id, comment_id);
        assert_eq!(out.content, "Chill event, maaaaan!".to_string());

        // Empty Data Test
        let data = json!({});

        let req = test::TestRequest::put()
                    .uri(format!("/comment/{}", comment_id.clone().to_string()).as_str())
                    .set_form(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
                    .uri(format!("/comment/{}", comment_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
        
        // Deleting an already deleted comment.
        let req = test::TestRequest::delete()
                    .uri(format!("/comment/{}", comment_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn create_event_comment_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/event/b554d7ac-cdea-410a-9bb4-70fc5c7748f8/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn create_event_comment_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/event/uuidied-writingthis/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn update_comment_invalid_uuid() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "One of the events of all time, maaaaan!",
        });

        let req = test::TestRequest::put()
                            .uri("/comment/uuidied-writingthis")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_all_task_comments() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentsTemplate>(body).unwrap();
        assert_eq!(out.comments.len(), 1);
        assert_eq!(out.comments.first().unwrap().parent_category_id, Uuid::from_str("7ae0c017-fe31-4aac-b767-100d18a8877b").unwrap());
    }

    #[actix_web::test]
    async fn get_all_task_comments_non_existent_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/task/7aecc0d7-fe32-3bdc-b767-100d18a8877b/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_all_task_comments_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/task/INVALIDUUIDFORMATZZZ/comment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_update_delete_task_comment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool task, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/task/7ae0c017-fe31-4aac-b767-100d18a8877b/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentTemplate>(body).unwrap();
        assert_eq!(out.author.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
        assert_eq!(out.content, "Cool task, maaaaan!");
        assert_eq!(out.parent_category_id, Uuid::from_str("7ae0c017-fe31-4aac-b767-100d18a8877b").unwrap());
        let comment_id = out.id;
        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Chill task, maaaaan!",
        });

        let req = test::TestRequest::put()
                    .uri(format!("/comment/{}", comment_id.clone().to_string()).as_str())
                    .set_form(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<CommentTemplate>(body).unwrap();
        assert_eq!(out.author.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
        assert_eq!(out.id, comment_id);
        assert_eq!(out.content, "Chill task, maaaaan!".to_string());

        // Empty Data Test
        let data = json!({});

        let req = test::TestRequest::put()
                    .uri(format!("/comment/{}", comment_id.clone().to_string()).as_str())
                    .set_form(data)
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
                    .uri(format!("/comment/{}", comment_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
        
        // Deleting an already deleted comment.
        let req = test::TestRequest::delete()
                    .uri(format!("/comment/{}", comment_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn create_task_comment_non_existent_task() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/task/b554d7ac-cdea-410a-9bb4-70fc5c7748f8/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn create_task_comment_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "author_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "content": "Cool event, maaaaan!",
        });

        let req = test::TestRequest::post()
                            .uri("/task/uuidied-writingthis/comment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_employments_per_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EmploymentsTemplate>(body).unwrap();
        assert_eq!(out.employments.len(), 1);
    }

    #[actix_web::test]
    async fn get_employments_non_existent_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35221a5b-da2c-4fe6-96d8-ce069b1ba5d4/employment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_employments_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/wrongUUIDFormatBois/employment")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EmploymentTemplate>(body).unwrap();
        assert_eq!(out.company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
        assert_eq!(out.user_id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
    }

    #[actix_web::test]
    async fn get_employment_non_existent_user() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-dade-4ac6-96dc-cede9b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_employment_non_existent_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/b5188eda-5bcd-4eda-8cae-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_employment_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/user/35341y53-BADUUID6d8-ce06zzz/employment/b5188eda-5bcd-4eda-8cae-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    //ToDo: No data for this yet.
    #[actix_web::test]
    async fn get_subordinates() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let req = test::TestRequest::get()
                                .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5/subordinates")
                                .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EmploymentsTemplate>(body).unwrap();
        assert_eq!(out.employments.len(), 1);
        assert_eq!(out.employments.first().unwrap().company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
    }

    #[actix_web::test]
    async fn get_subordinates_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let req = test::TestRequest::get()
                                .uri("/user/353ae253-dab6-55e6-96d8-ce069b1ba5d4/employment/b5188eda-528d-48d4-8cee-498e0971f9f5/subordinates")
                                .to_request();
        let res = test::call_service(&app, req).await;
        // User does not exist.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

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
    }

    #[actix_web::test]
    async fn create_update_delete_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "employment_type": "hpp",
            "hourly_rate": "200.0",
            "employee_level": "basic",
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
        });

        let req = test::TestRequest::post()
                            .uri("/employment")
                            .set_form(data.clone())
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EmploymentTemplate>(body).unwrap();
        assert_eq!(out.company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
        assert_eq!(out.user_id, Uuid::from_str("0465041f-fe64-461f-9f71-71e3b97ca85f").unwrap());
        assert_eq!(out.manager.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());

        let req = test::TestRequest::post()
                            .uri("/employment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        // Creating a duplicate employment.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "description": "Dirt Shoveller"
        });

        let req = test::TestRequest::patch()
                            .uri("/user/0465041f-fe64-461f-9f71-71e3b97ca85f/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<EmploymentTemplate>(body).unwrap();
        assert_eq!(out.company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
        assert_eq!(out.user_id, Uuid::from_str("0465041f-fe64-461f-9f71-71e3b97ca85f").unwrap());
        assert_eq!(out.manager.id, Uuid::from_str("35341253-da20-40b6-96d8-ce069b1ba5d4").unwrap());
        assert_eq!(out.description, Some("Dirt Shoveller".to_string()));

        let data = json!({});

        let req = test::TestRequest::patch()
                            .uri("/user/0465041f-fe64-461f-9f71-71e3b97ca85f/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        // Patching empty data.
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let req = test::TestRequest::delete()
                            .uri("/user/0465041f-fe64-461f-9f71-71e3b97ca85f/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::delete()
                            .uri("/user/0465041f-fe64-461f-9f71-71e3b97ca85f/employment/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

    }

    #[actix_web::test]
    async fn create_employment_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let data = json!({
            "user_id": "0465041f-INVALID4-461f-9f71-71aaagf",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "employment_type": "hpp",
            "hourly_rate": "200.0",
            "employee_level": "basic",
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
        });

        let req = test::TestRequest::post()
                            .uri("/employment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        // The user ID is invalid.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "manager_id": "35341253-da20-40b6-96d8-ce069b1ba5d4",
            "employment_type": "hpp",
            "hourly_rate": "200.0",
            "employee_level": "basic",
            "start_date": "2022-12-23",
            "end_date": "2022-12-26",
        });

        let req = test::TestRequest::post()
                            .uri("/employment")
                            .set_form(data)
                            .to_request();
        let res = test::call_service(&app, req).await;
        // Error: No company ID
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }
    
    #[actix_web::test]
    async fn get_all_event_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        let req = test::TestRequest::get()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff")
                            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<AllStaffTemplate>(body).unwrap();
        assert_eq!(out.staff.len(), 1);
    }

    #[actix_web::test]
    async fn get_all_event_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;

        // This should be a non-existent event.
        let req = test::TestRequest::get()
                            .uri("/event/beefdbce-caaa-410a-9bb4-70fc5c7748f8/staff")
                            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        let req = test::TestRequest::get()
                            .uri("/event/beezzzfdbce-caaa-4INVALIDFORMATbBOIYSb4-70fc5c7748f8/staff")
                            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_event_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/9281b570-4d02-4096-9136-338a613c71cd")
                    .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);

        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<StaffTemplate>(body).unwrap();

        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
        assert_eq!(out.id, Uuid::from_str("9281b570-4d02-4096-9136-338a613c71cd").unwrap());
    }

    #[actix_web::test]
    async fn get_event_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/918ab570-adb3-4c9d-9136-338a613c71cd")
                    .to_request();
        let res = test::call_service(&app, req).await;

        // Staff does not exist
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891INVALIDFORMAT4-zzzyzc7748f8/staff/918ab570-adb3-4c9d-9136-338a613c71cd")
                    .to_request();
        let res = test::call_service(&app, req).await;

        // Invalid UUID format.
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_update_delete_event_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "role": "basic"
        });

        let req = test::TestRequest::post()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff")
                    .set_form(data)
                    .to_request();
        
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::CREATED);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<StaffTemplate>(body).unwrap();
        assert_eq!(out.user.id, Uuid::from_str("0465041f-fe64-461f-9f71-71e3b97ca85f").unwrap());
        assert_eq!(out.company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());

        let staff_id = out.id;

        let data = json!({
            "role": "organizer",
        });

        let req = test::TestRequest::patch()
                                .uri(format!("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}", staff_id.to_string()).as_str())
                                .set_form(data)
                                .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::OK);
        let body_bytes = test::read_body(res).await;
        let body = str::from_utf8(body_bytes.borrow()).unwrap();
        let out = serde_json::from_str::<StaffTemplate>(body).unwrap();
        assert_eq!(out.user.id, Uuid::from_str("0465041f-fe64-461f-9f71-71e3b97ca85f").unwrap());
        assert_eq!(out.company.id, Uuid::from_str("b5188eda-528d-48d4-8cee-498e0971f9f5").unwrap());
        assert_eq!(out.event_id, Uuid::from_str("b71fd7ce-c891-410a-9bb4-70fc5c7748f8").unwrap());
        assert_eq!(out.role, StaffLevel::Organizer);

        let req = test::TestRequest::delete()
                    .uri(format!("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}", staff_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
        
        let req = test::TestRequest::delete()
                    .uri(format!("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff/{}", staff_id.to_string()).as_str())
                    .to_request();
        let res = test::call_service(&app, req).await;
        // Duplicate delete
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_event_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        
        let data = json!({
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "role": "basic"
        });

        let req = test::TestRequest::post()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/staff")
                    .set_form(data)
                    .to_request();
        
        let res = test::call_service(&app, req).await;

        // Missing user_id in form data
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);

        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "role": "basic"
        });

        let req = test::TestRequest::post()
                    .uri("/event/baafdece-c291-410a-9bb4-70fc5c7748f8/staff")
                    .set_form(data)
                    .to_request();
        
        let res = test::call_service(&app, req).await;

        // Non-existent event
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);

        let data = json!({
            "user_id": "0465041f-fe64-461f-9f71-71e3b97ca85f",
            "company_id": "b5188eda-528d-48d4-8cee-498e0971f9f5",
            "role": "basic"
        });

        let req = test::TestRequest::post()
                    .uri("/event/gginvalidUUIDBOIYZZZ91-410a-9bb4-70fc5c7748f8/staff")
                    .set_form(data)
                    .to_request();
        
        let res = test::call_service(&app, req).await;

        // Invalid UUID
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }
    
    #[actix_web::test]
    async fn get_all_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_not_accepted_assigned_staff() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_not_accepted_assigned_staff_errors() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_companies_per_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_companies_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_comapnies_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company_duplicate() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }


    #[actix_web::test]
    async fn update_associated_company_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_non_existent_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_non_existent_company() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_non_existent_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_employment_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_timesheet() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_non_existent_timesheet() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn get_timesheet_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_non_existent_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_duplicate() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_non_existent_event() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_non_existent_employment() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_invalid_uuid_format() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_empty_data() {
        let app = test::init_service(App::new().configure(organization::initialize::configure_app)).await;
        todo!()
    }
}