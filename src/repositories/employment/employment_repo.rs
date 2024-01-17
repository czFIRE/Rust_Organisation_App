use crate::common::DbResult;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::Transaction;
use std::sync::Arc;
use uuid::Uuid;

use std::ops::DerefMut;

use super::models::{
    Employment, EmploymentData, EmploymentExtended, EmploymentFilter,
    EmploymentUserCompanyFlattened, NewEmployment, EmploymentContractAndHourlyWage,
};

use crate::models::{EmployeeLevel, EmploymentContract, Gender, UserRole, UserStatus};

#[derive(Clone)]
pub struct EmploymentRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for EmploymentRepository {
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

// Read one lite employment using an existing transaction handle.
pub async fn read_one_lite_db_using_tx(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
    company_id: Uuid)
    -> DbResult<EmploymentContractAndHourlyWage> {

    let employment_lite = sqlx::query_as!(
        EmploymentContractAndHourlyWage,
        r#"
            SELECT hourly_wage,
                   type AS "employment_type!: EmploymentContract"
            FROM employment
            WHERE user_id = $1
              AND company_id = $2;
            "#,
        user_id,
        company_id
    )
        .fetch_one(tx.deref_mut())
        .await;

    employment_lite
}

impl EmploymentRepository {
    pub async fn create(&self, data: NewEmployment) -> DbResult<Employment> {
        let executor = self.pool.as_ref();

        let new_employment: Employment = sqlx::query_as!(
            Employment,
            r#" INSERT INTO employment (
                    user_id, company_id, manager_id, hourly_wage, 
                    start_date, end_date, description, 
                    type, level
                ) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
                RETURNING user_id, 
                    company_id, 
                    manager_id, 
                    hourly_wage, 
                    start_date, 
                    end_date, 
                    description, 
                    type AS "employment_type!: EmploymentContract", 
                    level AS "level!: EmployeeLevel", 
                    created_at, 
                    edited_at, 
                    deleted_at;"#,
            data.user_id,
            data.company_id,
            data.manager_id,
            data.hourly_wage,
            data.start_date,
            data.end_date,
            data.description,
            data.employment_type as EmploymentContract,
            data.level as EmployeeLevel,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_employment)
    }

    pub async fn read_one(
        &self,
        _user_uuid: Uuid,
        _company_uuid: Uuid,
    ) -> DbResult<EmploymentExtended> {
        // Implement redis here.
        self.read_one_db(_user_uuid, _company_uuid).await
    }

    // Actual DB access.
    async fn read_one_db(
        &self,
        user_uuid: Uuid,
        company_uuid: Uuid,
    ) -> DbResult<EmploymentExtended> {
        let executor = self.pool.as_ref();

        let employment = sqlx::query_as!(
            EmploymentUserCompanyFlattened,
            r#"
            SELECT 
                employment.user_id AS employment_user_id, 
                employment.company_id AS employment_company_id, 
                employment.manager_id AS employment_manager_id, 
                employment.hourly_wage AS employment_hourly_wage, 
                employment.start_date AS employment_start_date, 
                employment.end_date AS employment_end_date, 
                employment.description AS employment_description, 
                employment.type AS "employment_type!: EmploymentContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS "manager_id?", 
                user_record.name AS "manager_name?", 
                user_record.email AS "manager_email?", 
                user_record.birth AS "manager_birth?", 
                user_record.avatar_url AS "manager_avatar_url?", 
                user_record.gender AS "manager_gender?: Gender", 
                user_record.role AS "manager_role?: UserRole", 
                user_record.status AS "manager_status?: UserStatus", 
                user_record.created_at AS "manager_created_at?", 
                user_record.edited_at AS "manager_edited_at?", 
                user_record.deleted_at AS "manager_deleted_at?", 
                company.id AS company_id, 
                company.name AS company_name, 
                company.description AS company_description, 
                company.phone AS company_phone, 
                company.email AS company_email, 
                company.avatar_url AS company_avatar_url, 
                company.website AS company_website, 
                company.crn AS company_crn, 
                company.vatin AS company_vatin, 
                company.created_at AS company_created_at, 
                company.edited_at AS company_edited_at, 
                company.deleted_at AS company_deleted_at 
            FROM 
                employment 
                INNER JOIN company ON employment.company_id = company.id 
                LEFT OUTER JOIN user_record ON employment.manager_id = user_record.id 
            WHERE 
                employment.user_id = $1 
                AND employment.company_id = $2  
                AND employment.deleted_at IS NULL
                AND user_record.deleted_at IS NULL
                AND company.deleted_at IS NULL    
            "#,
            user_uuid,
            company_uuid,
        )
        .fetch_optional(executor)
        .await?;

        if employment.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(employment.unwrap().into())
    }

