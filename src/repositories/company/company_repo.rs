use crate::common::DbResult;
use crate::repositories::company::models::Company;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{Address, AddressData, CompanyData, CompanyExtended, CompanyFilters};

#[derive(Clone)]
pub struct CompanyRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for CompanyRepository {
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

impl CompanyRepository {
    // CRUD

    pub async fn _create(&self, data: CompanyData, address: AddressData) -> DbResult<Company> {
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

        let _address = sqlx::query_as!(
            Address,
            "INSERT INTO address (company_id, country, region, city, street, postal_code, street_number) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
            company.id,
            address.country,
            address.region,
            address.city,
            address.street,
            address.postal_code,
            address.street_number
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn read_one(&self, company_id: Uuid) -> DbResult<CompanyExtended> {
        // TODO - Redis here
        self.read_one_db(company_id).await
    }

    pub async fn read_one_db(&self, company_id: Uuid) -> DbResult<CompanyExtended> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(
            CompanyExtended, 
            "SELECT  
                company_id,
                name,
                description,
                phone,
                email,
                avatar_url,
                website,
                crn,
                vatin,
                company.created_at,
                company.edited_at,
                company.deleted_at,
                country,
                region,
                city,
                street,
                postal_code,
                street_number
            FROM company INNER JOIN address on company.id = address.company_id WHERE company.id = $1;", 
            company_id)
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

    pub async fn _read_all_extended(&self, filter: CompanyFilters) -> DbResult<Vec<CompanyExtended>> {
        let executor = self.pool.as_ref();

        let companies = sqlx::query_as!(
            CompanyExtended,
            "SELECT  
                company_id,
                name,
                description,
                phone,
                email,
                avatar_url,
                website,
                crn,
                vatin,
                company.created_at,
                company.edited_at,
                company.deleted_at,
                country,
                region,
                city,
                street,
                postal_code,
                street_number
            FROM company INNER JOIN address on company.id = address.company_id LIMIT $1 OFFSET $2;",
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(companies)
    }

    pub async fn _update(&self, company_id: Uuid, data: CompanyData, address: Option<AddressData>) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        if data.name.is_none()
            && data.description.is_none()
            && data.phone.is_none()
            && data.email.is_none()
            && data.avatar_url.is_none()
            && data.website.is_none()
            && data.crn.is_none()
            && data.vatin.is_none()
            && address.is_none()
        {
            // TODO - return better error
            return Err(sqlx::Error::RowNotFound);
        }

        let company_check = self.read_one(company_id).await?;

        if company_check.deleted_at.is_some() {
            // TODO - return better error
            return Err(sqlx::Error::RowNotFound);
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

        if let Some(address) = address {
            // TODO - coalesce is not needed here
            let _address = sqlx::query_as!(
                Address,
                "UPDATE
                    address 
                SET
                    country = COALESCE($1, country),
                    region = COALESCE($2, region),
                    city = COALESCE($3, city),
                    street = COALESCE($4, street),
                    postal_code = COALESCE($5, postal_code),
                    street_number = COALESCE($6, street_number)
                WHERE
                    company_id = $7
                RETURNING *;
                ",
                address.country,
                address.region,
                address.city,
                address.street,
                address.postal_code,
                address.street_number,
                company_id
            )
            .fetch_one(executor)
            .await?;
        }

        Ok(company)
    }

    pub async fn _delete(&self, company_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let company_check = self.read_one(company_id).await?;

        if company_check.deleted_at.is_some() {
            // TODO - return better error
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            "UPDATE company SET deleted_at = NOW(), edited_at = NOW() WHERE id = $1;",
            company_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
