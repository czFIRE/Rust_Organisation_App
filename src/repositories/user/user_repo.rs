use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{Gender, UserRole, UserStatus};

use super::models::{NewUser, User, UserData};

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

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
    pub async fn _read_one(&self, user_id: Uuid) -> DbResult<User> {
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
            WHERE 
                id = $1
            "#,
            user_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    // Update a user in the DB.
    pub async fn _update_user(&self, user_id: Uuid, data: UserData) -> DbResult<User> {
        // TODO - this should support transactions
        let executor = self.pool.as_ref();

        if data.avatar_url.is_none()
            && data.birth.is_none()
            && data.email.is_none()
            && data.gender.is_none()
            && data.name.is_none()
            && data.role.is_none()
        {
            // TODO - add better error
            return Err(sqlx::Error::RowNotFound);
        }

        // Should return error if we can't find the user
        let _user_check = self.read_one_db(user_id).await?;

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
                AND deleted_at IS NULL RETURNING id, 
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
    pub async fn _delete_user(&self, user_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _user_res = sqlx::query!(
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

    #[inline]
    pub async fn disconnect(&mut self) {
        self.pool.close().await;
    }
}
