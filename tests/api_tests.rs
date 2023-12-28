#[cfg(test)]
mod api_tests {
    use std::borrow::Borrow;

    use actix_web::http::{Method, self};
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};
    use organization::models::UserRole;
    use organization::templates::user::UserTemplate;
    use serde_json::json;
    use organization::{self, templates};
    use std::str;

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
        let req = test::TestRequest::post().uri("/user").set_json(user).to_request();
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
        let req = test::TestRequest::post().uri("/user").set_json(user.clone()).to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());

        let req = test::TestRequest::post().uri("/user").set_json(user).to_request();
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
                    .set_json(user_update)
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
                    .set_json(user_update)
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
                    .set_json(user_update)
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
                    .set_json(user_update)
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
    async fn get_deleted_user() {
        let app = test::init_service(App::new().service(organization::handlers::user::delete_user).service(organization::handlers::user::get_user)).await;

        let req = test::TestRequest::delete()
                    .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c")
                    .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

        let req = test::TestRequest::get()
                            .uri("/user/ac9bf689-a713-4b66-a3d0-41faaf0f8d0c")
                            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
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
        todo!()
    }

    #[actix_web::test]
    async fn get_existing_company() {
        todo!()
    }

    #[actix_web::test]
    async fn get_non_existing_company() {
        todo!()
    }

    #[actix_web::test]
    async fn get_company_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_company() {
        todo!()
    }

    #[actix_web::test]
    async fn create_duplicate_company() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_company() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_non_existent_company() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_company_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_company_empty_data() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_company() {
        todo!()
    }

    #[actix_web::test]
    async fn get_deleted_company() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_non_existent_company() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_company_invalid_uuid_format() {
        todo!()
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
        todo!()
    }

    #[actix_web::test]
    async fn get_existing_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_event_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event() {
        todo!()
    }

    #[actix_web::test]
    async fn create_event_duplicate() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_event() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_event_invalid_uuid_format() {
        todo!()
    }

    #[actix_web::test]
    async fn patch_event_empty_data() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_event() {
        todo!()
    }

    #[actix_web::test]
    async fn get_deleted_event() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_non_existent_event() {
        todo!()
    }

    #[actix_web::test]
    async fn delete_event_invalid_uuid_format() {
        todo!()
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


    
    //ToDo: Task test
    //ToDo: Comment test
    //ToDo: Employment test
    //ToDo: EventStaff test
    //ToDo: TaskStaff test
    //ToDo: AssociatedCompany test
    //ToDo: Timesheet test
}