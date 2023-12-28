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
    use uuid::{uuid, Uuid};

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
    async fn read(_pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(_pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

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
    async fn update(_pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(_pool);

        let mut user_repo = UserRepository::new(arc_pool);

        let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

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
            let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d9");

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
    async fn delete(_pool: PgPool) -> DbResult<()> {
        let arc_pool = Arc::new(_pool);

        let mut user_repo = UserRepository::new(arc_pool);

        {
            let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

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
            let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d4");

            let user = user_repo.read_one(user_id).await.unwrap();

            assert!(user.deleted_at.is_some());

            user_repo
                .delete_user(user_id)
                .await
                .expect_err("Repository should return error on deleting an already deleted user");
        }

        // delete on non-existing user

        {
            let user_id: Uuid = uuid!("35341253-da20-40b6-96d8-ce069b1ba5d9");

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
    use sqlx::PgPool;

    #[sqlx::test(fixtures("companies"))]
    async fn create_company_test(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("companies"))]
    async fn read_company_test(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("companies"))]
    async fn read_all_companies_test(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("companies"))]
    async fn update_company_test(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("companies"))]
    async fn delete_company_test(_pool: PgPool) {
        todo!()
    }
}

#[cfg(test)]
pub mod event_repo_tests {
    use sqlx::PgPool;

    #[sqlx::test(fixtures("events"))]
    async fn create(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("events"))]
    async fn read(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("events"))]
    async fn read_all(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("events"))]
    async fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("events"))]
    async fn delete(_pool: PgPool) {
        todo!()
    }
}

#[cfg(test)]
pub mod employment_repo_tests {
    use sqlx::PgPool;

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

#[cfg(test)]
pub mod event_staff_repo_tests {
    use sqlx::PgPool;

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

#[cfg(test)]
pub mod task_repo_tests {
    use sqlx::PgPool;

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

#[cfg(test)]
pub mod assigned_staff_repo_tests {
    use sqlx::PgPool;

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

#[cfg(test)]
pub mod comment_repo_tests {
    use sqlx::PgPool;

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
