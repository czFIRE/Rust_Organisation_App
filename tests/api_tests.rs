#[cfg(test)]
mod api_tests {
    use std::borrow::Borrow;

    use actix_web::http::{Method, self};
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};
    use chrono::{NaiveDate, Utc, TimeZone};
    use organization::models::{UserRole, TaskPriority};
    use organization::templates::company::{CompaniesTemplate, CompanyTemplate};
    use organization::templates::event::{EventsTemplate, EventTemplate};
    use organization::templates::task::{TasksTemplate, TaskTemplate};
    use organization::templates::user::UserTemplate;
    use serde_json::json;
    use organization::{self, templates};
    use uuid::Uuid;
    use std::str::{self, FromStr};

    struct Error {
        error: String,
    }

    #[actix_web::test]
    async fn index_get() {
        let app = test::init_service(App::new().service(organization::handlers::index::index)).await;
        let req = test::TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn create_user() {
        let app = test::init_service(App::new().service(organization::handlers::user::create_user)).await;
        // let req = test::TestRequest::default().insert_header(ContentType::plaintext()).to_request();
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
        let app = test::init_service(App::new().service(organization::handlers::user::create_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::get_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::get_user)).await;
        
        let req = test::TestRequest::get()
                            .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn user_get_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::user::get_user)).await;
        
        let req = test::TestRequest::get()
                            .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn patch_existing_user() {
        let app = test::init_service(App::new().service(organization::handlers::user::update_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::update_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::update_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::delete_user)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::user::delete_user)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/35341253-da20-40b6-96d8-ce069b1ba5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_user() {
        let app = test::init_service(App::new().service(organization::handlers::user::delete_user)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/35341289-d420-40b6-96d8-ce069b1ba5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_user_invalid_uuid_format () {
        let app = test::init_service(App::new().service(organization::handlers::user::delete_user)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/Sleepyhead-d420-zzz6-ygd8-5d4")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }


    #[actix_web::test]
    async fn get_user_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn upload_user_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn remove_user_avatar() {
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_companies() {
        let app = test::init_service(App::new().service(organization::handlers::company::get_all_companies)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::get_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::get_company)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company/b548eed1-538d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_company_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::company::get_company)).await;
        
        let req = test::TestRequest::get()
                            .uri("/company/b548eed1-sleepy-head-123zzz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_company() {
        let app = test::init_service(App::new().service(organization::handlers::company::create_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::create_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::update_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::update_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::update_company)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::company::update_company)).await;
        
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
        let app= test::init_service(App::new().service(organization::handlers::company::delete_company)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_company() {
        let app= test::init_service(App::new().service(organization::handlers::company::delete_company)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-528d-48d4-8cee-498e0971f9f5")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_company_invalid_uuid_format() {
        let app= test::init_service(App::new().service(organization::handlers::company::delete_company)).await;

        let req = test::TestRequest::delete()
                            .uri("/company/b5188eda-sleepy-head-123zzz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_company_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn upload_company_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn remove_company_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn get_events() {
        let app = test::init_service(App::new().service(organization::handlers::event::get_events)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::event::get_event)).await;
        
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
        let app = test::init_service(App::new().service(organization::handlers::event::get_event)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event/a71cd75e-a811-410a-9bb4-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_event_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::event::get_event)).await;
        
        let req = test::TestRequest::get()
                            .uri("/event/a71cd75e-sleepy-head-111z3zz")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_event() {
        let app = test::init_service(App::new().service(organization::handlers::event::create_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::create_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::update_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::update_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::update_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::update_event)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event::delete_event)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn delete_non_existent_event() {
        let app = test::init_service(App::new().service(organization::handlers::event::delete_event)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b7afddce-c8fe-45aa-a12c-70fc5c7748f8")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn delete_event_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::event::delete_event)).await;

        let req = test::TestRequest::delete()
                            .uri("/event/b71fd7ce-im-rusty-boizzz-1")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_event_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn upload_event_avatar() {
        todo!()
    }

    #[actix_web::test]
    async fn remove_event_avatar() {
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_tasks_per_event() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_tasks)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_tasks)).await;

        let req = test::TestRequest::get()
                    .uri("/event/ba1cd734-c571-42ea-9bb4-70fc5c7748f8/task")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_all_tasks_per_event_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_tasks)).await;

        let req = test::TestRequest::get()
                    .uri("/event/ba1cd734-tasks-boi-they-sure-are-difficult-are-they-not?")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn get_one_task() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_task)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_task)).await;

        let req = test::TestRequest::get()
                    .uri("/event/baaadfcf-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fe31-4aac-b767-100d18a8877b")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_non_existent_task() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_task)).await;

        let req = test::TestRequest::get()
                    .uri("/event/b71fd7ce-c891-410a-9bb4-70fc5c7748f8/task/7ae0c017-fffe-4aac-b767-1aacca8877b")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn get_one_task_invalid_uuid_format() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::get_event_task)).await;

        let req = test::TestRequest::get()
                    .uri("/event/sleepy-head-I-am?-70fc5c7748f8/task/nowaythiscanbeavalidUUIDbrotherrr")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn create_task() {
        let app = test::init_service(App::new().service(organization::handlers::event_task::create_task)).await;

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
        let app = test::init_service(App::new().service(organization::handlers::event_task::create_task)).await;

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
        todo!()
    }

    #[actix_web::test]
    async fn patch_non_existent_task() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_task_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_task_empty_data() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_task() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_non_existent_task() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_task_invalid_uuid_format() {
        todo!()
    }


    #[actix_web::test]
    async fn get_all_event_comments() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_event_comments_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_event_comments_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_comment_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_comment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn update_event_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn update_event_comment_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn update_event_comment_non_existent_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn update_event_comment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_task_comments() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_task_comments_non_existent_task() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_task_comments_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_task_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn create_task_comment_non_existent_task() {
        todo!()
    }

    #[actix_web::test]
    async fn create_task_comment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn update_task_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn update_task_comment_non_existent_task() {
        todo!()
    }

    #[actix_web::test]
    async fn update_task_comment_non_existent_comment() {
        todo!()
    }

    #[actix_web::test]
    async fn update_task_comment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employments_per_user() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employments_non_existent_user() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employments_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employment() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employment_non_existent_user() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employment_non_existent_company() {
        todo!()
    }

    #[actix_web::test]
    async fn get_employment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_subordinates() {
        todo!()
    }

    #[actix_web::test]
    async fn get_subordinates_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn create_employment() {
        todo!()
    }

    #[actix_web::test]
    async fn create_employment_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn update_emloyment() {
        todo!()
    }

    #[actix_web::test]
    async fn update_employment_errors() {
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_event_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_event_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn get_event_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn get_event_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_event_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_event_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_event_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_event_staff_errors() {
        todo!()
    }
    
    #[actix_web::test]
    async fn get_all_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn get_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn get_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn create_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn create_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn update_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn update_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_not_accepted_assigned_staff() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_not_accepted_assigned_staff_errors() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_companies_per_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_companies_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_associated_comapnies_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company() {
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company_duplicate() {
        todo!()
    }

    #[actix_web::test]
    async fn create_associated_company_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company() {
        todo!()
    }


    #[actix_web::test]
    async fn update_associated_company_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_non_existent_company() {
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn update_associated_company_empty_data() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_non_existent_company() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_associate_company_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_employment() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_non_existent_employment() {
        todo!()
    }

    #[actix_web::test]
    async fn get_all_timesheets_for_employment_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn get_timesheet() {
        todo!()
    }

    #[actix_web::test]
    async fn get_non_existent_timesheet() {
        todo!()
    }

    #[actix_web::test]
    async fn get_timesheet_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet() {
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_non_existent_employment() {
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_timesheet_duplicate() {
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet() {
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_non_existent_employment() {
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    pub async fn update_timesheet_empty_data() {
        todo!()
    }
}