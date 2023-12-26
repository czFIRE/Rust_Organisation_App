use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{
    models::AcceptanceStatus,
    repositories::{
        self,
        event_staff::models::StaffExtended,
        user::{self, user_repo},
    },
};

#[derive(Debug)]
pub struct NewAssignedStaff {
    pub task_id: Uuid,
    pub staff_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct AssignedStaff {
    pub task_id: Uuid,
    pub staff_id: Uuid,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct AssignedStaffExtended {
    pub task_id: Uuid,
    pub staff: StaffExtended,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct AssignedStaffData {
    pub status: AcceptanceStatus,
    pub decided_by: Uuid,
}

#[derive(Debug)]
pub struct AssignedStaffFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl From<AssignedStaffExtended> for AssignedStaff {
    fn from(assigned_staff: AssignedStaffExtended) -> Self {
        Self {
            task_id: assigned_staff.task_id,
            staff_id: assigned_staff.staff.user.id,
            status: assigned_staff.status,
            decided_by: assigned_staff.decided_by,
            created_at: assigned_staff.created_at,
            edited_at: assigned_staff.edited_at,
            deleted_at: assigned_staff.deleted_at,
        }
    }
}
