use crate::common::DbResult;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::postgres::PgPool;
use std::{ops::DerefMut, sync::Arc};
use uuid::Uuid;

use crate::models::{EmployeeLevel, EmploymentContract};

use super::models::{
    Address, AddressData, AddressUpdateData, Company, CompanyData, CompanyExtended, CompanyFilter,
    NewCompany,
};

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

    pub async fn create(
        &self,
        data: NewCompany,
        address: AddressData,
        first_employee_id: Uuid,
    ) -> DbResult<CompanyExtended> {
        let mut tx = self.pool.begin().await?;

        let company = sqlx::query_as!(
            Company,
            "INSERT INTO company (name, description, phone, email, website, crn, vatin) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
            data.name,
            if let Some(description) = data.description {
                description
            } else {
                "".to_string()
            },
            data.phone,
            data.email,
            if let Some(website) = data.website {
                website
            } else {
                "".to_string()
            },
            data.crn,
            data.vatin
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let address = sqlx::query_as!(
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
        .fetch_one(tx.deref_mut())
        .await?;

        let starter_date = Utc::now().naive_local().date();

        sqlx::query!(
            r#"
            INSERT INTO employment ( 
                user_id, company_id, manager_id, hourly_wage,
                start_date, end_date, description, type, level
            )
            VALUES ($1, $2, $3, 200, $4, date('9999-12-31'), 'First administrator', $5, $6);
            "#,
            first_employee_id,
            company.id,
            first_employee_id,
            starter_date,
            EmploymentContract::Hpp as EmploymentContract,
            EmployeeLevel::CompanyAdministrator as EmployeeLevel,
        )
        .execute(tx.deref_mut())
        .await?;

        tx.commit().await?;

        let company_extended = CompanyExtended {
            company_id: company.id,
            name: company.name,
            description: company.description,
            phone: company.phone,
            email: company.email,
            avatar_url: company.avatar_url,
            website: company.website,
            crn: company.crn,
            vatin: company.vatin,
            created_at: company.created_at,
            edited_at: company.edited_at,
            deleted_at: company.deleted_at,
            country: address.country,
            region: address.region,
            city: address.city,
            street: address.street,
            postal_code: address.postal_code,
            street_number: address.street_number,
        };

        Ok(company_extended)
    }

    pub async fn _read_one(&self, company_id: Uuid) -> DbResult<Company> {
        // TODO - Redis here
        self._read_one_db(company_id).await
    }

    pub async fn _read_one_db(&self, company_id: Uuid) -> DbResult<Company> {
        let executor = self.pool.as_ref();

        let company = sqlx::query_as!(
            Company,
            "SELECT * 
             FROM company 
             WHERE id = $1
               AND deleted_at IS NULL;",
            company_id
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn read_one_extended(&self, company_id: Uuid) -> DbResult<CompanyExtended> {
        // TODO - Redis here
        self.read_one_db_extended(company_id).await
    }

    pub async fn read_one_db_extended(&self, company_id: Uuid) -> DbResult<CompanyExtended> {
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
            FROM company 
                 INNER JOIN address on company.id = address.company_id 
                 WHERE company.id = $1
                   AND deleted_at IS NULL;",
            company_id
        )
        .fetch_one(executor)
        .await?;

        Ok(company)
    }

    pub async fn read_all(&self, filter: CompanyFilter) -> DbResult<Vec<Company>> {
        let mut name_filter = if filter.name.is_some() {
            filter.name.expect("Should be some").clone()
        } else {
            "".to_string()
        };
        name_filter.push('%');

        let executor = self.pool.as_ref();

        let companies = sqlx::query_as!(
            Company,
            "SELECT * FROM company 
             WHERE deleted_at IS NULL 
               AND name LIKE $3
             LIMIT $1 OFFSET $2;",
            filter.limit,
            filter.offset,
            name_filter,
        )
        .fetch_all(executor)
        .await?;

        Ok(companies)
    }

    pub async fn _read_all_extended(
        &self,
        filter: CompanyFilter,
    ) -> DbResult<Vec<CompanyExtended>> {
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

    fn is_company_data_empty(data: CompanyData) -> bool {
        data.name.is_none()
            && data.description.is_none()
            && data.phone.is_none()
            && data.email.is_none()
            && data.avatar_url.is_none()
            && data.website.is_none()
            && data.crn.is_none()
            && data.vatin.is_none()
    }

    fn is_address_data_empty(data: AddressUpdateData) -> bool {
        data.country.is_none()
            && data.region.is_none()
            && data.city.is_none()
            && data.postal_code.is_none()
            && data.street.is_none()
            && data.street_number.is_none()
    }

    pub async fn update(
        &self,
        company_id: Uuid,
        data: CompanyData,
        address: AddressUpdateData,
    ) -> DbResult<CompanyExtended> {
        let is_company_update_empty = Self::is_company_data_empty(data.clone());
        let is_address_update_empty = Self::is_address_data_empty(address.clone());

        if is_company_update_empty && is_address_update_empty {
            // ToDo: Improve this.
            return Err(sqlx::Error::TypeNotFound {
                type_name: "Empty data".to_string(),
            });
        }

        let mut tx = self.pool.begin().await?;

        if !is_company_update_empty {
            let updated = sqlx::query_as!(
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
                WHERE id = $9
                  AND deleted_at IS NULL
                RETURNING id,
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
                          company.deleted_at;
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
            .fetch_optional(tx.deref_mut())
            .await?;

            if updated.is_none() {
                tx.commit().await?;
                return Err(sqlx::Error::RowNotFound);
            }
        }

        if !is_address_update_empty {
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
                RETURNING company_id,
                          country,
                          region,
                          city,
                          street,
                          postal_code,
                          street_number;
                ",
                address.country,
                address.region,
                address.city,
                address.street,
                address.postal_code,
                address.street_number,
                company_id
            )
            .fetch_optional(tx.deref_mut())
            .await?;

            if _address.is_none() {
                tx.commit().await?;
                return Err(sqlx::Error::RowNotFound);
            }
        }

        //ToDo: This is currently duplicate.
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
            FROM company 
                 INNER JOIN address on company.id = address.company_id 
                 WHERE company.id = $1
                   AND deleted_at IS NULL;",
            company_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        tx.commit().await?;

        Ok(company)
    }

    pub async fn delete(&self, company_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let result = sqlx::query_as!(
            Company,
            "UPDATE company 
            SET deleted_at = NOW(), 
                edited_at = NOW() 
            WHERE id = $1
              AND deleted_at IS NULL
            RETURNING id,
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
                      company.deleted_at;",
            company_id
        )
        .fetch_optional(executor)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
