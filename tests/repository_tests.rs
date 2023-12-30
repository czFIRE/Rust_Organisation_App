#[cfg(test)]
pub mod user_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use sqlx::PgPool;

    use organization_app::{
        common::DbResult,
        models::{Gender, UserRole, UserStatus},
        repositories::{
            repository::DbRepository,
            user::{
                models::{NewUser, UserData},
                user_repo::UserRepository,
            },
        },
    };
    use uuid::uuid;

    #[sqlx::test(fixtures("users"))]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let new_user_data = NewUser {
            name: "Test User".to_string(),
            email: "hehe@mail.muni.cz".to_string(),
            birth: NaiveDate::from_ymd_opt(1997, 9, 15).unwrap(),
            gender: Gender::Other,
            role: UserRole::Admin,
        };

        let new_user = user_repo
            .create(new_user_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_user.name, new_user_data.name);
        assert_eq!(new_user.email, new_user_data.email);
        assert_eq!(new_user.birth, new_user_data.birth);
        assert_eq!(new_user.gender, new_user_data.gender);
        assert_eq!(new_user.role, new_user_data.role);
        assert_eq!(new_user.status, UserStatus::Available);

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
        let time_difference_created = time - new_user.created_at;
        let time_difference_edited = time - new_user.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert!(new_user.deleted_at.is_none());

        assert_eq!(
            new_user.avatar_url,
            Some("img/default/user.jpg".to_string())
        );

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

        let user = user_repo
            .read_one(user_id)
            .await
            .expect("Read should succeed");

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, "Dave Null");
        assert_eq!(user.email, "dave@null.com");
        assert_eq!(user.birth, NaiveDate::from_ymd_opt(1996, 6, 23).unwrap());
        assert_eq!(user.gender, Gender::Male);
        assert_eq!(user.role, UserRole::Admin);
        assert_eq!(user.avatar_url, Some("dave.jpg".to_string()));
        assert_eq!(user.status, UserStatus::Available);

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

        // Correct update

        {
            let user = user_repo
                .read_one(user_id)
                .await
                .expect("Read should succeed");

            let new_user_data = UserData {
                name: Some("Test User".to_string()),
                email: Some("hehe@mail.muni.cz".to_string()),
                birth: Some(NaiveDate::from_ymd_opt(1997, 9, 15).unwrap()),
                gender: Some(Gender::Other),
                role: Some(UserRole::Admin),
                avatar_url: Some("hehe.jpg".to_string()),
            };

            let updated_user = user_repo
                .update_user(user_id, new_user_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_user.id, user.id);
            assert_eq!(updated_user.name, new_user_data.name.unwrap());
            assert_eq!(updated_user.email, new_user_data.email.unwrap());
            assert_eq!(updated_user.birth, new_user_data.birth.unwrap());
            assert_eq!(updated_user.avatar_url, new_user_data.avatar_url);
            assert_eq!(updated_user.gender, new_user_data.gender.unwrap());
            assert_eq!(updated_user.role, new_user_data.role.unwrap());

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - updated_user.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_user.deleted_at.is_none());
        }

        // All are none

        {
            let new_user_data = UserData {
                name: None,
                email: None,
                birth: None,
                gender: None,
                role: None,
                avatar_url: None,
            };

            let _updated_user = user_repo
                .update_user(user_id, new_user_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d9");

            let new_user_data = UserData {
                name: Some("Test User".to_string()),
                email: None,
                birth: None,
                gender: None,
                role: None,
                avatar_url: None,
            };

            let _updated_user = user_repo
                .update_user(user_id, new_user_data)
                .await
                .expect_err("Update should fail - non existent user");
        }

        // Already deleted

        {
            let user = user_repo
                .read_one(user_id)
                .await
                .expect("Read should succeed");

            assert!(user.deleted_at.is_none());

            user_repo
                .delete_user(user_id)
                .await
                .expect("Delete should succeed");

            let deleted_user = user_repo
                .read_one(user_id)
                .await
                .expect("Read should succeed");

            assert!(deleted_user.deleted_at.is_some());

            let new_user_data = UserData {
                name: Some("Test User".to_string()),
                email: None,
                birth: None,
                gender: None,
                role: None,
                avatar_url: None,
            };

            let _updated_user = user_repo
                .update_user(user_id, new_user_data)
                .await
                .expect_err("Update should fail - already deleted user");
        }

        user_repo.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        {
            let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

            let user = user_repo.read_one(user_id).await.unwrap();

            assert!(user.deleted_at.is_none());

            user_repo.delete_user(user_id).await.unwrap();

            let new_user = user_repo.read_one(user_id).await.unwrap();

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - new_user.edited_at;
            let time_difference_deleted = time - new_user.deleted_at.unwrap();

            assert!(time_difference_edited.num_seconds() < 2);
            assert!(time_difference_deleted.num_seconds() < 2);
        }

        // delete on already deleted user

        {
            let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

            let user = user_repo.read_one(user_id).await.unwrap();

            assert!(user.deleted_at.is_some());

            user_repo
                .delete_user(user_id)
                .await
                .expect_err("Repository should return error on deleting an already deleted user");
        }

        // delete on non-existing user

        {
            let user_id = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d9");

            user_repo
                .delete_user(user_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing user");
        }

        user_repo.disconnect().await;

        Ok(())
    }
}

