#[cfg(test)]
pub mod user_repo_tests {
    use std::sync::Arc;

    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use sqlx::PgPool;

    use organization_app::{
        common::DbResult,
        models::{Gender, UserRole},
        repositories::user::{models::NewUser, user_repo::UserRepository},
    };

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

        let new_user = user_repo.create(new_user_data.clone()).await?;

        assert_eq!(new_user.name, "Test User");
        assert_eq!(new_user.email, new_user_data.email);
        assert_eq!(new_user.birth, new_user_data.birth);
        assert_eq!(new_user.gender, new_user_data.gender);
        assert_eq!(new_user.role, new_user_data.role);

        let time = NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap();
        let time_difference_created = time - new_user.created_at;
        let time_difference_edited = time - new_user.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert!(new_user.deleted_at.is_none());

        // TODO - user picture should be what?
        // assert_eq!(new_user.avatar_url, None);

        user_repo.disconnect().await;

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn read(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("users"))]
    async fn update(_pool: PgPool) {
        todo!()
    }

    #[sqlx::test(fixtures("users"))]
    async fn delete(_pool: PgPool) {
        todo!()
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
