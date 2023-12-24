#[cfg(test)]
pub mod user_repo_tests {
    use sqlx::PgPool;

    #[sqlx::test(fixtures("users"))]
    async fn create(_pool: PgPool) {
        todo!()
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