#[cfg(test)]
pub mod company_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDateTime, Utc};
    use organization_app::{
        common::DbResult,
        repositories::{
            company::{
                company_repo::CompanyRepository,
                models::{AddressData, CompanyData, CompanyFilter, NewCompany},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    #[sqlx::test(fixtures("companies"))]
    async fn create_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        let company_data = NewCompany {
            name: "Test Company".to_string(),
            description: Some("Test Description".to_string()),
            phone: "123456789".to_string(),
            email: "test@company.com".to_string(),
            website: Some("test.com".to_string()),
            crn: "CRN_123".to_string(),
            vatin: "VATIN_123".to_string(),
        };

        let address_data = AddressData {
            country: "Czech Republic".to_string(),
            region: "Moravia".to_string(),
            city: "Brno".to_string(),
            street: "Botanicka".to_string(),
            postal_code: "12345".to_string(),
            street_number: "68".to_string(),
        };

        let new_company = company_repo
            .create(company_data.clone(), address_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_company.name, company_data.name);
        assert_eq!(new_company.description, company_data.description);
        assert_eq!(new_company.phone, company_data.phone);
        assert_eq!(new_company.email, company_data.email);
        assert_eq!(
            new_company.avatar_url,
            Some("img/default/company.jpg".to_string()),
        );
        assert_eq!(new_company.website, company_data.website);
        assert_eq!(new_company.crn, company_data.crn);
        assert_eq!(new_company.vatin, company_data.vatin);

        assert_eq!(new_company.country, address_data.country);
        assert_eq!(new_company.region, address_data.region);
        assert_eq!(new_company.city, address_data.city);
        assert_eq!(new_company.street, address_data.street);
        assert_eq!(new_company.postal_code, address_data.postal_code);
        assert_eq!(new_company.street_number, address_data.street_number);

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

        let time_difference_created = time - new_company.created_at;
        let time_difference_edited = time - new_company.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert!(new_company.deleted_at.is_none());

        company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("companies"))]
    async fn read_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one(company_id)
                .await
                .expect("Read should succeed");

            assert_eq!(company.id, company_id);
            assert_eq!(company.name, "AMD");
            assert_eq!(
                company.description,
                Some("Advanced Micro Devices, Inc.".to_string())
            );
            assert_eq!(company.website, Some("https://amd.com".to_string()));
            assert_eq!(company.crn, "crn_amd".to_string());
            assert_eq!(company.vatin, "vatin_amd".to_string());
            assert_eq!(company.phone, "+1 408-749-4000");
            assert_eq!(company.email, "info@amd.com");

            let company_extended = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert_eq!(company_extended.company_id, company_id);
            assert_eq!(company_extended.name, "AMD");

            assert_eq!(company_extended.country, "United States".to_string());
            assert_eq!(company_extended.region, "CA".to_string());
            assert_eq!(company_extended.city, "Santa Clara".to_string());
            assert_eq!(company_extended.street, "Augustine Drive".to_string());
            assert_eq!(company_extended.postal_code, "95054".to_string());
            assert_eq!(company_extended.street_number, "2485".to_string());
        }

        company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("companies"))]
    async fn read_all_companies_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);
        {
            let filter = CompanyFilter {
                limit: Some(2),
                offset: Some(2),
            };

            let companies_ = company_repo
                .read_all(filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(companies_.len(), 1);

            let filter = CompanyFilter {
                limit: None,
                offset: None,
            };

            let companies = company_repo
                .read_all(filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(companies.len(), 3);

            let company1 = &companies[0];

            assert_eq!(company1.name, "AMD");

            let company2 = &companies[1];

            assert_eq!(company2.name, "ReportLab");

            let company3 = &companies[2];

            assert_eq!(company3.name, "Prusa Research");
        }

        company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("companies"))]
    async fn update_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        // Correct update

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            let company_data = CompanyData {
                name: Some("Test Company".to_string()),
                description: Some("Test Description".to_string()),
                phone: Some("123456789".to_string()),
                email: Some("test@test.com".to_string()),
                website: Some("test.com".to_string()),
                crn: Some("CRN_123".to_string()),
                vatin: Some("VATIN_123".to_string()),
                avatar_url: Some("test.jpg".to_string()),
            };

            let updated_company = company_repo
                .update(company_id, company_data.clone(), None)
                .await
                .expect("Update should succeed");

            assert_eq!(updated_company.company_id, company.company_id);
            assert_eq!(updated_company.name, company_data.name.unwrap());
            assert_eq!(updated_company.description, company_data.description);
            assert_eq!(updated_company.phone, company_data.phone.unwrap());
            assert_eq!(updated_company.email, company_data.email.unwrap());
            assert_eq!(updated_company.avatar_url, company_data.avatar_url);
            assert_eq!(updated_company.website, company_data.website);
            assert_eq!(updated_company.crn, company_data.crn.unwrap());
            assert_eq!(updated_company.vatin, company_data.vatin.unwrap());

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - updated_company.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_company.deleted_at.is_none());

            assert_eq!(updated_company.country, company.country);
        }

        // update address

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            let company_data = CompanyData {
                name: None,
                description: None,
                phone: None,
                email: None,
                website: None,
                crn: None,
                vatin: None,
                avatar_url: None,
            };

            let address_data = AddressData {
                country: "Czech Republic".to_string(),
                region: "Moravia".to_string(),
                city: "Brno".to_string(),
                street: "Botanicka".to_string(),
                postal_code: "12345".to_string(),
                street_number: "68".to_string(),
            };

            let updated_company = company_repo
                .update(company_id, company_data, Some(address_data.clone()))
                .await
                .expect("Update should succeed");

            assert_eq!(updated_company.company_id, company.company_id);
            assert_eq!(updated_company.name, company.name);
            assert_eq!(updated_company.description, company.description);
            assert_eq!(updated_company.phone, company.phone);
            assert_eq!(updated_company.email, company.email);
            assert_eq!(updated_company.avatar_url, company.avatar_url);
            assert_eq!(updated_company.website, company.website);
            assert_eq!(updated_company.crn, company.crn);
            assert_eq!(updated_company.vatin, company.vatin);

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - updated_company.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_company.deleted_at.is_none());

            assert_eq!(updated_company.country, address_data.country);
            assert_eq!(updated_company.region, address_data.region);
            assert_eq!(updated_company.city, address_data.city);
            assert_eq!(updated_company.street, address_data.street);
            assert_eq!(updated_company.postal_code, address_data.postal_code);
            assert_eq!(updated_company.street_number, address_data.street_number);
        }

        // All are none

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            let company_data = CompanyData {
                name: None,
                description: None,
                phone: None,
                email: None,
                website: None,
                crn: None,
                vatin: None,
                avatar_url: None,
            };

            let updated_company = company_repo
                .update(company_id, company_data.clone(), None)
                .await
                .expect_err("Update should fail - nothing to update");
        }

        // Non existent

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f9");

            let company_data = CompanyData {
                name: Some("Test Company".to_string()),
                description: Some("Test Description".to_string()),
                phone: Some("123456789".to_string()),
                email: None,
                website: None,
                crn: None,
                vatin: None,
                avatar_url: None,
            };

            let updated_company = company_repo
                .update(company_id, company_data.clone(), None)
                .await
                .expect_err("Update should fail - non existent company");
        }

        // Already deleted

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(company.deleted_at.is_none());

            company_repo
                .delete(company_id)
                .await
                .expect("Delete should succeed");

            let deleted_company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(deleted_company.deleted_at.is_some());

            let company_data = CompanyData {
                name: Some("Test Company".to_string()),
                description: Some("Test Description".to_string()),
                phone: Some("123456789".to_string()),
                email: None,
                website: None,
                crn: None,
                vatin: None,
                avatar_url: None,
            };

            let updated_company = company_repo
                .update(company_id, company_data.clone(), None)
                .await
                .expect_err("Update should fail - already deleted company");
        }

        company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("companies"))]
    async fn delete_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(company.deleted_at.is_none());

            company_repo.delete(company_id).await.unwrap();

            let new_company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - new_company.edited_at;
            let time_difference_deleted = time - new_company.deleted_at.unwrap();

            assert!(time_difference_edited.num_seconds() < 2);
            assert!(time_difference_deleted.num_seconds() < 2);
        }

        // delete on already deleted company

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(company.deleted_at.is_some());

            company_repo.delete(company_id).await.expect_err(
                "Repository should return error on deleting an already deleted company",
            );
        }

        // delete on non-existing company

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f9");

            company_repo
                .delete(company_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing company");
        }

        company_repo.disconnect().await;

        Ok(())
    }
}

