use crate::{
    common::DbResult,
    repositories::{
        company::models::Company,
        event::models::Event,
        event_staff::models::{Staff, StaffExtended},
        task::models::Task,
        user::models::User,
    },
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{AcceptanceStatus, Gender, StaffLevel, TaskPriority, UserRole, UserStatus};

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
    pub async fn create(&self, data: NewAssignedStaff) -> DbResult<AssignedStaff> {
        let executor = self.pool.as_ref();

        let assigned_staff = sqlx::query_as!(
            AssignedStaff,
            r#"INSERT INTO assigned_staff (task_id, staff_id, status, decided_by, created_at, edited_at, deleted_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7) 
               RETURNING task_id, staff_id, status AS "status!: AcceptanceStatus", decided_by, created_at, edited_at, deleted_at;"#,            
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
        task_uuid: Uuid,
        staff_uuid: Uuid,
    ) -> DbResult<AssignedStaffExtended> {
        // Redis here
        self._read_one_db(task_uuid, staff_uuid).await
    }

    async fn _read_one_db(
        &self,
        _task_uuid: Uuid,
        _staff_uuid: Uuid,
    ) -> DbResult<AssignedStaffExtended> {
        // TODO - use transaction here
        let executor = self.pool.as_ref();

        let assigned_staff: AssignedStaff = sqlx::query_as!(
            AssignedStaff,
            r#"SELECT task_id, staff_id, status AS "status!: AcceptanceStatus", decided_by,
            created_at, edited_at, deleted_at
            FROM assigned_staff
            WHERE task_id = $1 AND staff_id = $2;"#,
            _task_uuid,
            _staff_uuid
        )
        .fetch_one(executor)
        .await?;

        // we only care about id
        let task: Task = sqlx::query_as!(
            Task,
            r#"SELECT id, event_id, creator_id, title, description,
            finished_at, priority as "priority!: TaskPriority", accepts_staff, created_at, deleted_at, edited_at
            FROM task WHERE id = $1;"#
            , _task_uuid
        )
        .fetch_one(executor)
        .await?;

        let event_staff: Staff = sqlx::query_as!(
            Staff,
            r#"SELECT id, user_id, company_id, event_id, role AS "role!: StaffLevel", status AS "status!: AcceptanceStatus", decided_by, created_at, edited_at, deleted_at FROM event_staff WHERE id = $1 AND event_id = $2;"#,
            _staff_uuid,
            task.event_id
        )
        .fetch_one(executor)
        .await?;

        let user: User = sqlx::query_as!(
            User,
            r#"SELECT id, name, email, birth, avatar_url, gender AS "gender!: Gender", role AS "role!: UserRole", status AS "status!: UserStatus", created_at, edited_at, deleted_at FROM user_record WHERE id = $1;"#,
            _staff_uuid
        )
        .fetch_one(executor)
        .await?;

        let company: Company = sqlx::query_as!(
            Company,
            r#"SELECT * FROM company WHERE id = $1;"#,
            event_staff.company_id
        )
        .fetch_one(executor)
        .await?;

        let staff_extended = StaffExtended {
            user,
            company,
            event_id: event_staff.event_id,
            role: event_staff.role,
            status: event_staff.status,
            decided_by: event_staff.decided_by,
            created_at: event_staff.created_at,
            edited_at: event_staff.edited_at,
            deleted_at: event_staff.deleted_at,
        };

        let assigned_staff_extended = AssignedStaffExtended {
            task_id: assigned_staff.task_id,
            staff: staff_extended,
            status: assigned_staff.status,
            decided_by: assigned_staff.decided_by,
            created_at: assigned_staff.created_at,
            edited_at: assigned_staff.edited_at,
            deleted_at: assigned_staff.deleted_at,
        };

        Ok(assigned_staff_extended)
    }

    pub async fn _read_all_per_task(
        &self,
        _task_uuid: Uuid,
        _filter: AssignedStaffFilter,
    ) -> DbResult<Vec<AssignedStaffExtended>> {
        todo!()
    }

    pub async fn _update(
        &self,
        _uuid: Uuid,
        _data: AssignedStaffData,
    ) -> DbResult<AssignedStaffExtended> {
        todo!()
    }

    pub async fn _delete(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
