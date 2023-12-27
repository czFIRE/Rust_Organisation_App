use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    Employment, EmploymentData, EmploymentExtended, EmploymentFilter,
    EmploymentUserCompanyFlattened, NewEmployment,
};

use crate::models::{EmployeeContract, EmployeeLevel, Gender, UserRole, UserStatus};

#[derive(Clone)]
pub struct EmploymentRepository {
    pub pool: Arc<PgPool>,
}

impl EmploymentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, data: NewEmployment) -> DbResult<Employment> {
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
                    type AS "employment_type!: EmployeeContract", 
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
            data.employment_type as EmployeeContract,
            data.level as EmployeeLevel,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_employment)
    }

    pub async fn _read_one(
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

        let employment: EmploymentUserCompanyFlattened = sqlx::query_as!(
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
                employment.type AS "employment_type!: EmployeeContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS user_id, 
                user_record.name AS user_name, 
                user_record.email AS user_email, 
                user_record.birth AS user_birth, 
                user_record.avatar_url AS user_avatar_url, 
                user_record.gender AS "user_gender!: Gender", 
                user_record.role AS "user_role!: UserRole", 
                user_record.status AS "user_status!: UserStatus", 
                user_record.created_at AS user_created_at, 
                user_record.edited_at AS user_edited_at, 
                user_record.deleted_at AS user_deleted_at, 
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
                INNER JOIN user_record ON employment.user_id = user_record.id 
            WHERE 
                employment.user_id = $1 
                AND employment.company_id = $2          
            "#,
            user_uuid,
            company_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(employment.into())
    }

    // Retrieves all employments for a given user.
    pub async fn _read_all_of_user(
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
                employment.type AS "employment_type!: EmployeeContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS user_id, 
                user_record.name AS user_name, 
                user_record.email AS user_email, 
                user_record.birth AS user_birth, 
                user_record.avatar_url AS user_avatar_url, 
                user_record.gender AS "user_gender!: Gender", 
                user_record.role AS "user_role!: UserRole", 
                user_record.status AS "user_status!: UserStatus", 
                user_record.created_at AS user_created_at, 
                user_record.edited_at AS user_edited_at, 
                user_record.deleted_at AS user_deleted_at, 
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
                INNER JOIN user_record ON employment.user_id = user_record.id 
            WHERE 
                employment.user_id = $1 
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
    pub async fn _read_all_of_company(
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
                employment.type AS "employment_type!: EmployeeContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS user_id, 
                user_record.name AS user_name, 
                user_record.email AS user_email, 
                user_record.birth AS user_birth, 
                user_record.avatar_url AS user_avatar_url, 
                user_record.gender AS "user_gender!: Gender", 
                user_record.role AS "user_role!: UserRole", 
                user_record.status AS "user_status!: UserStatus", 
                user_record.created_at AS user_created_at, 
                user_record.edited_at AS user_edited_at, 
                user_record.deleted_at AS user_deleted_at, 
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
                INNER JOIN user_record ON employment.user_id = user_record.id 
            WHERE 
                employment.company_id = $1 
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
    pub async fn _read_subordinates(
        &self,
        manager_uuid: Uuid,
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
                employment.type AS "employment_type!: EmployeeContract", 
                employment.level AS "employment_level!: EmployeeLevel", 
                employment.created_at AS employment_created_at, 
                employment.edited_at AS employment_edited_at, 
                employment.deleted_at AS employment_deleted_at, 
                user_record.id AS user_id, 
                user_record.name AS user_name, 
                user_record.email AS user_email, 
                user_record.birth AS user_birth, 
                user_record.avatar_url AS user_avatar_url, 
                user_record.gender AS "user_gender!: Gender", 
                user_record.role AS "user_role!: UserRole", 
                user_record.status AS "user_status!: UserStatus", 
                user_record.created_at AS user_created_at, 
                user_record.edited_at AS user_edited_at, 
                user_record.deleted_at AS user_deleted_at, 
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
                INNER JOIN user_record ON employment.user_id = user_record.id 
            WHERE 
                employment.manager_id = $1 
            LIMIT $2 OFFSET $3          
            "#,
            manager_uuid,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(employment.into_iter().map(|e| e.into()).collect())
    }

    pub async fn _update_employment(
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

        let updated_employment: Employment = sqlx::query_as!(
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
                WHERE user_id=$1 AND company_id=$2 RETURNING 
                user_id, 
                company_id, 
                manager_id, 
                hourly_wage, 
                start_date, 
                end_date, 
                description, 
                type AS "employment_type!: EmployeeContract", 
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
            data.employment_type as Option<EmployeeContract>,
            data.level as Option<EmployeeLevel>,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated_employment)
    }

    pub async fn _delete_employment(&self, user_uuid: Uuid, company_uuid: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            r#"UPDATE employment
            SET deleted_at = NOW(), edited_at = NOW()
            WHERE user_id = $1 AND company_id = $2
            AND deleted_at IS NULL"#,
            user_uuid,
            company_uuid,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
