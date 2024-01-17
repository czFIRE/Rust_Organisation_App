pub mod test_constants {
    use uuid::{uuid, Uuid};

    pub const COMPANY0_ID: Uuid = uuid!("b5188eda-528d-48d4-8cee-498e0971f9f5");
    pub const COMPANY1_ID: Uuid = uuid!("134d5286-5f55-4637-9b98-223a5820a464");
    pub const COMPANY2_ID: Uuid = uuid!("71fa27d6-6f00-4ad0-8902-778e298aaed2");

    pub const USER0_ID: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");
    pub const USER1_ID: Uuid = uuid!("0465041f-fe64-461f-9f71-71e3b97ca85f");
    pub const USER2_ID: Uuid = uuid!("ac9bf689-a713-4b66-a3d0-41faaf0f8d0c");
    pub const USER3_ID: Uuid = uuid!("51a01dbf-dcd5-43a0-809c-94ed8e61d420");

    pub const EVENT0_ID: Uuid = uuid!("b71fd7ce-c891-410a-9bb4-70fc5c7748f8");
    pub const EVENT1_ID: Uuid = uuid!("3f152d12-0bbd-429a-a9c5-28967d6370cc");

    pub const TIMESHEET0_ID: Uuid = uuid!("d47e8141-a77e-4d55-a2d5-4a77de24b6d0");
    pub const TIMESHEET1_ID: Uuid = uuid!("0f0f0ff5-0073-47cc-bd1f-540a04fee9ea");
    pub const TIMESHEET2_ID: Uuid = uuid!("c51e77aa-bd80-42c7-8b8a-003f018328f6");
    pub const TIMESHEET3_ID: Uuid = uuid!("8446b2ba-8223-4388-be5f-9efdfc4ea265");
    pub const TIMESHEET4_ID: Uuid = uuid!("a19a0ac6-3bd2-4ebd-bc8d-ec111ec9f705");
    pub const TIMESHEET5_ID: Uuid = uuid!("ced9f31c-8662-4812-9005-b8ae85d3b951");

    pub const EVENT_STAFF0_ID: Uuid = uuid!("9281b570-4d02-4096-9136-338a613c71cd");
    pub const EVENT_STAFF1_ID: Uuid = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd3");
    pub const EVENT_STAFF2_ID: Uuid = uuid!("aa7f3d0e-ab48-473b-ac69-b84cb74f34f7");

    pub const TASK0_ID: Uuid = uuid!("7ae0c017-fe31-4aac-b767-100d18a8877b");
    pub const TASK1_ID: Uuid = uuid!("bd9b422d-33c1-42a2-88bf-a56ce6cc55a6");

    pub const COMMENT0_ID: Uuid = uuid!("0d6cec6a-4fe8-4e44-bf68-e33de0ed121b");
    pub const COMMENT1_ID: Uuid = uuid!("daac23ec-fb36-434a-823b-49716ed2002c");

    // a delta for float comparisons
    pub const DELTA: f32 = 0.0000001;
}