#[cfg(test)]
pub mod event_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use organization_app::{
        common::DbResult,
        repositories::{
            event::{
                event_repo::EventRepository,
                models::{EventData, EventFilter, NewEvent},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("events"))]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        let new_event_data = NewEvent {
            name: "Test Event".to_string(),
            description: Some("Test Description".to_string()),
            website: Some("test.com".to_string()),
            start_date: NaiveDate::from_ymd_opt(2021, 9, 15).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2021, 9, 16).unwrap(),
        };

        let new_event = event_repo
            .create(new_event_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_event.name, new_event_data.name);
        assert_eq!(new_event.description, new_event_data.description);
        assert_eq!(new_event.website, new_event_data.website);
        assert_eq!(new_event.start_date, new_event_data.start_date);
        assert_eq!(new_event.end_date, new_event_data.end_date);

        assert_eq!(
            new_event.avatar_url,
            Some("img/default/event.jpg".to_string())
        );

        assert!(new_event.accepts_staff);

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
        let time_difference_created = time - new_event.created_at;
        let time_difference_edited = time - new_event.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert!(new_event.deleted_at.is_none());

        event_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("events"))]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

        let event = event_repo
            .read_one(event_id)
            .await
            .expect("Read should succeed");

        assert_eq!(event.id, event_id);
        assert_eq!(event.name, "Woodstock");
        assert_eq!(
            event.description,
            Some("A legendary music festival.".to_string())
        );
        assert_eq!(event.website, Some("https://woodstock.com".to_string()));
        assert_eq!(
            event.start_date,
            NaiveDate::from_ymd_opt(1969, 8, 15).unwrap()
        );
        assert_eq!(
            event.end_date,
            NaiveDate::from_ymd_opt(1969, 8, 18).unwrap()
        );
        assert_eq!(event.avatar_url, Some("woodstock.png".to_string()));
        assert!(event.accepts_staff);

        event_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("events"))]
    async fn read_all(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        {
            let filter = EventFilter {
                limit: None,
                offset: None,
                accepts_staff: None,
            };

            let events = event_repo
                .read_all(filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(events.len(), 2);

            let event = &events[0];

            assert_eq!(event.name, "Woodstock");

            let event = &events[1];

            assert_eq!(event.name, "PyCon");
        }

        {
            let filter = EventFilter {
                limit: None,
                offset: None,
                accepts_staff: Some(false),
            };

            let events = event_repo
                .read_all(filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(events.len(), 1);

            let event = &events[0];

            assert_eq!(event.name, "PyCon");
        }

        event_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("events"))]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

        // Correct update

        {
            let event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            let new_event_data = EventData {
                name: Some("Test Event".to_string()),
                description: Some("Test Description".to_string()),
                website: Some("test.com".to_string()),
                start_date: Some(NaiveDate::from_ymd_opt(2021, 9, 15).unwrap()),
                end_date: Some(NaiveDate::from_ymd_opt(2021, 9, 16).unwrap()),
                avatar_url: Some("test.jpg".to_string()),
            };

            let updated_event = event_repo
                .update(event_id, new_event_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_event.id, event.id);
            assert_eq!(updated_event.name, new_event_data.name.unwrap());
            assert_eq!(updated_event.description, new_event_data.description);
            assert_eq!(updated_event.website, new_event_data.website);
            assert_eq!(updated_event.start_date, new_event_data.start_date.unwrap());
            assert_eq!(updated_event.end_date, new_event_data.end_date.unwrap());

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - updated_event.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_event.deleted_at.is_none());
        }

        // All are none

        {
            let new_event_data = EventData {
                name: None,
                description: None,
                website: None,
                start_date: None,
                end_date: None,
                avatar_url: None,
            };

            let _updated_event = event_repo
                .update(event_id, new_event_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f9");

            let new_event_data = EventData {
                name: Some("Test Event".to_string()),
                description: None,
                website: None,
                start_date: None,
                end_date: None,
                avatar_url: None,
            };

            let _updated_event = event_repo
                .update(event_id, new_event_data)
                .await
                .expect_err("Update should fail - non existent event");
        }

        // Already deleted

        {
            let event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            assert!(event.deleted_at.is_none());

            event_repo
                .delete(event_id)
                .await
                .expect("Delete should succeed");

            let deleted_event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            assert!(deleted_event.deleted_at.is_some());

            let new_event_data = EventData {
                name: Some("Test Event".to_string()),
                description: None,
                website: None,
                start_date: None,
                end_date: None,
                avatar_url: None,
            };

            let _updated_event = event_repo
                .update(event_id, new_event_data)
                .await
                .expect_err("Update should fail - already deleted event");
        }

        event_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("events"))]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            let event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            assert!(event.deleted_at.is_none());

            event_repo.delete(event_id).await.unwrap();

            let new_event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - new_event.edited_at;
            let time_difference_deleted = time - new_event.deleted_at.unwrap();

            assert!(time_difference_edited.num_seconds() < 2);
            assert!(time_difference_deleted.num_seconds() < 2);
        }

        // delete on already deleted event

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            let event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            assert!(event.deleted_at.is_some());

            event_repo
                .delete(event_id)
                .await
                .expect_err("Repository should return error on deleting an already deleted event");
        }

        // delete on non-existing event

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f9");

            event_repo
                .delete(event_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing event");
        }

        event_repo.disconnect().await;

        Ok(())
    }
}

