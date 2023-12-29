use crate::common::DbResult;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    AssociatedCompany, AssociatedCompanyData, AssociatedCompanyExtented, AssociatedCompanyFilter,
    AssociatedCompanyFlattened, NewAssociatedCompany,
};

use crate::models::Association;

#[derive(Clone)]
pub struct AssociatedCompanyRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for AssociatedCompanyRepository {
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

impl AssociatedCompanyRepository {
    pub async fn create(&self, data: NewAssociatedCompany) -> DbResult<AssociatedCompany> {
        let executor = self.pool.as_ref();

        let new_associated_company: AssociatedCompany = sqlx::query_as!(
            AssociatedCompany,
            r#" INSERT INTO associated_company (
                company_id, event_id, type
                ) VALUES 
                ($1, $2, $3) 
            RETURNING 
                company_id, 
                event_id, 
                type as "association_type!: Association", 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.company_id,
            data.event_id,
            data.association_type as Association,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_associated_company)
    }

    pub async fn read_one(
        &self,
        company_id: Uuid,
        event_id: Uuid,
    ) -> DbResult<AssociatedCompanyExtented> {
        // TODO REDIS here
        self.read_one_db(company_id, event_id).await
    }

    pub async fn read_one_db(
        &self,
        company_id: Uuid,
        event_id: Uuid,
    ) -> DbResult<AssociatedCompanyExtented> {
        let executor = self.pool.as_ref();

        let associated_company: AssociatedCompanyFlattened = sqlx::query_as!(
            AssociatedCompanyFlattened,
            r#" SELECT 
                company.id as "company_id!", 
                company.name as "company_name!", 
                company.description as "company_description", 
                company.phone as "company_phone!", 
                company.email as "company_email!", 
                company.avatar_url as "company_avatar_url", 
                company.website as "company_website", 
                company.crn as "company_crn!", 
                company.vatin as "company_vatin!", 
                company.created_at as "company_created_at!", 
                company.edited_at as "company_edited_at!", 
                company.deleted_at as "company_deleted_at", 
                event.id as "event_id!", 
                event.name as "event_name!", 
                event.description as "event_description", 
                event.website as "event_website", 
                event.accepts_staff as "event_accepts_staff!", 
                event.start_date as "event_start_date!", 
                event.end_date as "event_end_date!", 
                event.avatar_url as "event_avatar_url", 
                event.created_at as "event_created_at!", 
                event.edited_at as "event_edited_at!", 
                event.deleted_at as "event_deleted_at", 
                associated_company.type as "association_type!: Association", 
                associated_company.created_at as "created_at!", 
                associated_company.edited_at as "edited_at!", 
                associated_company.deleted_at as "deleted_at" 
            FROM associated_company 
            INNER JOIN company ON associated_company.company_id = company.id 
            INNER JOIN event ON associated_company.event_id = event.id 
            WHERE associated_company.company_id = $1 AND associated_company.event_id = $2;
            "#,
            company_id,
            event_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(associated_company.into())
    }

    pub async fn read_all(
        &self,
        filter: AssociatedCompanyFilter,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        // TODO REDIS here
        self.read_all_db(filter).await
    }

    pub async fn read_all_db(
        &self,
        filter: AssociatedCompanyFilter,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        let executor = self.pool.as_ref();

        let associated_companies: Vec<AssociatedCompanyFlattened> = sqlx::query_as!(
            AssociatedCompanyFlattened,
            r#" SELECT 
                company.id as "company_id!", 
                company.name as "company_name!", 
                company.description as "company_description", 
                company.phone as "company_phone!", 
                company.email as "company_email!", 
                company.avatar_url as "company_avatar_url", 
                company.website as "company_website", 
                company.crn as "company_crn!", 
                company.vatin as "company_vatin!", 
                company.created_at as "company_created_at!", 
                company.edited_at as "company_edited_at!", 
                company.deleted_at as "company_deleted_at", 
                event.id as "event_id!", 
                event.name as "event_name!", 
                event.description as "event_description", 
                event.website as "event_website", 
                event.accepts_staff as "event_accepts_staff!", 
                event.start_date as "event_start_date!", 
                event.end_date as "event_end_date!", 
                event.avatar_url as "event_avatar_url", 
                event.created_at as "event_created_at!", 
                event.edited_at as "event_edited_at!", 
                event.deleted_at as "event_deleted_at", 
                associated_company.type as "association_type!: Association", 
                associated_company.created_at as "created_at!", 
                associated_company.edited_at as "edited_at!", 
                associated_company.deleted_at as "deleted_at" 
            FROM associated_company 
            INNER JOIN company ON associated_company.company_id = company.id 
            INNER JOIN event ON associated_company.event_id = event.id
            LIMIT $1 OFFSET $2;
            "#,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(associated_companies
            .into_iter()
            .map(|associated_company| associated_company.into())
            .collect())
    }

    pub async fn read_all_for_event(
        &self,
        filter: AssociatedCompanyFilter,
        event_id: Uuid,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        // TODO REDIS here
        self.read_all_db_for_event(filter, event_id).await
    }

    pub async fn read_all_db_for_event(
        &self,
        filter: AssociatedCompanyFilter,
        event_id: Uuid,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        let executor = self.pool.as_ref();

        let associated_companies: Vec<AssociatedCompanyFlattened> = sqlx::query_as!(
            AssociatedCompanyFlattened,
            r#" SELECT 
                company.id as "company_id!", 
                company.name as "company_name!", 
                company.description as "company_description", 
                company.phone as "company_phone!", 
                company.email as "company_email!", 
                company.avatar_url as "company_avatar_url", 
                company.website as "company_website", 
                company.crn as "company_crn!", 
                company.vatin as "company_vatin!", 
                company.created_at as "company_created_at!", 
                company.edited_at as "company_edited_at!", 
                company.deleted_at as "company_deleted_at", 
                event.id as "event_id!", 
                event.name as "event_name!", 
                event.description as "event_description", 
                event.website as "event_website", 
                event.accepts_staff as "event_accepts_staff!", 
                event.start_date as "event_start_date!", 
                event.end_date as "event_end_date!", 
                event.avatar_url as "event_avatar_url", 
                event.created_at as "event_created_at!", 
                event.edited_at as "event_edited_at!", 
                event.deleted_at as "event_deleted_at", 
                associated_company.type as "association_type!: Association", 
                associated_company.created_at as "created_at!", 
                associated_company.edited_at as "edited_at!", 
                associated_company.deleted_at as "deleted_at" 
            FROM associated_company 
            INNER JOIN company ON associated_company.company_id = company.id 
            INNER JOIN event ON associated_company.event_id = event.id 
            WHERE associated_company.event_id = $1
            LIMIT $2 OFFSET $3;
            "#,
            event_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(associated_companies
            .into_iter()
            .map(|associated_company| associated_company.into())
            .collect())
    }

    pub async fn read_all_for_company(
        &self,
        filter: AssociatedCompanyFilter,
        company_id: Uuid,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        // TODO REDIS here
        self.read_all_db_for_company(filter, company_id).await
    }

    pub async fn read_all_db_for_company(
        &self,
        filter: AssociatedCompanyFilter,
        company_id: Uuid,
    ) -> DbResult<Vec<AssociatedCompanyExtented>> {
        let executor = self.pool.as_ref();

        let associated_companies: Vec<AssociatedCompanyFlattened> = sqlx::query_as!(
            AssociatedCompanyFlattened,
            r#" SELECT 
                company.id as "company_id!", 
                company.name as "company_name!", 
                company.description as "company_description", 
                company.phone as "company_phone!", 
                company.email as "company_email!", 
                company.avatar_url as "company_avatar_url", 
                company.website as "company_website", 
                company.crn as "company_crn!", 
                company.vatin as "company_vatin!", 
                company.created_at as "company_created_at!", 
                company.edited_at as "company_edited_at!", 
                company.deleted_at as "company_deleted_at", 
                event.id as "event_id!", 
                event.name as "event_name!", 
                event.description as "event_description", 
                event.website as "event_website", 
                event.accepts_staff as "event_accepts_staff!", 
                event.start_date as "event_start_date!", 
                event.end_date as "event_end_date!", 
                event.avatar_url as "event_avatar_url", 
                event.created_at as "event_created_at!", 
                event.edited_at as "event_edited_at!", 
                event.deleted_at as "event_deleted_at", 
                associated_company.type as "association_type!: Association", 
                associated_company.created_at as "created_at!", 
                associated_company.edited_at as "edited_at!", 
                associated_company.deleted_at as "deleted_at" 
            FROM associated_company 
            INNER JOIN company ON associated_company.company_id = company.id 
            INNER JOIN event ON associated_company.event_id = event.id 
            WHERE associated_company.company_id = $1
            LIMIT $2 OFFSET $3;
            "#,
            company_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(associated_companies
            .into_iter()
            .map(|associated_company| associated_company.into())
            .collect())
    }

    pub async fn update(
        &self,
        company_id: Uuid,
        event_id: Uuid,
        data: AssociatedCompanyData,
    ) -> DbResult<AssociatedCompany> {
        let executor = self.pool.as_ref();

        let associated_company_check = self.read_one_db(company_id, event_id).await?;

        if data.association_type.is_none() {
            // TODO: Return better error
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_associated_company: AssociatedCompany = sqlx::query_as!(
            AssociatedCompany,
            r#" UPDATE associated_company SET 
                type = $1, edited_at = NOW()
            WHERE company_id = $2 AND event_id = $3
            RETURNING 
                company_id, 
                event_id, 
                type as "association_type!: Association", 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.association_type.unwrap() as Association,
            company_id,
            event_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated_associated_company)
    }

    pub async fn delete(&self, company_id: Uuid, event_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let associated_company_check = self.read_one_db(company_id, event_id).await?;

        if associated_company_check.deleted_at.is_some() {
            // TODO - return better error
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query_as!(
            AssociatedCompany,
            r#" UPDATE associated_company SET 
                deleted_at = NOW(),
                edited_at = NOW()
            WHERE company_id = $1 AND event_id = $2
            "#,
            company_id,
            event_id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
