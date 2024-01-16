use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::{types::chrono::NaiveDateTime, FromRow};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub creator_id: Uuid,
    pub company_id: Uuid,
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
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EventData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EventFilter {
    pub accepts_staff: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