#[cfg(test)]
pub mod associated_company_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDateTime, Utc};
    use organization_app::{
        common::DbResult,
        models::Association,
        repositories::{
            associated_company::{
                associated_company_repo::AssociatedCompanyRepository,
                models::{AssociatedCompanyData, AssociatedCompanyFilter, NewAssociatedCompany},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    #[sqlx::test(fixtures("associated_company"))]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let associated_company_data = NewAssociatedCompany {
            event_id: uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8"),
            company_id: uuid!("71fa27d6-6f00-4ad0-8902-778e298aaed2"),
            association_type: Association::Media,
        };

        let new_associated_company = associated_company_repo
            .create(associated_company_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(
            new_associated_company.event_id,
            associated_company_data.event_id
        );
        assert_eq!(
            new_associated_company.company_id,
            associated_company_data.company_id
        );
        assert_eq!(
            new_associated_company.association_type,
            associated_company_data.association_type
        );

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

        let time_difference_created = time - new_associated_company.created_at;
        let time_difference_edited = time - new_associated_company.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);

        assert!(new_associated_company.deleted_at.is_none());

        associated_company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("associated_company"))]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
        let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

        let associated_company = associated_company_repo
            .read_one(company_id, event_id)
            .await
            .expect("Read should succeed");

        assert_eq!(associated_company.event.id, event_id);
        assert_eq!(associated_company.company.id, company_id);
        assert_eq!(associated_company.association_type, Association::Organizer);

        associated_company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("associated_company"))]
    async fn read_all(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        // Read all

        {
            let filter = AssociatedCompanyFilter {
                limit: None,
                offset: None,
            };

            let associated_companies = associated_company_repo
                .read_all(filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(associated_companies.len(), 4);

            let associated_company = &associated_companies[0];

            assert_eq!(associated_company.company.name, "AMD");

            let associated_company = &associated_companies[1];

            assert_eq!(associated_company.company.name, "ReportLab");

            let associated_company = &associated_companies[2];

            assert_eq!(associated_company.company.name, "Prusa Research");

            let associated_company = &associated_companies[3];

            assert_eq!(associated_company.company.name, "AMD");
        }

        // Read all for an event

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            let filter = AssociatedCompanyFilter {
                limit: None,
                offset: None,
            };

            let associated_companies = associated_company_repo
                .read_all_companies_for_event(event_id, filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(associated_companies.len(), 2);

            let associated_company = &associated_companies[0];

            assert_eq!(associated_company.company.name, "AMD");

            let associated_company = &associated_companies[1];

            assert_eq!(associated_company.company.name, "ReportLab");
        }

        // Read all for a company

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");

            let filter = AssociatedCompanyFilter {
                limit: None,
                offset: None,
            };

            let associated_companies = associated_company_repo
                .read_all_events_for_company(company_id, filter)
                .await
                .expect("Read all should succeed");

            assert_eq!(associated_companies.len(), 2);

            let associated_company = &associated_companies[0];

            assert_eq!(associated_company.event.name, "Woodstock");

            let associated_company = &associated_companies[1];

            assert_eq!(associated_company.event.name, "PyCon");
        }

        associated_company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("associated_company"))]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
        let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

        // Correct update

        {
            let associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            let new_associated_company_data = AssociatedCompanyData {
                association_type: Some(Association::Media),
            };

            let updated_associated_company = associated_company_repo
                .update(company_id, event_id, new_associated_company_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(
                updated_associated_company.event_id,
                associated_company.event.id
            );
            assert_eq!(
                updated_associated_company.company_id,
                associated_company.company.id
            );
            assert_eq!(
                updated_associated_company.association_type,
                new_associated_company_data.association_type.unwrap()
            );

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - updated_associated_company.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_associated_company.deleted_at.is_none());
        }

        // All are none

        {
            let new_associated_company_data = AssociatedCompanyData {
                association_type: None,
            };

            let _updated_associated_company = associated_company_repo
                .update(company_id, event_id, new_associated_company_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f9");

            let new_associated_company_data = AssociatedCompanyData {
                association_type: Some(Association::Media),
            };

            let _updated_associated_company = associated_company_repo
                .update(company_id, event_id, new_associated_company_data)
                .await
                .expect_err("Update should fail - non existent associated company");
        }

        // Already deleted

        {
            let associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            assert!(associated_company.deleted_at.is_none());

            associated_company_repo
                .delete(company_id, event_id)
                .await
                .expect("Delete should succeed");

            let deleted_associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            assert!(deleted_associated_company.deleted_at.is_some());

            let new_associated_company_data = AssociatedCompanyData {
                association_type: Some(Association::Media),
            };

            let _updated_associated_company = associated_company_repo
                .update(company_id, event_id, new_associated_company_data)
                .await
                .expect_err("Update should fail - already deleted associated company");
        }

        associated_company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("associated_company"))]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            let associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            assert!(associated_company.deleted_at.is_none());

            associated_company_repo
                .delete(company_id, event_id)
                .await
                .unwrap();

            let new_associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - new_associated_company.edited_at;
            let time_difference_deleted = time - new_associated_company.deleted_at.unwrap();

            assert!(time_difference_edited.num_seconds() < 2);
            assert!(time_difference_deleted.num_seconds() < 2);
        }

        // delete on already deleted associated company

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            let associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            assert!(associated_company.deleted_at.is_some());

            associated_company_repo
                .delete(company_id, event_id)
                .await
                .expect_err(
                "Repository should return error on deleting an already deleted associated company",
            );
        }

        // delete on non-existing associated company

        {
            let company_id = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f9");
            let event_id = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");

            associated_company_repo
                .delete(company_id, event_id)
                .await
                .expect_err(
                    "Repository should return error on deleting a non-existing associated company",
                );
        }

        associated_company_repo.disconnect().await;

        Ok(())
    }
}

