use crate::common::DbResult;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use sqlx::prelude::FromRow;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
    // timestamps
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct CompanyData {
    pub name: String,
    pub description: Option<String>,
    pub phone: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub crn: String,
    pub vatin: String,
}

#[derive(Clone)]
pub struct CompanyRepository {
    pub pool: Arc<PgPool>,
}

impl CompanyRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // CRUD

    pub async fn _create(&self, data: CompanyData) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(
            Company,
            "INSERT INTO company (name, description, phone, email, avatar_url, website, crn, vatin) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
            data.name,
            data.description,
            data.phone,
            data.email,
            data.avatar_url,
            data.website,
            data.crn,
            data.vatin
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn _read_one(&self, uuid: Uuid) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(Company, "SELECT * FROM company WHERE id = $1;", uuid)
            .fetch_one(executor)
            .await?;

        Ok(company)
    }

    pub async fn _read_all(&self) -> DbResult<Vec<Company>> {
        let executor = self.pool.as_ref();

        let companies = sqlx::query_as!(Company, "SELECT * FROM company;")
            .fetch_all(executor)
            .await?;

        Ok(companies)
    }

    pub async fn _update(&self, uuid: Uuid, data: CompanyData) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(
            Company,
            "UPDATE company SET name = $1, description = $2, phone = $3, email = $4, avatar_url = $5, website = $6, crn = $7, vatin = $8, edited_at = NOW() WHERE id = $9 RETURNING *;",
            data.name,
            data.description,
            data.phone,
            data.email,
            data.avatar_url,
            data.website,
            data.crn,
            data.vatin,
            uuid
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn _delete(&self, uuid: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!("UPDATE company SET deleted_at = NOW() WHERE id = $1;", uuid)
            .execute(executor)
            .await?;

        Ok(())
    }
}
