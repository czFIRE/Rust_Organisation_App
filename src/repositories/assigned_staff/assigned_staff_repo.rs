use crate::{
    common::DbResult, repositories::assigned_staff::models::AssignedStaffStaffUserCompanyFlattened,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{AcceptanceStatus, Gender, StaffLevel, UserRole, UserStatus};

use super::models::{
    AssignedStaff, AssignedStaffData, AssignedStaffExtended, AssignedStaffFilter, NewAssignedStaff,
};

#[derive(Clone)]
pub struct AssignedStaffRepository {
    pub pool: Arc<PgPool>,
}

impl AssignedStaffRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // was AssignedStaffExtended
    pub async fn _create(&self, data: NewAssignedStaff) -> DbResult<AssignedStaff> {
        let executor = self.pool.as_ref();

        let assigned_staff = sqlx::query_as!(
            AssignedStaff,
            r#"
            INSERT INTO assigned_staff (
                task_id, staff_id, status, decided_by, 
                created_at, edited_at, deleted_at
            ) 
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7) 
            RETURNING task_id, 
                staff_id, 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.task_id,
            data.staff_id,
            AcceptanceStatus::Pending as AcceptanceStatus,
            None::<Uuid>,
            chrono::Utc::now().naive_utc(),
            chrono::Utc::now().naive_utc(),
            None::<chrono::NaiveDateTime>,
        )
        .fetch_one(executor)
        .await?;

        // TODO - are we returning the right thing here?
        Ok(assigned_staff)
    }

    pub async fn _read_one(
        &self,
        task_id: Uuid,
        staff_id: Uuid,
    ) -> DbResult<AssignedStaffExtended> {
        // Redis here
        self.read_one_db(task_id, staff_id).await
    }

    async fn read_one_db(&self, task_id: Uuid, staff_id: Uuid) -> DbResult<AssignedStaffExtended> {
        // TODO - use transaction here
        let executor = self.pool.as_ref();

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
                event_staff.role AS "staff_role!: StaffLevel", 
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
                company.deleted_at AS company_deleted_at
            FROM 
                assigned_staff 
                INNER JOIN event_staff ON assigned_staff.staff_id = event_staff.id
                INNER JOIN user_record ON event_staff.user_id = user_record.id
                INNER JOIN company ON event_staff.company_id = company.id
            WHERE 
                assigned_staff.task_id = $1 AND assigned_staff.staff_id = $2;"#,
            task_id,
            staff_id
        )
        .fetch_one(executor)
        .await?;

        Ok(assigned_staff.into())
    }

    pub async fn _read_all_per_task(
        &self,
        task_id: Uuid,
        filter: AssignedStaffFilter,
    ) -> DbResult<Vec<AssignedStaffExtended>> {
        let executor = self.pool.as_ref();

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
                event_staff.role AS "staff_role!: StaffLevel", 
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
                company.deleted_at AS company_deleted_at
            FROM 
                assigned_staff 
                INNER JOIN event_staff ON assigned_staff.staff_id = event_staff.id
                INNER JOIN user_record ON event_staff.user_id = user_record.id
                INNER JOIN company ON event_staff.company_id = company.id
            WHERE 
                assigned_staff.task_id = $1
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

    pub async fn _update(
        &self,
        staff_id: Uuid,
        task_id: Uuid,
        data: AssignedStaffData,
    ) -> DbResult<AssignedStaff> {
        let executor = self.pool.as_ref();

        // both have to bet set for this to work
        if data.status.is_none() || data.decided_by.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let assigned_staff = sqlx::query_as!(
            AssignedStaff,
            r#"
            UPDATE assigned_staff SET
                status = $1,
                decided_by = $2,
                edited_at = now()
            WHERE
                staff_id = $3 AND task_id = $4
            RETURNING task_id, 
                staff_id, 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.status as Option<AcceptanceStatus>,
            data.decided_by,
            staff_id,
            task_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(assigned_staff)
    }

    pub async fn _delete(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
