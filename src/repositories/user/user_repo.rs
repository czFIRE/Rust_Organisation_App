use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

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
    pub async fn _create(&self, _data: NewUser) -> DbResult<User> {
        todo!()
    }

    // Reads a single user from the database.
    pub async fn _read_one(&self, _uuid: Uuid) -> DbResult<User> {
        todo!()
    }

    /*
     * This is the actual function that accesses the DB. _read_one is used for
     * handling the request through redis - or trying to, at least. If the entry
     * is not cached, we read from the DB.
     */
    async fn _read_one_db(&self, _uuid: Uuid) -> DbResult<User> {
        todo!()
    }

    // Update a user in the DB.
    pub async fn _update_user(&self, _uuid: Uuid, _data: UserData) -> DbResult<User> {
        todo!()
    }

    // Remove a user in the DB if they exist.
    pub async fn _delete_user(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
