use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    NewStaff, Staff, StaffData, StaffExtended, StaffFilter, StaffUserCompanyFlattened,
};

use crate::models::{AcceptanceStatus, Gender, StaffLevel, UserRole, UserStatus};

#[derive(Clone)]
pub struct StaffRepository {
    pub pool: Arc<PgPool>,
}

impl StaffRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, data: NewStaff) -> DbResult<Staff> {
        let executor = self.pool.as_ref();

        let new_staff: Staff = sqlx::query_as!(
            Staff,
            r#" INSERT INTO event_staff 
                ( user_id, company_id, event_id, role ) 
            VALUES 
                ($1, $2, $3, $4) 
            RETURNING id, 
                user_id, 
                company_id, 
                event_id, 
                role AS "role!: StaffLevel", 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.user_id,
            data.company_id,
            data.event_id,
            data.role as StaffLevel,
        )
        .fetch_one(executor)
        .await?;

        Ok(new_staff)
    }

    pub async fn _read_one(&self, event_staff_id: Uuid) -> DbResult<StaffExtended> {
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
                event_staff 
                INNER JOIN user_record on event_staff.user_id = user_record.id 
                INNER JOIN company on event_staff.company_id = company.id 
            WHERE 
                event_staff.id = $1;
            "#,
            event_staff_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(staff.into())
    }

    pub async fn _read_all(
        &self,
        event_uuid: Uuid,
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
                event_staff 
                INNER JOIN user_record on event_staff.user_id = user_record.id 
                INNER JOIN company on event_staff.company_id = company.id 
            WHERE 
                event_staff.event_id = $1
            LIMIT $2 OFFSET $3;
            "#,
            event_uuid,
            filter.limit,
            filter.offset,
        )
        .fetch_all(executor)
        .await?;

        Ok(staff.into_iter().map(|s| s.into()).collect())
    }

    pub async fn _update(&self, event_staff_id: Uuid, data: StaffData) -> DbResult<Staff> {
        if data.role.is_none() && data.status.is_none() && data.decided_by.is_none() {
            // TODO - better error
            return Err(sqlx::Error::RowNotFound);
        }

        let executor = self.pool.as_ref();

        let updated_staff: Staff = sqlx::query_as!(
            Staff,
            r#" UPDATE event_staff SET 
                role = COALESCE($1, role), 
                status = COALESCE($2, status), 
                decided_by = COALESCE($3, decided_by), 
                edited_at = now() 
            WHERE id = $4 
            RETURNING id, 
                user_id, 
                company_id, 
                event_id, 
                role AS "role!: StaffLevel", 
                status AS "status!: AcceptanceStatus", 
                decided_by, 
                created_at, 
                edited_at, 
                deleted_at;
            "#,
            data.role as Option<StaffLevel>,
            data.status as Option<AcceptanceStatus>,
            data.decided_by,
            event_staff_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated_staff)
    }

    pub async fn _delete(&self, event_staff_id: Uuid) -> DbResult<()> {
        let executor = self.pool.as_ref();

        let _deleted_staff = sqlx::query!(
            r#" UPDATE event_staff SET 
                deleted_at = now(), 
                edited_at = now() 
            WHERE id = $1 
            RETURNING id;
            "#,
            event_staff_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(())
    }
}
