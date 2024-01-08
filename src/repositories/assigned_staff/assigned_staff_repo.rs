use crate::{
    common::DbResult, repositories::assigned_staff::models::AssignedStaffStaffUserCompanyFlattened,
};
use async_trait::async_trait;
use sqlx::{postgres::PgPool, Postgres, Transaction};
use std::{sync::Arc, ops::DerefMut};
use uuid::Uuid;

use crate::models::{AcceptanceStatus, EventRole, Gender, UserRole, UserStatus};

use super::models::{
    AssignedStaff, AssignedStaffData, AssignedStaffExtended, AssignedStaffFilter, NewAssignedStaff,
};

#[derive(Clone)]
pub struct AssignedStaffRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for AssignedStaffRepository {
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

impl AssignedStaffRepository {
    // was AssignedStaffExtended
    pub async fn create(&self, data: NewAssignedStaff) -> DbResult<AssignedStaffExtended> {
        let mut tx = self.pool.begin().await?;

        let new_staff = sqlx::query_as!(
            AssignedStaff,
            r#"
            INSERT INTO assigned_staff (
                task_id, staff_id
            ) 
            VALUES 
                ($1, $2) 
            RETURNING 
                task_id, 
                staff_id, 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at,
                status AS "status!: AcceptanceStatus";
            "#,
            data.task_id,
            data.staff_id,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let assigned_staff = self.read_one_tx(new_staff.task_id, new_staff.staff_id, tx).await?;

        // TODO - are we returning the right thing here?
        Ok(assigned_staff)
    }

    pub async fn read_one(&self, task_id: Uuid, staff_id: Uuid) -> DbResult<AssignedStaffExtended> {
        // Redis here
        self.read_one_db(task_id, staff_id).await
    }

    async fn read_one_db(&self, task_id: Uuid, staff_id: Uuid) -> DbResult<AssignedStaffExtended> {
        let executor = self.pool.as_ref();

        //TODO Note to self, make sure that removing the ? from the end of decided_by:avatar_url did not break anything.
        let assigned_staff: AssignedStaffStaffUserCompanyFlattened = sqlx::query_as!(
            AssignedStaffStaffUserCompanyFlattened,
            r#"
            SELECT 
                assigned_staff.task_id AS assigned_staff_task_id,
                assigned_staff.staff_id AS assigned_staff_id, 
                assigned_staff.status AS "assigned_staff_status!: AcceptanceStatus", 
                assigned_staff.decided_by AS assigned_staff_decided_by, 
                assigned_staff.created_at AS assigned_staff_created_at,
                assigned_staff.edited_at AS assigned_staff_edited_at, 
                assigned_staff.deleted_at AS assigned_staff_deleted_at,
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
                company.deleted_at AS company_deleted_at,
                user_record_decided_by.id AS "decided_by_user_id?", 
                user_record_decided_by.name AS "decided_by_user_name?",
                user_record_decided_by.email AS "decided_by_user_email?", 
                user_record_decided_by.birth AS "decided_by_user_birth?", 
                user_record_decided_by.avatar_url AS "decided_by_user_avatar_url", 
                user_record_decided_by.gender AS "decided_by_user_gender?: Gender", 
                user_record_decided_by.role AS "decided_by_user_role?: UserRole", 
                user_record_decided_by.status AS "decided_by_user_status?: UserStatus", 
                user_record_decided_by.created_at AS "decided_by_user_created_at?",
                user_record_decided_by.edited_at AS "decided_by_user_edited_at?", 
                user_record_decided_by.deleted_at AS "decided_by_user_deleted_at?"
            FROM 
                assigned_staff 
                INNER JOIN event_staff ON assigned_staff.staff_id = event_staff.id
                INNER JOIN user_record ON event_staff.user_id = user_record.id
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN event_staff AS event_staff_decided_by ON assigned_staff.decided_by = event_staff_decided_by.id
                LEFT OUTER JOIN user_record AS user_record_decided_by ON event_staff_decided_by.user_id = user_record_decided_by.id
            WHERE 
                assigned_staff.task_id = $1 
                AND assigned_staff.staff_id = $2
                AND assigned_staff.deleted_at IS NULL;"#,
            task_id,
            staff_id
        )
        .fetch_one(executor)
        .await?;

        Ok(assigned_staff.into())
    }

    async fn read_one_tx(&self, task_id: Uuid, staff_id: Uuid, mut tx: Transaction<'_, Postgres>) -> DbResult<AssignedStaffExtended> {
        let assigned_staff: AssignedStaffStaffUserCompanyFlattened = sqlx::query_as!(
            AssignedStaffStaffUserCompanyFlattened,
            r#"
            SELECT 
                assigned_staff.task_id AS assigned_staff_task_id,
                assigned_staff.staff_id AS assigned_staff_id, 
                assigned_staff.status AS "assigned_staff_status!: AcceptanceStatus", 
                assigned_staff.decided_by AS assigned_staff_decided_by, 
                assigned_staff.created_at AS assigned_staff_created_at,
                assigned_staff.edited_at AS assigned_staff_edited_at, 
                assigned_staff.deleted_at AS assigned_staff_deleted_at,
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
                company.deleted_at AS company_deleted_at,
                user_record_decided_by.id AS "decided_by_user_id?", 
                user_record_decided_by.name AS "decided_by_user_name?",
                user_record_decided_by.email AS "decided_by_user_email?", 
                user_record_decided_by.birth AS "decided_by_user_birth?", 
                user_record_decided_by.avatar_url AS "decided_by_user_avatar_url", 
                user_record_decided_by.gender AS "decided_by_user_gender?: Gender", 
                user_record_decided_by.role AS "decided_by_user_role?: UserRole", 
                user_record_decided_by.status AS "decided_by_user_status?: UserStatus", 
                user_record_decided_by.created_at AS "decided_by_user_created_at?",
                user_record_decided_by.edited_at AS "decided_by_user_edited_at?", 
                user_record_decided_by.deleted_at AS "decided_by_user_deleted_at?"
            FROM 
                assigned_staff 
                INNER JOIN event_staff ON assigned_staff.staff_id = event_staff.id
                INNER JOIN user_record ON event_staff.user_id = user_record.id
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN event_staff AS event_staff_decided_by ON assigned_staff.decided_by = event_staff_decided_by.id
                LEFT OUTER JOIN user_record AS user_record_decided_by ON event_staff_decided_by.user_id = user_record_decided_by.id
            WHERE 
                assigned_staff.task_id = $1 
                AND assigned_staff.staff_id = $2
                AND assigned_staff.deleted_at IS NULL;"#,
            task_id,
            staff_id
        )
        .fetch_one(tx.deref_mut())
        .await?;

        tx.commit().await?;

        Ok(assigned_staff.into())
    }

    pub async fn read_all_per_task(
        &self,
        task_id: Uuid,
        filter: AssignedStaffFilter,
    ) -> DbResult<Vec<AssignedStaffExtended>> {
        let executor = self.pool.as_ref();

        //TODO Note to self, make sure that removing the ? from the end of decided_by:avatar_url did not break anything.
        let assigned_staff_to_task: Vec<AssignedStaffStaffUserCompanyFlattened> = sqlx::query_as!(
            AssignedStaffStaffUserCompanyFlattened,
            r#"
            SELECT 
                assigned_staff.task_id AS assigned_staff_task_id,
                assigned_staff.staff_id AS assigned_staff_id, 
                assigned_staff.status AS "assigned_staff_status!: AcceptanceStatus", 
                assigned_staff.decided_by AS assigned_staff_decided_by, 
                assigned_staff.created_at AS assigned_staff_created_at,
                assigned_staff.edited_at AS assigned_staff_edited_at, 
                assigned_staff.deleted_at AS assigned_staff_deleted_at,
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
                company.deleted_at AS company_deleted_at,
                user_record_decided_by.id AS "decided_by_user_id?", 
                user_record_decided_by.name AS "decided_by_user_name?",
                user_record_decided_by.email AS "decided_by_user_email?", 
                user_record_decided_by.birth AS "decided_by_user_birth?", 
                user_record_decided_by.avatar_url AS "decided_by_user_avatar_url", 
                user_record_decided_by.gender AS "decided_by_user_gender?: Gender", 
                user_record_decided_by.role AS "decided_by_user_role?: UserRole", 
                user_record_decided_by.status AS "decided_by_user_status?: UserStatus", 
                user_record_decided_by.created_at AS "decided_by_user_created_at?",
                user_record_decided_by.edited_at AS "decided_by_user_edited_at?", 
                user_record_decided_by.deleted_at AS "decided_by_user_deleted_at?"
            FROM 
                assigned_staff 
                INNER JOIN event_staff ON assigned_staff.staff_id = event_staff.id
                INNER JOIN user_record ON event_staff.user_id = user_record.id
                INNER JOIN company ON event_staff.company_id = company.id
                LEFT OUTER JOIN event_staff AS event_staff_decided_by ON assigned_staff.decided_by = event_staff_decided_by.id
                LEFT OUTER JOIN user_record AS user_record_decided_by ON event_staff_decided_by.user_id = user_record_decided_by.id
            WHERE 
                assigned_staff.task_id = $1
                AND assigned_staff.deleted_at IS NULL
            LIMIT $2 OFFSET $3"#,
            task_id,
            filter.limit,
            filter.offset
        )
        .fetch_all(executor)
        .await?;

        Ok(assigned_staff_to_task
            .into_iter()
            .map(|x| x.into())
            .collect())
    }

    pub async fn update(
        &self,
        task_id: Uuid,
        staff_id: Uuid,
        data: AssignedStaffData,
    ) -> DbResult<AssignedStaffExtended> {
        let mut tx = self.pool.begin().await?;

        let _ = sqlx::query_as!(
            AssignedStaff,
            r#"
            UPDATE assigned_staff SET
                status = $1,
                decided_by = $2,
                edited_at = now()
            WHERE
                staff_id = $3 
                AND task_id = $4
                AND deleted_at IS NULL
            RETURNING task_id, 
                staff_id, 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.status as AcceptanceStatus,
            data.decided_by,
            staff_id,
            task_id,
        )
        .fetch_one(tx.deref_mut())
        .await?;

        let updated_staff = self.read_one(task_id, staff_id).await?;

        Ok(updated_staff)
    }

    pub async fn delete(&self, task_id: Uuid, staff_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _ = sqlx::query_as!(
            AssignedStaff,
            r#"
            UPDATE assigned_staff SET
                deleted_at = now(),
                edited_at = now()
            WHERE
                staff_id = $1 
                AND task_id = $2
                AND deleted_at IS NULL
            RETURNING task_id, 
                staff_id, 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            staff_id,
            task_id
        )
        .fetch_one(executor)
        .await?;

        Ok(())
    }

    pub async fn delete_rejected(&self, task_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        sqlx::query!(
            r#"
            UPDATE assigned_staff SET
                deleted_at = now(),
                edited_at = now()
            WHERE
                task_id = $1
                AND status = 'rejected'
                AND deleted_at IS NULL;
            "#,
            task_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
