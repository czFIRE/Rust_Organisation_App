use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{Gender, UserRole, UserStatus};

use super::models::{NewUser, User, UserData, UsersQuery};

use async_trait::async_trait;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for UserRepository {
    /// Database repository constructor
    #[must_use]
    fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Method allowing the database repository to disconnect from the database pool gracefully
    async fn disconnect(&mut self) -> () {
        self.pool.close().await;
    }
}

impl UserRepository {
    // Creates a new user entry in the database.
    pub async fn create(&self, data: NewUser) -> DbResult<User> {
        let executor = self.pool.as_ref();

        let new_user: User = sqlx::query_as!(
            User,
            r#"INSERT INTO user_record (name, email, birth, gender, role) 
            VALUES 
                ($1, $2, $3, $4, $5) 
            RETURNING id, 
                name, 
                email, 
                birth, 
                avatar_url, 
                gender AS "gender!: Gender", 
                role AS "role!: UserRole", 
                status AS "status!: UserStatus", 
                created_at, 
                edited_at, 
                deleted_at
            "#,
            data.name,
            data.email,
            data.birth,
            data.gender as Gender,
            data.role as UserRole,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_user)
    }

    // Reads a single user from the database.
    pub async fn read_one(&self, user_id: Uuid) -> DbResult<User> {
        // TODO: Redis here

        self.read_one_db(user_id).await
    }

    /*
     * This is the actual function that accesses the DB. _read_one is used for
     * handling the request through redis - or trying to, at least. If the entry
     * is not cached, we read from the DB.
     */
    async fn read_one_db(&self, user_id: Uuid) -> DbResult<User> {
        let executor = self.pool.as_ref();

        let user: User = sqlx::query_as!(
            User,
            r#"SELECT 
                id, 
                name, 
                email, 
                birth, 
                avatar_url, 
                gender AS "gender!: Gender", 
                role AS "role!: UserRole", 
                status AS "status!: UserStatus", 
                created_at, 
                edited_at, 
                deleted_at 
            FROM 
                user_record 
            WHERE id = $1
              AND deleted_at IS NULL
            "#,
            user_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn read_one_with_email(&self, email: String) -> DbResult<User> {
        // TODO: Redis here

        self.read_one_with_email_db(email).await
    }

    async fn read_one_with_email_db(&self, email: String) -> DbResult<User> {
        let executor = self.pool.as_ref();

        let user = sqlx::query_as!(
            User,
            r#"SELECT 
                id, 
                name, 
                email, 
                birth, 
                avatar_url, 
                gender AS "gender!: Gender", 
                role AS "role!: UserRole", 
                status AS "status!: UserStatus", 
                created_at, 
                edited_at, 
                deleted_at 
            FROM 
                user_record 
            WHERE email = $1
              AND deleted_at IS NULL
            "#,
            email
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn _read_all(&self, filter: UsersQuery) -> DbResult<Vec<User>> {
        // TODO: Redis here

        self._read_all_db(filter).await
    }

    //ToDo: Use wildcard???
    async fn _read_all_db(&self, filter: UsersQuery) -> DbResult<Vec<User>> {
        let executor = self.pool.as_ref();

        let mut name_filter = if filter.name.is_some() {
            filter.name.expect("Should be some").clone()
        } else {
            "".to_string()
        };
        name_filter.push('%');

        let mut email_filter = if filter.email.is_some() {
            filter.email.expect("Should be some").clone()
        } else {
            "".to_string()
        };
        email_filter.push('%');

        let users: Vec<User> = sqlx::query_as!(
            User,
            r#"SELECT 
                id, 
                name, 
                email, 
                birth, 
                avatar_url, 
                gender AS "gender!: Gender", 
                role AS "role!: UserRole", 
                status AS "status!: UserStatus", 
                created_at, 
                edited_at, 
                deleted_at 
            FROM 
                user_record
            WHERE deleted_at IS NULL
              AND name LIKE $1
              AND email LIKE $2
            ORDER BY name
            "#,
            name_filter,
            email_filter,
        )
        .fetch_all(executor)
        .await?;

        Ok(users)
    }

    // Update a user in the DB.
    pub async fn update_user(&self, user_id: Uuid, data: UserData) -> DbResult<User> {
        let executor = self.pool.as_ref();

        if data.avatar_url.is_none()
            && data.birth.is_none()
            && data.email.is_none()
            && data.gender.is_none()
            && data.name.is_none()
            && data.role.is_none()
        {
            // TODO - add better error
            return Err(sqlx::Error::TypeNotFound {
                type_name: "User Error".to_string(),
            });
        }

        let user_res: User = sqlx::query_as!(
            User,
            r#"UPDATE 
                user_record 
            SET 
                name = COALESCE($1, name), 
                email = COALESCE($2, email), 
                birth = COALESCE($3, birth), 
                gender = COALESCE($4, gender), 
                role = COALESCE($5, role), 
                avatar_url = COALESCE($6, avatar_url),
                edited_at = NOW() 
            WHERE 
                id = $7 
                AND deleted_at IS NULL 
            RETURNING id, 
                name, 
                email, 
                birth, 
                avatar_url, 
                gender AS "gender!: Gender", 
                role AS "role!: UserRole", 
                status AS "status!: UserStatus", 
                created_at, 
                edited_at, 
                deleted_at
            "#,
            data.name,
            data.email,
            data.birth,
            data.gender as Option<Gender>,
            data.role as Option<UserRole>,
            data.avatar_url,
            user_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(user_res)
    }

    // Remove a user in the DB if they exist.
    pub async fn delete_user(&self, user_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let user = self.read_one(user_id).await?;

        if user.deleted_at.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            r#"UPDATE user_record
            SET deleted_at = NOW(), edited_at = NOW()
            WHERE id = $1
            AND deleted_at IS NULL
            "#,
            user_id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