    // Retrieves all employments for a given user.
    pub async fn read_all_for_user(
        &self,
        user_uuid: Uuid,
        filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        let executor = self.pool.as_ref();

        let employment: Vec<EmploymentUserCompanyFlattened> = sqlx::query_as!(
            EmploymentUserCompanyFlattened,
            r#"
            SELECT 
                employment.user_id AS employment_user_id, 
                employment.company_id AS employment_company_id, 
                employment.manager_id AS employment_manager_id, 
                employment.hourly_wage AS employment_hourly_wage, 
                employment.start_date AS employment_start_date, 
                employment.end_date AS employment_end_date, 
                employment.description AS employment_description, 
                employment.type AS "employment_type!: EmploymentContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS "manager_id?", 
                user_record.name AS "manager_name?", 
                user_record.email AS "manager_email?", 
                user_record.birth AS "manager_birth?", 
                user_record.avatar_url AS "manager_avatar_url?", 
                user_record.gender AS "manager_gender?: Gender", 
                user_record.role AS "manager_role?: UserRole", 
                user_record.status AS "manager_status?: UserStatus", 
                user_record.created_at AS "manager_created_at?", 
                user_record.edited_at AS "manager_edited_at?", 
                user_record.deleted_at AS "manager_deleted_at?",
                company.id AS company_id, 
                company.name AS company_name, 
                company.description AS company_description, 
                company.phone AS company_phone, 
                company.email AS company_email, 
                company.avatar_url AS company_avatar_url, 
                company.website AS company_website, 
                company.crn AS company_crn, 
                company.vatin AS company_vatin, 
                company.created_at AS company_created_at, 
                company.edited_at AS company_edited_at, 
                company.deleted_at AS company_deleted_at 
            FROM 
                employment 
                INNER JOIN company ON employment.company_id = company.id 
                LEFT OUTER JOIN user_record ON employment.manager_id = user_record.id 
            WHERE 
                employment.user_id = $1 
                AND employment.deleted_at IS NULL
            LIMIT $2 OFFSET $3          
            "#,
            user_uuid,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(employment.into_iter().map(|e| e.into()).collect())
    }

    // Retrieves all employments for a given company.
    pub async fn _read_all_for_company(
        &self,
        company_uuid: Uuid,
        filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        let executor = self.pool.as_ref();

        let employment: Vec<EmploymentUserCompanyFlattened> = sqlx::query_as!(
            EmploymentUserCompanyFlattened,
            r#"
            SELECT 
                employment.user_id AS employment_user_id, 
                employment.company_id AS employment_company_id, 
                employment.manager_id AS employment_manager_id, 
                employment.hourly_wage AS employment_hourly_wage, 
                employment.start_date AS employment_start_date, 
                employment.end_date AS employment_end_date, 
                employment.description AS employment_description, 
                employment.type AS "employment_type!: EmploymentContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS "manager_id?", 
                user_record.name AS "manager_name?", 
                user_record.email AS "manager_email?", 
                user_record.birth AS "manager_birth?", 
                user_record.avatar_url AS "manager_avatar_url?", 
                user_record.gender AS "manager_gender?: Gender", 
                user_record.role AS "manager_role?: UserRole", 
                user_record.status AS "manager_status?: UserStatus", 
                user_record.created_at AS "manager_created_at?", 
                user_record.edited_at AS "manager_edited_at?", 
                user_record.deleted_at AS "manager_deleted_at?",
                company.id AS company_id, 
                company.name AS company_name, 
                company.description AS company_description, 
                company.phone AS company_phone, 
                company.email AS company_email, 
                company.avatar_url AS company_avatar_url, 
                company.website AS company_website, 
                company.crn AS company_crn, 
                company.vatin AS company_vatin, 
                company.created_at AS company_created_at, 
                company.edited_at AS company_edited_at, 
                company.deleted_at AS company_deleted_at 
            FROM 
                employment 
                INNER JOIN company ON employment.company_id = company.id 
                LEFT OUTER JOIN user_record ON employment.manager_id = user_record.id 
            WHERE 
                employment.company_id = $1 
                AND employment.deleted_at IS NULL
            LIMIT $2 OFFSET $3          
            "#,
            company_uuid,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(employment.into_iter().map(|e| e.into()).collect())
    }

    // Retrieves all subordinates for a given manager.
    pub async fn read_subordinates(
        &self,
        manager_uuid: Uuid,
        company_uuid: Uuid,
        filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        let executor = self.pool.as_ref();

        let employment: Vec<EmploymentUserCompanyFlattened> = sqlx::query_as!(
            EmploymentUserCompanyFlattened,
            r#"
            SELECT 
                employment.user_id AS employment_user_id, 
                employment.company_id AS employment_company_id, 
                employment.manager_id AS employment_manager_id, 
                employment.hourly_wage AS employment_hourly_wage, 
                employment.start_date AS employment_start_date, 
                employment.end_date AS employment_end_date, 
                employment.description AS employment_description, 
                employment.type AS "employment_type!: EmploymentContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS "manager_id?", 
                user_record.name AS "manager_name?", 
                user_record.email AS "manager_email?", 
                user_record.birth AS "manager_birth?", 
                user_record.avatar_url AS "manager_avatar_url?", 
                user_record.gender AS "manager_gender?: Gender", 
                user_record.role AS "manager_role?: UserRole", 
                user_record.status AS "manager_status?: UserStatus", 
                user_record.created_at AS "manager_created_at?", 
                user_record.edited_at AS "manager_edited_at?", 
                user_record.deleted_at AS "manager_deleted_at?",
                company.id AS company_id, 
                company.name AS company_name, 
                company.description AS company_description, 
                company.phone AS company_phone, 
                company.email AS company_email, 
                company.avatar_url AS company_avatar_url, 
                company.website AS company_website, 
                company.crn AS company_crn, 
                company.vatin AS company_vatin, 
                company.created_at AS company_created_at, 
                company.edited_at AS company_edited_at, 
                company.deleted_at AS company_deleted_at 
            FROM 
                employment 
                INNER JOIN company ON employment.company_id = company.id 
                LEFT OUTER JOIN user_record ON employment.manager_id = user_record.id 
            WHERE 
                employment.manager_id = $1
                AND employment.company_id = $2
                AND employment.deleted_at IS NULL
            LIMIT $3 OFFSET $4          
            "#,
            manager_uuid,
            company_uuid,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(employment.into_iter().map(|e| e.into()).collect())
    }

    pub async fn update(
        &self,
        user_uuid: Uuid,
        company_uuid: Uuid,
        data: EmploymentData,
    ) -> DbResult<Employment> {
        if data.manager_id.is_none()
            && data.hourly_wage.is_none()
            && data.start_date.is_none()
            && data.end_date.is_none()
            && data.description.is_none()
            && data.employment_type.is_none()
            && data.level.is_none()
        {
            return Err(sqlx::Error::RowNotFound);
        }

        let executor = self.pool.as_ref();

        let updated_employment = sqlx::query_as!(
            Employment,
            r#" UPDATE employment SET 
                manager_id = COALESCE($3, manager_id), 
                hourly_wage = COALESCE($4, hourly_wage), 
                start_date = COALESCE($5, start_date), 
                end_date = COALESCE($6, end_date), 
                description = COALESCE($7, description), 
                type = COALESCE($8, type), 
                level = COALESCE($9, level),
                edited_at = now() 
                WHERE user_id=$1 
                  AND company_id=$2
                  AND deleted_at IS NULL 
                RETURNING 
                user_id, 
                company_id, 
                manager_id, 
                hourly_wage, 
                start_date, 
                end_date, 
                description, 
                type AS "employment_type!: EmploymentContract", 
                level AS "level!: EmployeeLevel", 
                created_at, 
                edited_at, 
                deleted_at;"#,
            user_uuid,
            company_uuid,
            data.manager_id,
            data.hourly_wage,
            data.start_date,
            data.end_date,
            data.description,
            data.employment_type as Option<EmploymentContract>,
            data.level as Option<EmployeeLevel>,
        )
        .fetch_optional(executor)
        .await?;

        if updated_employment.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(updated_employment.expect("Should be some."))
    }

    pub async fn delete(&self, user_uuid: Uuid, company_uuid: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let result = sqlx::query_as!(
            Employment,
            r#"UPDATE employment
            SET deleted_at = NOW(), edited_at = NOW()
            WHERE user_id = $1 AND company_id = $2
            AND deleted_at IS NULL
            RETURNING 
                user_id, 
                company_id, 
                manager_id, 
                hourly_wage, 
                start_date, 
                end_date, 
                description, 
                type AS "employment_type!: EmploymentContract", 
                level AS "level!: EmployeeLevel", 
                created_at, 
                edited_at, 
                deleted_at;"#,
            user_uuid,
            company_uuid,
        )
        .fetch_optional(executor)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
