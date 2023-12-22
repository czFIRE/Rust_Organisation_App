use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::{models::AcceptanceStatus, repositories::event_staff::models::StaffExtended};

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