#[cfg(test)]
pub mod user_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use sqlx::PgPool;

    use organization::{
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

    use crate::test_constants;

    #[sqlx::test(fixtures("users"), migrations = "migrations/no_seed")]
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

        assert_eq!(new_user.avatar_url, "img/default/user.jpg".to_string());

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"), migrations = "migrations/no_seed")]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id = test_constants::USER0_ID;

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
        assert_eq!(user.avatar_url, "dave.jpg".to_string());
        assert_eq!(user.status, UserStatus::Available);

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"), migrations = "migrations/no_seed")]
    async fn read_all(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        {
            let users = user_repo
                ._read_all()
                .await
                .expect("Read all should succeed");

            assert_eq!(users.len(), 3);

            let user1 = &users[0];

            assert_eq!(user1.name, "Dave Null");

            let user2 = &users[1];

            assert_eq!(user2.name, "Tana Smith");

            let user3 = &users[2];

            assert_eq!(user3.name, "John Doe");
        }

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id = test_constants::USER0_ID;

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
            assert_eq!(updated_user.avatar_url, new_user_data.avatar_url.unwrap());
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

            let _ = user_repo
                .read_one(user_id)
                .await
                .expect_err("Read should not succeed");

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

    #[sqlx::test(fixtures("users"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(arc_pool);

        {
            let user_id = test_constants::USER0_ID;

            let user = user_repo.read_one(user_id).await.unwrap();

            assert!(user.deleted_at.is_none());

            user_repo.delete_user(user_id).await.unwrap();

            let _ = user_repo.read_one(user_id).await.expect_err("Should fail.");
        }

        // delete on already deleted user

        {
            let user_id = test_constants::USER0_ID;

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
    use organization::{
        common::DbResult,
        repositories::{
            company::{
                company_repo::CompanyRepository,
                models::{AddressData, AddressUpdateData, CompanyData, CompanyFilter, NewCompany},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("companies"), migrations = "migrations/no_seed")]
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
            "img/default/company.jpg".to_string(),
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

    #[sqlx::test(fixtures("companies"), migrations = "migrations/no_seed")]
    async fn read_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        {
            let company_id = test_constants::COMPANY0_ID;

            let company = company_repo
                ._read_one(company_id)
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

    #[sqlx::test(fixtures("companies"), migrations = "migrations/no_seed")]
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

    #[sqlx::test(fixtures("companies"), migrations = "migrations/no_seed")]
    async fn update_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        // Correct update

        {
            let company_id = test_constants::COMPANY0_ID;

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

            let address_update = AddressUpdateData {
                city: None,
                region: None,
                postal_code: None,
                country: None,
                street: None,
                street_number: None,
            };

            let updated_company = company_repo
                .update(company_id, company_data.clone(), address_update)
                .await
                .expect("Update should succeed");

            assert_eq!(updated_company.company_id, company.company_id);
            assert_eq!(updated_company.name, company_data.name.unwrap());
            assert_eq!(updated_company.description, company_data.description);
            assert_eq!(updated_company.phone, company_data.phone.unwrap());
            assert_eq!(updated_company.email, company_data.email.unwrap());
            assert_eq!(updated_company.avatar_url, company_data.avatar_url.unwrap());
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
            let company_id = test_constants::COMPANY0_ID;

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

            let address_data = AddressUpdateData {
                country: Some("Czech Republic".to_string()),
                region: Some("Moravia".to_string()),
                city: Some("Brno".to_string()),
                street: Some("Botanicka".to_string()),
                postal_code: Some("12345".to_string()),
                street_number: Some("68".to_string()),
            };

            let updated_company = company_repo
                .update(company_id, company_data, address_data.clone())
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

            assert_eq!(updated_company.country, address_data.country.unwrap());
            assert_eq!(updated_company.region, address_data.region.unwrap());
            assert_eq!(updated_company.city, address_data.city.unwrap());
            assert_eq!(updated_company.street, address_data.street.unwrap());
            assert_eq!(
                updated_company.postal_code,
                address_data.postal_code.unwrap()
            );
            assert_eq!(
                updated_company.street_number,
                address_data.street_number.unwrap()
            );
        }

        // All are none

        {
            let company_id = test_constants::COMPANY0_ID;

            let _ = company_repo
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

            let address_update = AddressUpdateData {
                city: None,
                region: None,
                postal_code: None,
                country: None,
                street: None,
                street_number: None,
            };

            let _ = company_repo
                .update(company_id, company_data.clone(), address_update)
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

            let address_update = AddressUpdateData {
                city: None,
                region: None,
                postal_code: None,
                country: None,
                street: None,
                street_number: None,
            };

            let _ = company_repo
                .update(company_id, company_data.clone(), address_update)
                .await
                .expect_err("Update should fail - non existent company");
        }

        // Already deleted

        {
            let company_id = test_constants::COMPANY0_ID;

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(company.deleted_at.is_none());

            company_repo
                .delete(company_id)
                .await
                .expect("Delete should succeed");

            let _ = company_repo
                .read_one_extended(company_id)
                .await
                .expect_err("Read should not succeed");

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

            let address_update = AddressUpdateData {
                city: None,
                region: None,
                postal_code: None,
                country: None,
                street: None,
                street_number: None,
            };

            let _ = company_repo
                .update(company_id, company_data.clone(), address_update)
                .await
                .expect_err("Update should fail - already deleted company");
        }

        company_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("companies"), migrations = "migrations/no_seed")]
    async fn delete_company_test(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut company_repo = CompanyRepository::new(arc_pool);

        {
            let company_id = test_constants::COMPANY0_ID;

            let company = company_repo
                .read_one_extended(company_id)
                .await
                .expect("Read should succeed");

            assert!(company.deleted_at.is_none());

            company_repo.delete(company_id).await.unwrap();

            let _ = company_repo
                .read_one_extended(company_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted company

        {
            let company_id = test_constants::COMPANY0_ID;

            let _ = company_repo
                .read_one_extended(company_id)
                .await
                .expect_err("Read should not succeed");

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
    use organization::{
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
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("events"), migrations = "migrations/no_seed")]
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

        assert_eq!(new_event.avatar_url, "img/default/event.jpg".to_string());

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

    #[sqlx::test(fixtures("events"), migrations = "migrations/no_seed")]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        let event_id = test_constants::EVENT0_ID;

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
        assert_eq!(event.avatar_url, "woodstock.png".to_string());
        assert!(event.accepts_staff);

        event_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("events"), migrations = "migrations/no_seed")]
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

    #[sqlx::test(fixtures("events"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        let event_id = test_constants::EVENT0_ID;

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
                start_date: Some(NaiveDate::from_ymd_opt(2025, 9, 15).unwrap()),
                end_date: Some(NaiveDate::from_ymd_opt(2025, 9, 16).unwrap()),
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
            assert_eq!(updated_event.avatar_url, new_event_data.avatar_url.unwrap());

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

            let _ = event_repo
                .read_one(event_id)
                .await
                .expect_err("Read should not succeed");

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

    #[sqlx::test(fixtures("events"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_repo = EventRepository::new(arc_pool);

        {
            let event_id = test_constants::EVENT0_ID;

            let event = event_repo
                .read_one(event_id)
                .await
                .expect("Read should succeed");

            assert!(event.deleted_at.is_none());

            event_repo.delete(event_id).await.unwrap();

            let _ = event_repo
                .read_one(event_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted event

        {
            let event_id = test_constants::EVENT0_ID;

            let _ = event_repo
                .read_one(event_id)
                .await
                .expect_err("Read should not succeed");

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
    use organization::{
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

    use crate::test_constants;

    #[sqlx::test(fixtures("associated_company"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let associated_company_data = NewAssociatedCompany {
            event_id: test_constants::EVENT0_ID,
            company_id: test_constants::COMPANY2_ID,
            association_type: Association::Media,
        };

        let new_associated_company = associated_company_repo
            .create(associated_company_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(
            new_associated_company.event.id,
            associated_company_data.event_id
        );
        assert_eq!(
            new_associated_company.company.id,
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

    #[sqlx::test(fixtures("associated_company"), migrations = "migrations/no_seed")]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let company_id = test_constants::COMPANY0_ID;
        let event_id = test_constants::EVENT0_ID;

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

    #[sqlx::test(fixtures("associated_company"), migrations = "migrations/no_seed")]
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
                ._read_all(filter)
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
            let event_id = test_constants::EVENT0_ID;

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
            let company_id = test_constants::COMPANY0_ID;

            let filter = AssociatedCompanyFilter {
                limit: None,
                offset: None,
            };

            let associated_companies = associated_company_repo
                ._read_all_events_for_company(company_id, filter)
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

    #[sqlx::test(fixtures("associated_company"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        let company_id = test_constants::COMPANY0_ID;
        let event_id = test_constants::EVENT0_ID;

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
                updated_associated_company.event.id,
                associated_company.event.id
            );
            assert_eq!(
                updated_associated_company.company.id,
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

            let _deleted_associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect_err("Read should not succeed");

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

    #[sqlx::test(fixtures("associated_company"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut associated_company_repo = AssociatedCompanyRepository::new(arc_pool);

        {
            let company_id = test_constants::COMPANY0_ID;
            let event_id = test_constants::EVENT0_ID;

            let associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect("Read should succeed");

            assert!(associated_company.deleted_at.is_none());

            associated_company_repo
                .delete(company_id, event_id)
                .await
                .unwrap();

            let _deleted_associated_company = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted associated company

        {
            let company_id = test_constants::COMPANY0_ID;
            let event_id = test_constants::EVENT0_ID;

            let _ = associated_company_repo
                .read_one(company_id, event_id)
                .await
                .expect_err("Read should not succeed");

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
            let event_id = test_constants::EVENT0_ID;

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
    use std::sync::Arc;

    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use organization::{
        common::DbResult,
        models::{EmployeeLevel, EmploymentContract},
        repositories::{
            employment::{
                employment_repo::EmploymentRepository,
                models::{EmploymentData, EmploymentFilter, NewEmployment},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;

    use crate::test_constants;

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        let employment_data = NewEmployment {
            user_id: test_constants::USER2_ID,
            company_id: test_constants::COMPANY2_ID,
            manager_id: Some(test_constants::USER0_ID),
            hourly_wage: 100.0,
            start_date: NaiveDate::from_ymd_opt(2021, 9, 15).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 9, 16).unwrap(),
            description: Some("Test Description".to_string()),
            employment_type: EmploymentContract::Hpp,
            level: EmployeeLevel::CompanyAdministrator,
        };

        let new_employment = employment_repo
            .create(employment_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_employment.user_id, employment_data.user_id);
        assert_eq!(new_employment.company_id, employment_data.company_id);
        assert_eq!(new_employment.manager_id, employment_data.manager_id);
        assert_eq!(new_employment.hourly_wage, employment_data.hourly_wage);
        assert_eq!(new_employment.start_date, employment_data.start_date);
        assert_eq!(new_employment.end_date, employment_data.end_date);
        assert_eq!(new_employment.description, employment_data.description);
        assert_eq!(
            new_employment.employment_type,
            employment_data.employment_type
        );
        assert_eq!(new_employment.level, employment_data.level);

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
        let time_difference_created = time - new_employment.created_at;
        let time_difference_edited = time - new_employment.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);

        assert!(new_employment.deleted_at.is_none());

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        // Manager exists

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let employment = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect("Read should succeed");

            assert_eq!(employment.company.name, "AMD");
            assert_eq!(employment.manager.unwrap().name, "Dave Null");
            assert_eq!(employment.hourly_wage, 200.0);
        }

        // Manager doesn't exist

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER0_ID;

            let employment = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect("Read should succeed");

            assert_eq!(employment.company.name, "AMD");
            assert!(employment.manager.is_none());
            assert_eq!(employment.hourly_wage, 300.0);
        }

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    async fn read_all_per_user(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        let user_id = test_constants::USER2_ID;

        let filter = EmploymentFilter {
            limit: None,
            offset: None,
        };

        let employments = employment_repo
            .read_all_for_user(user_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(employments.len(), 2);

        // Had to switch the indices.
        let employment = &employments[1];

        assert_eq!(employment.company.name, "AMD");
        assert_eq!(employment.manager.clone().unwrap().name, "Dave Null");

        let employment = &employments[0];

        assert_eq!(employment.company.name, "ReportLab");
        assert!(employment.manager.is_none());

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    pub fn read_all_per_company(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        let company_id = test_constants::COMPANY0_ID;

        let filter = EmploymentFilter {
            limit: None,
            offset: None,
        };

        let employments = employment_repo
            ._read_all_for_company(company_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(employments.len(), 3);

        let employment = &employments[2];

        assert_eq!(employment.hourly_wage, 200.0);
        assert_eq!(employment.manager.clone().unwrap().name, "Dave Null");

        let employment = &employments[1];

        assert_eq!(employment.hourly_wage, 250.0);
        assert_eq!(employment.manager.clone().unwrap().name, "Dave Null");

        let employment = &employments[0];

        assert_eq!(employment.hourly_wage, 300.0);
        assert!(employment.manager.is_none());

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    pub fn read_all_subordinates(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        let user_id = test_constants::USER0_ID;
        let company_id = test_constants::COMPANY0_ID;

        let filter = EmploymentFilter {
            limit: None,
            offset: None,
        };

        let employments = employment_repo
            .read_subordinates(user_id, company_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(employments.len(), 2);

        let employment = &employments[0];

        assert_eq!(employment.company.name, "AMD");
        assert_eq!(employment.manager.clone().unwrap().name, "Dave Null");
        assert_eq!(employment.hourly_wage, 250.0);
        assert_eq!(employment.user_id, test_constants::USER1_ID);

        let employment = &employments[1];

        assert_eq!(employment.company.name, "AMD");
        assert_eq!(employment.manager.clone().unwrap().name, "Dave Null");
        assert_eq!(employment.hourly_wage, 200.0);
        assert_eq!(employment.user_id, test_constants::USER2_ID);

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    pub fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        // Valid update

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let new_employment_data = EmploymentData {
                manager_id: Some(test_constants::USER0_ID),
                hourly_wage: Some(10000.0),
                start_date: Some(NaiveDate::from_ymd_opt(2027, 9, 15).unwrap()),
                end_date: Some(NaiveDate::from_ymd_opt(2027, 9, 16).unwrap()),
                description: Some("Test Description".to_string()),
                employment_type: Some(EmploymentContract::Hpp),
                level: Some(EmployeeLevel::CompanyAdministrator),
            };

            let updated_employment = employment_repo
                .update(user_id, company_id, new_employment_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_employment.user_id, user_id);
            assert_eq!(updated_employment.company_id, company_id);

            assert_eq!(
                updated_employment.manager_id,
                new_employment_data.manager_id
            );
            assert_eq!(
                updated_employment.hourly_wage,
                new_employment_data.hourly_wage.unwrap()
            );
            assert_eq!(
                updated_employment.start_date,
                new_employment_data.start_date.unwrap()
            );
            assert_eq!(
                updated_employment.end_date,
                new_employment_data.end_date.unwrap()
            );
            assert_eq!(
                updated_employment.description,
                new_employment_data.description
            );
            assert_eq!(
                updated_employment.employment_type,
                new_employment_data.employment_type.unwrap()
            );
            assert_eq!(updated_employment.level, new_employment_data.level.unwrap());

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_edited = time - updated_employment.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_employment.deleted_at.is_none());
        }

        // All are none

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let new_employment_data = EmploymentData {
                manager_id: None,
                hourly_wage: None,
                start_date: None,
                end_date: None,
                description: None,
                employment_type: None,
                level: None,
            };

            let _updated_employment = employment_repo
                .update(user_id, company_id, new_employment_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let company_id = test_constants::COMPANY2_ID;
            let user_id = test_constants::USER0_ID;

            let new_employment_data = EmploymentData {
                manager_id: None,
                hourly_wage: Some(10000.0),
                start_date: None,
                end_date: None,
                description: None,
                employment_type: Some(EmploymentContract::Hpp),
                level: Some(EmployeeLevel::CompanyAdministrator),
            };

            let _updated_employment = employment_repo
                .update(user_id, company_id, new_employment_data)
                .await
                .expect_err("Update should fail - non existent employment");
        }

        // Already deleted

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let employment = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect("Read should succeed");

            assert!(employment.deleted_at.is_none());

            employment_repo
                .delete(user_id, company_id)
                .await
                .expect("Delete should succeed");

            let new_employment_data = EmploymentData {
                manager_id: None,
                hourly_wage: Some(10000.0),
                start_date: None,
                end_date: None,
                description: None,
                employment_type: Some(EmploymentContract::Hpp),
                level: Some(EmployeeLevel::CompanyAdministrator),
            };

            let _updated_employment = employment_repo
                .update(user_id, company_id, new_employment_data)
                .await
                .expect_err("Update should fail - already deleted employment");
        }

        employment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("employments"), migrations = "migrations/no_seed")]
    pub fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut employment_repo = EmploymentRepository::new(arc_pool);

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let employment = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect("Read should succeed");

            assert!(employment.deleted_at.is_none());

            employment_repo.delete(user_id, company_id).await.unwrap();

            let _new_employment = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect_err("Read should not succeed - we can't read a deleted entry.");
        }

        // delete on already deleted employment

        {
            let company_id = test_constants::COMPANY0_ID;
            let user_id = test_constants::USER2_ID;

            let _ = employment_repo
                .read_one(user_id, company_id)
                .await
                .expect_err("Read should not succeed");

            employment_repo
                .delete(user_id, company_id)
                .await
                .expect_err(
                    "Repository should return error on deleting an already deleted employment",
                );
        }

        // delete on non-existing employment

        {
            let company_id = test_constants::COMPANY2_ID;
            let user_id = test_constants::USER1_ID;

            employment_repo
                .delete(user_id, company_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing employment");
        }

        employment_repo.disconnect().await;

        Ok(())
    }
}

// needs user, company, event
#[cfg(test)]
pub mod event_staff_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDateTime, Utc};
    use organization::{
        common::DbResult,
        models::{AcceptanceStatus, EventRole},
        repositories::{
            event_staff::{
                event_staff_repo::StaffRepository,
                models::{NewStaff, StaffData, StaffFilter},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("event_staff"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_staff_repo = StaffRepository::new(arc_pool);

        let event_staff_data = NewStaff {
            user_id: test_constants::USER2_ID,
            company_id: test_constants::COMPANY2_ID,
            role: EventRole::Organizer,
        };

        let new_event_staff = event_staff_repo
            .create(test_constants::EVENT0_ID, event_staff_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_event_staff.user.id, event_staff_data.user_id);
        assert_eq!(new_event_staff.event_id, test_constants::EVENT0_ID);
        assert_eq!(new_event_staff.company.id, event_staff_data.company_id);
        assert_eq!(new_event_staff.role, event_staff_data.role);

        assert_eq!(new_event_staff.status, AcceptanceStatus::Pending);
        assert!(new_event_staff.decided_by.is_none());

        assert!(new_event_staff.deleted_at.is_none());

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

        let time_difference_created = time - new_event_staff.created_at;
        let time_difference_edited = time - new_event_staff.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);

        event_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("event_staff"), migrations = "migrations/no_seed")]
    async fn read(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_staff_repo = StaffRepository::new(arc_pool);

        let event_staff_id = test_constants::EVENT_STAFF0_ID;

        let event_staff = event_staff_repo
            .read_one(event_staff_id)
            .await
            .expect("Read should succeed");

        assert_eq!(event_staff.user.name, "Dave Null");
        assert_eq!(event_staff.company.name, "AMD");
        assert_eq!(event_staff.role, EventRole::Organizer);
        assert_eq!(event_staff.status, AcceptanceStatus::Accepted);
        assert!(event_staff.decided_by.is_some());

        event_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("event_staff"), migrations = "migrations/no_seed")]
    async fn read_all_per_event(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_staff_repo = StaffRepository::new(arc_pool);

        let event_id = test_constants::EVENT0_ID;

        let filter = StaffFilter {
            limit: None,
            offset: None,
        };

        let event_staffs = event_staff_repo
            .read_all_for_event(event_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(event_staffs.len(), 2);

        let event_staff = &event_staffs[0];

        assert_eq!(event_staff.user.name, "Dave Null");

        let event_staff = &event_staffs[1];

        assert_eq!(event_staff.user.name, "Tana Smith");

        event_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("event_staff"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_staff_repo = StaffRepository::new(arc_pool);

        // Valid update

        {
            let decider_staff_id = test_constants::EVENT_STAFF0_ID;

            let event_staff_id = test_constants::EVENT_STAFF1_ID;

            let event_staff = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect("Read should succeed");

            let new_event_staff_data = StaffData {
                role: Some(EventRole::Staff),
                status: Some(AcceptanceStatus::Rejected),
                decided_by: Some(decider_staff_id),
            };

            let updated_event_staff = event_staff_repo
                .update(event_staff_id, new_event_staff_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_event_staff.user.id, event_staff.user.id);
            assert_eq!(updated_event_staff.company.id, event_staff.company.id);
            assert_eq!(updated_event_staff.role, new_event_staff_data.role.unwrap());
            assert_eq!(
                updated_event_staff.status,
                new_event_staff_data.status.unwrap()
            );
            assert_eq!(
                updated_event_staff.decided_by.unwrap(),
                new_event_staff_data.decided_by.unwrap()
            );

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_edited = time - updated_event_staff.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_event_staff.deleted_at.is_none());
        }

        // All are none

        {
            let decider_staff_id = test_constants::EVENT_STAFF0_ID;

            let event_staff_id_wrong = test_constants::EVENT_STAFF1_ID;

            let new_event_staff_data = StaffData {
                role: None,
                status: None,
                decided_by: Some(decider_staff_id),
            };

            let _updated_event_staff = event_staff_repo
                .update(event_staff_id_wrong, new_event_staff_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let decider_staff_id = test_constants::EVENT_STAFF0_ID;

            let event_staff_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            let new_event_staff_data = StaffData {
                role: None,
                status: Some(AcceptanceStatus::Rejected),
                decided_by: Some(decider_staff_id),
            };

            let _updated_event_staff = event_staff_repo
                .update(event_staff_id, new_event_staff_data)
                .await
                .expect_err("Update should fail - non existent event staff");
        }

        // Already deleted

        {
            let decider_staff_id = test_constants::EVENT_STAFF0_ID;

            let event_staff_id = test_constants::EVENT_STAFF1_ID;

            let event_staff = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect("Read should succeed");

            assert!(event_staff.deleted_at.is_none());

            event_staff_repo
                .delete(event_staff_id)
                .await
                .expect("Delete should succeed");

            let _ = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect_err("Read should not succeed");

            let new_event_staff_data = StaffData {
                role: None,
                status: Some(AcceptanceStatus::Rejected),
                decided_by: Some(decider_staff_id),
            };

            let _updated_event_staff = event_staff_repo
                .update(event_staff_id, new_event_staff_data)
                .await
                .expect_err("Update should fail - already deleted event staff");
        }

        event_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("event_staff"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut event_staff_repo = StaffRepository::new(arc_pool);

        {
            let event_staff_id = test_constants::EVENT_STAFF1_ID;

            let event_staff = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect("Read should succeed");

            assert!(event_staff.deleted_at.is_none());

            event_staff_repo.delete(event_staff_id).await.unwrap();

            let _new_event_staff = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted event staff

        {
            let event_staff_id = test_constants::EVENT_STAFF1_ID;

            let _ = event_staff_repo
                .read_one(event_staff_id)
                .await
                .expect_err("Read should not succeed");

            event_staff_repo.delete(event_staff_id).await.expect_err(
                "Repository should return error on deleting an already deleted event staff",
            );
        }

        // delete on non-existing event staff

        {
            let event_staff_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            event_staff_repo.delete(event_staff_id).await.expect_err(
                "Repository should return error on deleting a non-existing event staff",
            );
        }

        event_staff_repo.disconnect().await;

        Ok(())
    }
}

// event, event_staff
#[cfg(test)]
pub mod task_repo_tests {
    use chrono::{NaiveDateTime, Utc};
    use organization::{
        common::DbResult,
        models::TaskPriority,
        repositories::{
            repository::DbRepository,
            task::{
                models::{NewTask, TaskData, TaskFilter},
                task_repo::TaskRepository,
            },
        },
    };
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("task"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut task_repo = TaskRepository::new(arc_pool);

        let new_task_data = NewTask {
            event_id: test_constants::EVENT0_ID,
            creator_id: test_constants::EVENT_STAFF1_ID,
            description: Some("Test Description".to_string()),
            title: "Test Title".to_string(),
            priority: TaskPriority::High,
        };

        let new_task = task_repo
            .create(new_task_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_task.event_id, new_task_data.event_id);
        assert_eq!(new_task.creator_id, new_task_data.creator_id);
        assert_eq!(new_task.description, new_task_data.description);
        assert_eq!(new_task.title, new_task_data.title);
        assert_eq!(new_task.priority, new_task_data.priority);

        assert!(new_task.deleted_at.is_none());

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

        let time_difference_created = time - new_task.created_at;
        let time_difference_edited = time - new_task.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);

        task_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("task"), migrations = "migrations/no_seed")]
    async fn read_one(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut task_repo = TaskRepository::new(arc_pool);

        let task_id = test_constants::TASK0_ID;

        let task = task_repo
            .read_one(task_id)
            .await
            .expect("Read should succeed");

        assert_eq!(task.title, "Prepare stage for Joe Cocker");
        assert!(task.description.is_none());
        assert_eq!(task.priority, TaskPriority::Medium);
        assert_eq!(task.creator.name, "Dave Null");

        task_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("task"), migrations = "migrations/no_seed")]
    async fn read_all_per_event(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut task_repo = TaskRepository::new(arc_pool);

        let filter = TaskFilter {
            limit: None,
            offset: None,
        };

        {
            let event_id = test_constants::EVENT0_ID;

            let tasks = task_repo
                .read_all_for_event(event_id, filter.clone())
                .await
                .expect("Read should succeed");

            assert_eq!(tasks.len(), 2);

            let task = &tasks[0];

            assert_eq!(task.title, "Prepare stage for Joe Cocker");

            let task = &tasks[1];

            assert_eq!(task.title, "Prepare stage for Santa");
        }

        {
            let tasks = task_repo
                ._read_all(filter)
                .await
                .expect("Read should succeed");

            // TODO - better test

            assert_eq!(tasks.len(), 2);

            let task = &tasks[0];

            assert_eq!(task.title, "Prepare stage for Santa");

            let task = &tasks[1];

            assert_eq!(task.title, "Prepare stage for Joe Cocker");
        }

        task_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("task"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut task_repo = TaskRepository::new(arc_pool);

        // Valid update

        {
            let task_id = test_constants::TASK0_ID;

            let _ = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            let new_task_data = TaskData {
                title: Some("New Title".to_string()),
                description: Some("New Description".to_string()),
                finished_at: Some(
                    NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap(),
                ),
                priority: Some(TaskPriority::Low),
                accepts_staff: Some(false),
            };

            let updated_task = task_repo
                .update(task_id, new_task_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_task.title, new_task_data.title.unwrap());
            assert_eq!(updated_task.description, new_task_data.description);
            assert_eq!(updated_task.priority, new_task_data.priority.unwrap());
            assert_eq!(
                updated_task.accepts_staff,
                new_task_data.accepts_staff.unwrap()
            );
            assert_eq!(updated_task.finished_at, new_task_data.finished_at);

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_edited = time - updated_task.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_task.deleted_at.is_none());
        }

        // All are none

        {
            let task_id = test_constants::TASK0_ID;

            let new_task_data = TaskData {
                title: None,
                description: None,
                finished_at: None,
                priority: None,
                accepts_staff: None,
            };

            let _updated_task = task_repo
                .update(task_id, new_task_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let task_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            let new_task_data = TaskData {
                title: Some("New Title".to_string()),
                description: Some("New Description".to_string()),
                finished_at: None,
                priority: Some(TaskPriority::Low),
                accepts_staff: Some(false),
            };

            let _updated_task = task_repo
                .update(task_id, new_task_data)
                .await
                .expect_err("Update should fail - non existent task");
        }

        // Already deleted

        {
            let task_id = test_constants::TASK0_ID;

            let task = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            assert!(task.deleted_at.is_none());

            task_repo
                .delete(task_id)
                .await
                .expect("Delete should succeed");

            let deleted_task = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            assert!(deleted_task.deleted_at.is_some());

            let new_task_data = TaskData {
                title: Some("New Title".to_string()),
                description: Some("New Description".to_string()),
                finished_at: None,
                priority: Some(TaskPriority::Low),
                accepts_staff: Some(false),
            };

            let _updated_task = task_repo
                .update(task_id, new_task_data)
                .await
                .expect_err("Update should fail - already deleted task");
        }

        task_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("task"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut task_repo = TaskRepository::new(arc_pool);

        {
            let task_id = test_constants::TASK0_ID;

            let task = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            assert!(task.deleted_at.is_none());

            task_repo.delete(task_id).await.unwrap();

            let new_task = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
            let time_difference_edited = time - new_task.edited_at;
            let time_difference_deleted = time - new_task.deleted_at.unwrap();

            assert!(time_difference_edited.num_seconds() < 2);
            assert!(time_difference_deleted.num_seconds() < 2);
        }

        // delete on already deleted task

        {
            let task_id = test_constants::TASK0_ID;

            let task = task_repo
                .read_one(task_id)
                .await
                .expect("Read should succeed");

            assert!(task.deleted_at.is_some());

            task_repo
                .delete(task_id)
                .await
                .expect_err("Repository should return error on deleting an already deleted task");
        }

        // delete on non-existing task

        {
            let task_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            task_repo
                .delete(task_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing task");
        }

        task_repo.disconnect().await;

        Ok(())
    }
}

// needs event_staff, task
#[cfg(test)]
pub mod assigned_staff_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDateTime, Utc};
    use organization::{
        common::DbResult,
        models::AcceptanceStatus,
        repositories::{
            assigned_staff::{
                assigned_staff_repo::AssignedStaffRepository,
                models::{AssignedStaffData, AssignedStaffFilter, NewAssignedStaff},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("assigned_staff"), migrations = "migrations/no_seed")]
    pub fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut assigned_staff_repo = AssignedStaffRepository::new(arc_pool);

        let assigned_staff_data = NewAssignedStaff {
            staff_id: test_constants::EVENT_STAFF1_ID,
            task_id: test_constants::TASK1_ID,
        };

        let new_assigned_staff = assigned_staff_repo
            .create(assigned_staff_data.clone())
            .await
            .expect("Create should succeed");

        assert_eq!(new_assigned_staff.staff.id, assigned_staff_data.staff_id);
        assert_eq!(new_assigned_staff.task_id, assigned_staff_data.task_id);
        assert_eq!(new_assigned_staff.status, AcceptanceStatus::Pending);

        assert!(new_assigned_staff.decided_by.is_none());
        assert!(new_assigned_staff.deleted_at.is_none());

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

        let time_difference_created = time - new_assigned_staff.created_at;
        let time_difference_edited = time - new_assigned_staff.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);

        assigned_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("assigned_staff"), migrations = "migrations/no_seed")]
    pub fn read_one(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut assigned_staff_repo = AssignedStaffRepository::new(arc_pool);

        let staff_id = test_constants::EVENT_STAFF0_ID;
        let task_id = test_constants::TASK0_ID;

        let assigned_staff = assigned_staff_repo
            .read_one(task_id, staff_id)
            .await
            .expect("Read should succeed");

        assert_eq!(assigned_staff.staff.user.name, "Dave Null");
        assert_eq!(assigned_staff.decided_by_user.unwrap().name, "Dave Null");

        assigned_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("assigned_staff"), migrations = "migrations/no_seed")]
    pub fn read_all_per_task(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut assigned_staff_repo = AssignedStaffRepository::new(arc_pool);

        let task_id = test_constants::TASK0_ID;

        let filter = AssignedStaffFilter {
            limit: None,
            offset: None,
        };

        let mut assigned_staffs = assigned_staff_repo
            .read_all_per_task(task_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(assigned_staffs.len(), 2);

        assigned_staffs.sort_by(|a, b| a.staff.user.name.cmp(&b.staff.user.name));

        let assigned_staff = &assigned_staffs[0];

        assert_eq!(assigned_staff.staff.user.name, "Dave Null");
        assert_eq!(
            assigned_staff.decided_by_user.clone().unwrap().name,
            "Dave Null"
        );

        let assigned_staff = &assigned_staffs[1];

        assert_eq!(assigned_staff.staff.user.name, "Tana Smith");
        assert!(assigned_staff.decided_by.clone().is_none());

        assigned_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("assigned_staff"), migrations = "migrations/no_seed")]
    pub fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut assigned_staff_repo = AssignedStaffRepository::new(arc_pool);

        // Valid update

        {
            let decider_staff_id = test_constants::EVENT_STAFF0_ID;

            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = test_constants::TASK0_ID;

            let assigned_staff_data = AssignedStaffData {
                status: AcceptanceStatus::Accepted,
                decided_by: decider_staff_id,
            };

            let updated_assigned_staff = assigned_staff_repo
                .update(task_id, staff_id, assigned_staff_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_assigned_staff.status, AcceptanceStatus::Accepted);
            assert_eq!(
                updated_assigned_staff.decided_by,
                Some(assigned_staff_data.decided_by)
            );

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_edited = time - updated_assigned_staff.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_assigned_staff.deleted_at.is_none());
        }

        // Non existent

        {
            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            let assigned_staff_data = AssignedStaffData {
                status: AcceptanceStatus::Accepted,
                decided_by: test_constants::EVENT_STAFF0_ID,
            };

            let _updated_assigned_staff = assigned_staff_repo
                .update(task_id, staff_id, assigned_staff_data)
                .await
                .expect_err("Update should fail - non existent assigned staff");
        }

        // Already deleted

        {
            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = test_constants::TASK0_ID;

            let assigned_staff = assigned_staff_repo
                .read_one(task_id, staff_id)
                .await
                .expect("Read should succeed");

            assert!(assigned_staff.deleted_at.is_none());

            assigned_staff_repo
                .delete(task_id, staff_id)
                .await
                .expect("Delete should succeed");

            let _ = assigned_staff_repo
                .read_one(task_id, staff_id)
                .await
                .expect_err("Read should not succeed");

            let assigned_staff_data = AssignedStaffData {
                status: AcceptanceStatus::Accepted,
                decided_by: test_constants::EVENT_STAFF0_ID,
            };

            let _updated_assigned_staff = assigned_staff_repo
                .update(task_id, staff_id, assigned_staff_data)
                .await
                .expect_err("Update should fail - already deleted assigned staff");
        }

        assigned_staff_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("assigned_staff"), migrations = "migrations/no_seed")]
    pub fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut assigned_staff_repo = AssignedStaffRepository::new(arc_pool);

        {
            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = test_constants::TASK0_ID;

            let assigned_staff = assigned_staff_repo
                .read_one(task_id, staff_id)
                .await
                .expect("Read should succeed");

            assert!(assigned_staff.deleted_at.is_none());

            assigned_staff_repo.delete(task_id, staff_id).await.unwrap();

            let _ = assigned_staff_repo
                .read_one(task_id, staff_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted assigned staff

        {
            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = test_constants::TASK0_ID;

            let _ = assigned_staff_repo
                .read_one(task_id, staff_id)
                .await
                .expect_err("Read should not succeed");

            assigned_staff_repo
                .delete(task_id, staff_id)
                .await
                .expect_err(
                    "Repository should return error on deleting an already deleted assigned staff",
                );
        }

        // delete on non-existing assigned staff

        {
            let staff_id = test_constants::EVENT_STAFF1_ID;
            let task_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            assigned_staff_repo
                .delete(task_id, staff_id)
                .await
                .expect_err(
                    "Repository should return error on deleting a non-existing assigned staff",
                );
        }

        assigned_staff_repo.disconnect().await;

        Ok(())
    }
}

// needs event, task, user
#[cfg(test)]
pub mod comment_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDateTime, Utc};
    use organization::{
        common::DbResult,
        repositories::{
            comment::{
                comment_repo::CommentRepository,
                models::{CommentData, CommentFilter, NewComment},
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;
    use uuid::uuid;

    use crate::test_constants;

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        // Valid create

        {
            let new_comment_data = NewComment {
                author_id: test_constants::USER0_ID,
                event_id: Some(test_constants::EVENT0_ID),
                task_id: None,
                content: "Test Content".to_string(),
            };

            let new_comment = comment_repo
                .create(new_comment_data.clone())
                .await
                .expect("Create should succeed");

            assert_eq!(new_comment.author.id, new_comment_data.author_id);
            assert_eq!(new_comment.event_id, new_comment_data.event_id);
            assert_eq!(new_comment.task_id, new_comment_data.task_id);
            assert_eq!(new_comment.content, new_comment_data.content);
            assert_eq!(new_comment.deleted_at, None);

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_created = time - new_comment.created_at;
            let time_difference_edited = time - new_comment.edited_at;

            assert!(time_difference_created.num_seconds() < 2);
            assert!(time_difference_edited.num_seconds() < 2);
        }

        // All are none

        {
            let new_comment_data = NewComment {
                author_id: test_constants::USER0_ID,
                event_id: None,
                task_id: None,
                content: "Test Content".to_string(),
            };

            let _new_comment = comment_repo
                .create(new_comment_data)
                .await
                .expect_err("Create should fail - all fields are none");
        }

        // Both are some

        {
            let new_comment_data = NewComment {
                author_id: test_constants::USER0_ID,
                event_id: Some(test_constants::EVENT0_ID),
                task_id: Some(test_constants::TASK0_ID),
                content: "Test Content".to_string(),
            };

            let _new_comment = comment_repo
                .create(new_comment_data)
                .await
                .expect_err("Create should fail - both event and task are some");
        }

        comment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn read_one(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        let comment_id = test_constants::COMMENT0_ID;

        let comment = comment_repo
            ._read_one(comment_id)
            .await
            .expect("Read should succeed");

        assert_eq!(comment.content, "Joe will need 3 guitars on stage.");
        assert_eq!(comment.author.name, "Dave Null");

        comment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn read_all_per_event(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        let event_id = test_constants::EVENT0_ID;

        let filter = CommentFilter {
            limit: None,
            offset: None,
        };

        let mut comments = comment_repo
            .read_all_per_event(event_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(comments.len(), 2);

        comments.sort_by(|a, b| a.author.name.cmp(&b.author.name));

        let comment = &comments[0];

        assert_eq!(comment.content, "Oh nooo!");
        assert_eq!(comment.author.name, "Dave Null");

        let comment = &comments[1];

        assert_eq!(comment.content, "I have a problem.");
        assert_eq!(comment.author.name, "Tana Smith");

        comment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn read_all_per_task(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        let task_id = test_constants::TASK0_ID;

        let filter = CommentFilter {
            limit: None,
            offset: None,
        };

        let mut comments = comment_repo
            .read_all_per_task(task_id, filter)
            .await
            .expect("Read should succeed");

        assert_eq!(comments.len(), 2);

        comments.sort_by(|a, b| a.author.name.cmp(&b.author.name));

        let comment = &comments[0];

        assert_eq!(comment.content, "Joe will need 3 guitars on stage.");
        assert_eq!(comment.author.name, "Dave Null");

        let comment = &comments[1];

        assert_eq!(comment.content, "I will take care of it.");
        assert_eq!(comment.author.name, "Tana Smith");

        comment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        // Valid update

        {
            let comment_id = test_constants::COMMENT0_ID;

            let _comment = comment_repo
                ._read_one(comment_id)
                .await
                .expect("Read should succeed");

            let new_comment_data = CommentData {
                content: "New Content".to_string(),
            };

            let updated_comment = comment_repo
                .update(comment_id, new_comment_data.clone())
                .await
                .expect("Update should succeed");

            assert_eq!(updated_comment.content, new_comment_data.content);

            let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();

            let time_difference_edited = time - updated_comment.edited_at;
            assert!(time_difference_edited.num_seconds() < 2);

            assert!(updated_comment.deleted_at.is_none());
        }

        // All are none

        {
            let comment_id = test_constants::COMMENT0_ID;

            let new_comment_data = CommentData {
                content: "".to_string(),
            };

            let _updated_comment = comment_repo
                .update(comment_id, new_comment_data)
                .await
                .expect_err("Update should fail - all fields are none");
        }

        // Non existent

        {
            let comment_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            let new_comment_data = CommentData {
                content: "New Content".to_string(),
            };

            let _updated_comment = comment_repo
                .update(comment_id, new_comment_data)
                .await
                .expect_err("Update should fail - non existent comment");
        }

        // Already deleted

        {
            let comment_id = test_constants::COMMENT0_ID;

            let comment = comment_repo
                ._read_one(comment_id)
                .await
                .expect("Read should succeed");

            assert!(comment.deleted_at.is_none());

            comment_repo
                .delete(comment_id)
                .await
                .expect("Delete should succeed");

            let _ = comment_repo
                ._read_one(comment_id)
                .await
                .expect_err("Read should not succeed");

            let new_comment_data = CommentData {
                content: "New Content".to_string(),
            };

            let _updated_comment = comment_repo
                .update(comment_id, new_comment_data)
                .await
                .expect_err("Update should fail - already deleted comment");
        }

        comment_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("comments"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut comment_repo = CommentRepository::new(arc_pool);

        // Valid delete

        {
            let comment_id = test_constants::COMMENT0_ID;

            let comment = comment_repo
                ._read_one(comment_id)
                .await
                .expect("Read should succeed");

            assert!(comment.deleted_at.is_none());

            comment_repo.delete(comment_id).await.unwrap();

            let _ = comment_repo
                ._read_one(comment_id)
                .await
                .expect_err("Read should not succeed");
        }

        // delete on already deleted comment

        {
            let comment_id = test_constants::COMMENT0_ID;

            let _ = comment_repo
                ._read_one(comment_id)
                .await
                .expect_err("Read should not succeed");

            comment_repo.delete(comment_id).await.expect_err(
                "Repository should return error on deleting an already deleted comment",
            );
        }

        // delete on non-existing comment

        {
            let comment_id = uuid!("a96d1d99-93b5-469b-ac62-654b0cf7ebd9");

            comment_repo
                .delete(comment_id)
                .await
                .expect_err("Repository should return error on deleting a non-existing comment");
        }

        comment_repo.disconnect().await;

        Ok(())
    }
}

#[cfg(test)]
mod timesheet_repo_tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use organization::{
        models::ApprovalStatus,
        repositories::timesheet::{
            models::{TimesheetCreateData, TimesheetReadAllData, TimesheetUpdateData},
            timesheet_repo::TimesheetRepository,
        },
    };
    use sqlx::PgPool;

    use crate::test_constants::{
        COMPANY1_ID, COMPANY2_ID, EVENT0_ID, TIMESHEET0_ID, TIMESHEET1_ID,
        TIMESHEET4_ID, USER1_ID, USER2_ID,
    };
    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn create(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let user_id = USER1_ID;
            let company_id = COMPANY1_ID;
            let event_id = EVENT0_ID;
            let start_date = NaiveDate::from_ymd_opt(1969, 8, 15).unwrap();
            let end_date = NaiveDate::from_ymd_opt(1969, 08, 18).unwrap();
            let data = TimesheetCreateData {
                start_date,
                end_date,
                user_id,
                company_id,
                event_id,
            };

            let result = timesheet_repo.create(data).await.expect("Should succed");
            assert_eq!(result.workdays.len(), 4);
            for day in result.workdays.into_iter() {
                assert!(day.date >= start_date);
                assert!(day.date <= end_date);
            }
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
        }
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_one(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let sheet_id = TIMESHEET0_ID;
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;
            let event_id = EVENT0_ID;

            let result = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succed");
            assert_eq!(result.workdays.len(), 2);
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
            assert_eq!(result.timesheet.company_id, company_id);
            assert_eq!(result.timesheet.event_id, event_id);
            assert_eq!(result.timesheet.user_id, user_id);
        }

        {
            let sheet_id = TIMESHEET1_ID;

            let result = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succed");
            assert_eq!(result.workdays.len(), 3);
            assert_eq!(result.timesheet.approval_status, ApprovalStatus::Accepted);
        }

        // Non-existent sheet
        {
            let sheet_id = TIMESHEET4_ID;

            let _ = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect_err("Should not succed");
        }
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_all_per_employment(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;

            let data = TimesheetReadAllData {
                limit: None,
                offset: None,
            };
            let result = timesheet_repo
                .read_all_per_employment(user_id, company_id, data)
                .await
                .expect("Should succed");
            assert_eq!(result.len(), 1);
        }

        // Non-existent employment
        {
            let user_id = USER2_ID;
            let company_id = COMPANY2_ID;

            let data = TimesheetReadAllData {
                limit: None,
                offset: None,
            };
            let result = timesheet_repo
                .read_all_per_employment(user_id, company_id, data)
                .await
                .expect("Should succed");
            assert_eq!(result.len(), 0);
        }
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn update(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let sheet_id = TIMESHEET0_ID;
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;
            let event_id = EVENT0_ID;

            let result = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succeed.");
            assert_eq!(result.workdays.len(), 2);
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
            assert_eq!(result.timesheet.company_id, company_id);
            assert_eq!(result.timesheet.event_id, event_id);
            assert_eq!(result.timesheet.user_id, user_id);

            let data = TimesheetUpdateData {
                start_date: None,
                end_date: None,
                total_hours: None,
                is_editable: None,
                status: None,
                manager_note: Some("Change X and Y.".to_string()),
                workdays: None,
            };

            let result = timesheet_repo
                .update(sheet_id, data)
                .await
                .expect("Should succeed.");
            assert_eq!(result.workdays.len(), 2);
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
            assert_eq!(result.timesheet.company_id, company_id);
            assert_eq!(result.timesheet.event_id, event_id);
            assert_eq!(result.timesheet.user_id, user_id);
            assert!(result.timesheet.manager_note.is_some());
            assert_eq!(
                result.timesheet.manager_note.expect("Should be some"),
                "Change X and Y.".to_string()
            );
        }

        // Empty data
        {
            let sheet_id = TIMESHEET0_ID;
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;
            let event_id = EVENT0_ID;

            let result = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succeed.");
            assert_eq!(result.workdays.len(), 2);
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
            assert_eq!(result.timesheet.company_id, company_id);
            assert_eq!(result.timesheet.event_id, event_id);
            assert_eq!(result.timesheet.user_id, user_id);

            let data = TimesheetUpdateData {
                start_date: None,
                end_date: None,
                total_hours: None,
                is_editable: None,
                status: None,
                manager_note: None,
                workdays: None,
            };

            let _ = timesheet_repo
                .update(sheet_id, data)
                .await
                .expect_err("Should not succeed.");
        }

        // Non-existent sheet
        {
            let sheet_id = TIMESHEET4_ID;

            let _ = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect_err("Should not succeed.");

            let data = TimesheetUpdateData {
                start_date: None,
                end_date: None,
                total_hours: None,
                is_editable: None,
                status: None,
                manager_note: Some("Change X and Y.".to_string()),
                workdays: None,
            };

            let _ = timesheet_repo
                .update(sheet_id, data)
                .await
                .expect_err("Should succeed.");
        }

        // Deleted sheet
        {
            let sheet_id = TIMESHEET0_ID;
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;
            let event_id = EVENT0_ID;

            let result = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succeed.");
            assert_eq!(result.workdays.len(), 2);
            assert_eq!(
                result.timesheet.approval_status,
                ApprovalStatus::NotRequested
            );
            assert!(result.timesheet.is_editable);
            assert_eq!(result.timesheet.company_id, company_id);
            assert_eq!(result.timesheet.event_id, event_id);
            assert_eq!(result.timesheet.user_id, user_id);

            let _ = timesheet_repo
                ._delete(sheet_id)
                .await
                .expect("Should succeed");

            let data = TimesheetUpdateData {
                start_date: None,
                end_date: None,
                total_hours: None,
                is_editable: None,
                status: None,
                manager_note: Some("Change X and Y.".to_string()),
                workdays: None,
            };

            let _ = timesheet_repo
                .update(sheet_id, data)
                .await
                .expect_err("Should fail because we can't edit a deleted timesheet.");
        }
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn delete(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let sheet_id = TIMESHEET0_ID;

            let _ = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect("Should succeed.");

            let _ = timesheet_repo
                ._delete(sheet_id)
                .await
                .expect("Should succeed.");

            let _ = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect_err("Should fail.");

            // Deleting an already deleted sheet.
            let _ = timesheet_repo
                ._delete(sheet_id)
                .await
                .expect_err("Should fail.");
        }

        // Non-existent sheet
        {
            let sheet_id = TIMESHEET4_ID;

            let _ = timesheet_repo
                ._read_one(sheet_id)
                .await
                .expect_err("Should not succeed.");

            let _ = timesheet_repo
                ._delete(sheet_id)
                .await
                .expect_err("Should not succeed.");
        }
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_all_with_date_from_to_per_employment(pool: PgPool) {
        let arc_pool = Arc::new(pool);

        let timesheet_repo = TimesheetRepository::new(arc_pool);

        {
            let user_id = USER1_ID;
            let company_id = COMPANY1_ID;

            //
            // Check 1 timesheet is returned
            // and it has only **2 workays** (as we request
            // to omit workdays that are outside of `date range`).
            //
            {
                let result = timesheet_repo
                    .read_all_with_date_from_to_per_employment(
                        user_id, company_id,
                        NaiveDate::from_ymd_opt(1969, 7, 28).unwrap(),
                        NaiveDate::from_ymd_opt(1969, 7, 31).unwrap(),
                        true)
                    .await
                    .expect("Should succeed");
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].workdays.len(), 2);
            }

            //
            // Check 1 timesheet is returned
            // and it has all its workdays (as we request
            // to include workdays that are outside of `date range`).
            //
            {
                let result = timesheet_repo
                    .read_all_with_date_from_to_per_employment(
                        user_id, company_id,
                        NaiveDate::from_ymd_opt(1969, 7, 28).unwrap(),
                        NaiveDate::from_ymd_opt(1969, 7, 31).unwrap(),
                        false)
                    .await
                    .expect("Should succeed");
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].workdays.len(), 6);
            }

            // check two timesheets are returned
            {
                let result = timesheet_repo
                    .read_all_with_date_from_to_per_employment(
                        user_id, company_id,
                        NaiveDate::from_ymd_opt(1969, 7, 28).unwrap(),
                        NaiveDate::from_ymd_opt(1969, 8, 30).unwrap(),
                        true)
                    .await
                    .expect("Should succeed");
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].workdays.len(), 3);
                assert_eq!(result[1].workdays.len(), 6);
            }

            // check date range yielding nothing
            {
                let result = timesheet_repo
                    .read_all_with_date_from_to_per_employment(
                        user_id, company_id,
                        NaiveDate::from_ymd_opt(1969, 8, 26).unwrap(),
                        NaiveDate::from_ymd_opt(1969, 9, 10).unwrap(),
                        false)
                    .await
                    .expect("Should succeed");
                assert_eq!(result.len(), 0);
            }
        }

        //
        // Check no timesheet is returned when we request a date range
        // that's outside of employee's work window.
        //
        {
            let user_id = USER2_ID;
            let company_id = COMPANY1_ID;

            let result = timesheet_repo
                .read_all_with_date_from_to_per_employment(
                    user_id, company_id,
                    NaiveDate::from_ymd_opt(1969, 9, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
                    true)
                .await
                .expect("Should succeed");
            assert_eq!(result.len(), 0);
        }
    }
}

#[cfg(test)]
mod wage_preset_repo_tests {
    use std::sync::Arc;

    use crate::test_constants::DELTA;

    use chrono::NaiveDate;
    use organization::{
        common::DbResult,
        repositories::{
            wage_preset::{
                wage_preset_repo::WagePresetRepository,
                models::WagePreset,
            },
            repository::DbRepository,
        },
    };
    use sqlx::PgPool;

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_one(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut wage_preset_repo = WagePresetRepository::new(arc_pool);

        {
            let name = "cz_2024-01-01".to_string();

            let preset = wage_preset_repo
                .read_one(&name)
                .await
                .expect("Should succeed");
            assert_eq!(preset.currency, "CZK");
        }

        wage_preset_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_all(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut wage_preset_repo = WagePresetRepository::new(arc_pool);

        let presets = wage_preset_repo
            .read_all()
            .await
            .expect("Should succeed");
        assert_eq!(presets.len(), 3);

        wage_preset_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("all_inclusive"), migrations = "migrations/no_seed")]
    async fn read_optional_matching_date(pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(pool);

        let mut wage_preset_repo = WagePresetRepository::new(arc_pool);

        // non-existent
        {
            let preset_optional = wage_preset_repo
                .read_optional_matching_date(
                    &NaiveDate::from_ymd_opt(1965, 12, 31).unwrap())
                .await
                .expect("Should succeed");
            assert!(preset_optional.is_none());
        }

        {
            let preset_optional = wage_preset_repo
                .read_optional_matching_date(
                    &NaiveDate::from_ymd_opt(2023, 06, 01).unwrap())
                .await
                .expect("Should succeed");
            assert!(preset_optional.is_some());
            assert!(preset_optional.unwrap().min_hourly_wage - 100.0 < DELTA);
        }

        {
            let preset_optional = wage_preset_repo
                .read_optional_matching_date(
                    &NaiveDate::from_ymd_opt(2024, 01, 01).unwrap())
                .await
                .expect("Should succeed");
            assert!(preset_optional.is_some());
            assert!(preset_optional.unwrap().min_hourly_wage - 118.3 < DELTA);
        }

        {
            let preset_optional = wage_preset_repo
                .read_optional_matching_date(
                    &NaiveDate::from_ymd_opt(2030, 06, 10).unwrap())
                .await
                .expect("Should succeed");
            assert!(preset_optional.is_some());
            assert!(preset_optional.unwrap().min_hourly_wage - 118.3 < DELTA);
        }

        wage_preset_repo.disconnect().await;

        Ok(())
    }
}
