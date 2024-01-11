use askama::Template;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::repositories::event::models::Event;

#[derive(Template, Debug, Deserialize)]
#[template(path = "event/event.html")]
pub struct EventTemplate {
    pub id: Uuid,
    pub avatar_url: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub accepts_staff: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl From<Event> for EventTemplate {
    fn from(event: Event) -> Self {
        EventTemplate {
            id: event.id,
            avatar_url: event.avatar_url,
            name: event.name,
            description: event
                .description
                .unwrap_or("No description set.".to_string()),
            website: event.website.unwrap_or("No website set.".to_string()),
            accepts_staff: event.accepts_staff,
            start_date: event.start_date,
            end_date: event.end_date,
            created_at: event.created_at,
            edited_at: event.edited_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EventLite {
    pub id: Uuid,
    pub avatar_url: String,
    pub name: String,
    pub accepts_staff: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl From<Event> for EventLite {
    fn from(event: Event) -> Self {
        EventLite {
            id: event.id,
            avatar_url: event.avatar_url,
            name: event.name,
            accepts_staff: event.accepts_staff,
            start_date: event.start_date,
            end_date: event.end_date,
        }
    }
}

#[derive(Template, Debug, Deserialize)]
#[template(path = "event/events.html")]
pub struct EventsTemplate {
    pub events: Vec<EventLite>,
}
