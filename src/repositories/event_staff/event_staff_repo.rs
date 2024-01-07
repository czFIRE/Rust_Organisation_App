use crate::common::DbResult;
use async_trait::async_trait;
use sqlx::{postgres::PgPool, Postgres, Transaction};
use std::{sync::Arc, ops::DerefMut};
use uuid::Uuid;

use crate::repositories::event_staff::models::StaffInfo;

use super::models::{
    NewStaff, StaffData, StaffExtended, StaffFilter, StaffUserCompanyFlattened,
};

use crate::models::{AcceptanceStatus, EventRole, Gender, UserRole, UserStatus};

#[derive(Clone)]
pub struct StaffRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for StaffRepository {
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

impl StaffRepository {
    pub async fn create(&self, event_id: Uuid, data: NewStaff) -> DbResult<StaffExtended> {
        let mut tx = self.pool.begin().await?;

        let staff_info: StaffInfo = sqlx::query_as!(
            StaffInfo,
            r#" INSERT INTO event_staff 
                ( user_id, company_id, event_id, role ) 
            VALUES 
                ($1, $2, $3, $4)
            RETURNING id;
            "#,
            data.user_id,
            data.company_id,
            event_id,
            data.role as EventRole,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let new_staff = self.read_one_tx(staff_info.id, tx).await?;

        Ok(new_staff)
    }

    pub async fn read_one(&self, event_staff_id: Uuid) -> DbResult<StaffExtended> {
        // Redis here
        self.read_one_db(event_staff_id).await
    }

    async fn read_one_db(&self, event_staff_id: Uuid) -> DbResult<StaffExtended> {
        let executor = self.pool.as_ref();

        let staff: StaffUserCompanyFlattened = sqlx::query_as!(
            StaffUserCompanyFlattened,
            r#"
            SELECT 
                event_staff.id AS staff_id, 
                event_staff.user_id AS staff_user_id, 
                event_staff.company_id AS staff_company_id, 
                event_staff.event_id AS staff_event_id, 
                event_staff.role AS "staff_role!: EventRole", 
                event_staff.status AS "staff_status!: AcceptanceStatus", 
                event_staff.decided_by AS staff_decided_by, 
                event_staff.created_at AS staff_created_at, 
                event_staff.edited_at AS staff_edited_at, 
                event_staff.deleted_at AS staff_deleted_at, 
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
                decider.id AS "decider_id?",
                decider.name AS "decider_name?",
                decider.status AS "decider_status?: UserStatus",
                decider.birth AS "decider_birth?",
                decider.gender AS "decider_gender?: Gender",
                decider.avatar_url AS "decider_avatar_url?",
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
                event_staff 
                INNER JOIN user_record ON event_staff.user_id = user_record.id 
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN (event_staff AS decider_staff
                INNER JOIN user_record AS decider ON decider_staff.user_id = decider.id)
                ON event_staff.decided_by = decider_staff.id
            WHERE 
                event_staff.id = $1
                AND event_staff.deleted_at IS NULL;
            "#,
            event_staff_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(staff.into())
    }

    async fn read_one_tx(&self, event_staff_id: Uuid, mut tx: Transaction<'_, Postgres>) -> DbResult<StaffExtended> {
        let staff: StaffUserCompanyFlattened = sqlx::query_as!(
            StaffUserCompanyFlattened,
            r#"
            SELECT 
                event_staff.id AS staff_id, 
                event_staff.user_id AS staff_user_id, 
                event_staff.company_id AS staff_company_id, 
                event_staff.event_id AS staff_event_id, 
                event_staff.role AS "staff_role!: EventRole", 
                event_staff.status AS "staff_status!: AcceptanceStatus", 
                event_staff.decided_by AS staff_decided_by, 
                event_staff.created_at AS staff_created_at, 
                event_staff.edited_at AS staff_edited_at, 
                event_staff.deleted_at AS staff_deleted_at, 
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
                decider.id AS "decider_id?",
                decider.name AS "decider_name?",
                decider.status AS "decider_status?: UserStatus",
                decider.birth AS "decider_birth?",
                decider.gender AS "decider_gender?: Gender",
                decider.avatar_url AS "decider_avatar_url?",
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
                event_staff 
                INNER JOIN user_record ON event_staff.user_id = user_record.id 
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN (event_staff AS decider_staff
                INNER JOIN user_record AS decider ON decider_staff.user_id = decider.id)
                ON event_staff.decided_by = decider_staff.id
            WHERE 
                event_staff.id = $1
                AND event_staff.deleted_at IS NULL;
            "#,
            event_staff_id,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        tx.commit().await?;

        Ok(staff.into())
    }

    pub async fn read_all_for_event(
        &self,
        event_id: Uuid,
        filter: StaffFilter,
    ) -> DbResult<Vec<StaffExtended>> {
        let executor = self.pool.as_ref();

        let staff: Vec<StaffUserCompanyFlattened> = sqlx::query_as!(
            StaffUserCompanyFlattened,
            r#"
            SELECT 
                event_staff.id AS staff_id, 
                event_staff.user_id AS staff_user_id, 
                event_staff.company_id AS staff_company_id, 
                event_staff.event_id AS staff_event_id, 
                event_staff.role AS "staff_role!: EventRole", 
                event_staff.status AS "staff_status!: AcceptanceStatus", 
                event_staff.decided_by AS staff_decided_by, 
                event_staff.created_at AS staff_created_at, 
                event_staff.edited_at AS staff_edited_at, 
                event_staff.deleted_at AS staff_deleted_at, 
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
                decider.id AS "decider_id?",
                decider.name AS "decider_name?",
                decider.status AS "decider_status?: UserStatus",
                decider.birth AS "decider_birth?",
                decider.gender AS "decider_gender?: Gender",
                decider.avatar_url AS "decider_avatar_url?",
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
                event_staff 
                INNER JOIN user_record ON event_staff.user_id = user_record.id 
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN (event_staff AS decider_staff
                INNER JOIN user_record AS decider ON decider_staff.user_id = decider.id)
                ON event_staff.decided_by = decider_staff.id
            WHERE 
                event_staff.event_id = $1
                AND event_staff.deleted_at IS NULL
            LIMIT $2 OFFSET $3;
            "#,
            event_id,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(staff.into_iter().map(|s| s.into()).collect())
    }

    pub async fn update(&self, event_staff_id: Uuid, data: StaffData) -> DbResult<StaffExtended> {
        if data.role.is_none() && data.status.is_none() {
            // TODO - better error
            return Err(sqlx::Error::TypeNotFound { type_name: "User Error".to_string() });
        }

        let mut tx = self.pool.begin().await?;

        let staff_info: Option<StaffInfo> = sqlx::query_as!(
            StaffInfo,
            r#" UPDATE event_staff SET 
                role = COALESCE($1, role), 
                status = COALESCE($2, status), 
                decided_by = COALESCE($3, decided_by), 
                edited_at = now() 
            WHERE id = $4 
              AND deleted_at IS NULL
            RETURNING id;
            "#,
            data.role as Option<EventRole>,
            data.status as Option<AcceptanceStatus>,
            data.decided_by,
            event_staff_id,
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if staff_info.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_staff = self.read_one_tx(staff_info.unwrap().id, tx).await?;

        Ok(updated_staff)
    }

    pub async fn delete(&self, event_staff_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let result: Option<StaffInfo> = sqlx::query_as!(
            StaffInfo,
            r#" UPDATE event_staff SET 
                deleted_at = now(), 
                edited_at = now() 
            WHERE id = $1 
              AND deleted_at IS NULL
            RETURNING id;
            "#,
            event_staff_id,
        )
        .fetch_optional(executor)
        .await?;

        if result.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn delete_rejected(&self, event_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _result: Vec<StaffInfo> = sqlx::query_as!(
            StaffInfo,
            r#" UPDATE event_staff SET 
                deleted_at = now(), 
                edited_at = now() 
            WHERE event_id = $1 
              AND deleted_at IS NULL
              AND status = 'pending'
            RETURNING id;
            "#,
            event_id,
        )
        .fetch_all(executor)
        .await?;

        Ok(())
    }
}
