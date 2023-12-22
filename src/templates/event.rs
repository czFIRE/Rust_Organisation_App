use askama::Template;
use sqlx::types::uuid;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Template)]
#[template(path = "event/event.html")]
pub struct EventTemplate {
    pub id: Uuid,
    pub avatar_url: String,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub accepts_staff: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime
}

#[derive(Template)]
#[template(path = "event/event-lite.html")]
pub struct EventLiteTemplate {
    pub id: Uuid,
    pub avatar_url: String,
    pub name: String,
    pub accepts_staff: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate
}