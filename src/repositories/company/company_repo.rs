use crate::common::DbResult;
use crate::repositories::company::models::Company;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{CompanyData, CompanyFilters};

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
            "INSERT INTO company (name, description, phone, email, avatar_url, website, crn, vatin) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
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

    pub async fn _read_one(&self, company_id: Uuid) -> DbResult<Company> {
        // TODO - Redis here
        self.read_one_db(company_id).await
    }

    pub async fn read_one_db(&self, company_id: Uuid) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(Company, "SELECT * FROM company WHERE id = $1;", company_id)
            .fetch_one(executor)
            .await?;

        Ok(company)
    }

    pub async fn _read_all(&self, filter: CompanyFilters) -> DbResult<Vec<Company>> {
        let executor = self.pool.as_ref();

        let companies = sqlx::query_as!(
            Company,
            "SELECT * FROM company LIMIT $1 OFFSET $2;",
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(companies)
    }

    pub async fn _update(&self, company_id: Uuid, data: CompanyData) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        if data.name.is_none()
            && data.description.is_none()
            && data.phone.is_none()
            && data.email.is_none()
            && data.avatar_url.is_none()
            && data.website.is_none()
            && data.crn.is_none()
            && data.vatin.is_none()
        {
            return self.read_one_db(company_id).await;
        }

        let company = sqlx::query_as!(
            Company,
            "UPDATE
                company 
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                phone = COALESCE($3, phone),
                email = COALESCE($4, email),
                avatar_url = COALESCE($5, avatar_url),
                website = COALESCE($6, website),
                crn = COALESCE($7, crn),
                vatin = COALESCE($8, vatin),
                edited_at = NOW()
            WHERE
                id = $9
            RETURNING *;
            ",
            data.name,
            data.description,
            data.phone,
            data.email,
            data.avatar_url,
            data.website,
            data.crn,
            data.vatin,
            company_id
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn _delete(&self, company_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            "UPDATE company SET deleted_at = NOW(), edited_at = NOW() WHERE id = $1;",
            company_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