// needs user, company
#[cfg(test)]
pub mod employment_repo_tests {
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("employments"))]
    async fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    async fn read(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    async fn read_all_per_user(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    pub fn read_all_per_company(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    pub fn read_all_subordinates(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    pub fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("employments"))]
    pub fn delete(_pool: PgPool) {
        todo!()
    }
}

// needs user, company, event
#[cfg(test)]
pub mod event_staff_repo_tests {
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("event_staff"))]
    async fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("event_staff"))]
    async fn read(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("event_staff"))]
    async fn read_all_per_event(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("event_staff"))]
    async fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("event_staff"))]
    async fn delete(_pool: PgPool) {
        todo!()
    }
}

// event, event_staff
#[cfg(test)]
pub mod task_repo_tests {
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("task"))]
    async fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("task"))]
    async fn read_one(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("task"))]
    async fn read_all_per_event(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("task"))]
    async fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("task"))]
    async fn delete(_pool: PgPool) {
        todo!()
    }
}

// needs event_staff, task
#[cfg(test)]
pub mod assigned_staff_repo_tests {
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("assigned_staff"))]
    pub fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("assigned_staff"))]
    pub fn read_one(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("assigned_staff"))]
    pub fn read_all_per_task(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("assigned_staff"))]
    pub fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("assigned_staff"))]
    pub fn delete(_pool: PgPool) {
        todo!()
    }
}

// needs event, task, user
#[cfg(test)]
pub mod comment_repo_tests {
    use sqlx::PgPool;
    use uuid::{uuid, Uuid};

    #[sqlx::test(fixtures("comments"))]
    async fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("comments"))]
    async fn read_one(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("comments"))]
    async fn read_all_per_event(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("comments"))]
    async fn read_all_per_task(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("comments"))]
    async fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("comments"))]
    async fn delete(_pool: PgPool) {
        todo!()
    }
}
