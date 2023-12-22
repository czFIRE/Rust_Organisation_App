use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    models::{AcceptanceStatus, StaffLevel},
    repositories::{company::models::Company, user::models::User},
};

#[derive(Debug)]
pub struct NewStaff {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub role: StaffLevel,
}

#[derive(Debug, FromRow)]
pub struct Staff {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct StaffExtended {
    pub user: User,
    pub company: Company,
    pub event_id: Uuid,
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct StaffData {
    pub role: StaffLevel,
    pub status: AcceptanceStatus,
    pub decided_by: Option<Uuid>,
}

#[derive(Debug)]
pub struct StaffFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
