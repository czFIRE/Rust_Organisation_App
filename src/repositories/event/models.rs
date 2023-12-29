use chrono::NaiveDate;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

use crate::models::Association;

#[derive(Debug, Clone)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, FromRow, Clone)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub accepts_staff: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct EventData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventFilter {
    pub accepts_staff: Option<bool>,
    pub associated_company_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
