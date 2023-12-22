use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{Event, EventData, EventFilter, NewEvent};

#[derive(Clone)]
pub struct EventRepository {
    pub pool: Arc<PgPool>,
}

impl EventRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewEvent) -> DbResult<Event> {
        todo!()
    }

    pub async fn _read_one(&self, _uuid: Uuid) -> DbResult<Event> {
        // Redis here
        self._read_one_db(_uuid).await
    }

    async fn _read_one_db(&self, _uuid: Uuid) -> DbResult<Event> {
        todo!()
    }

    pub async fn _read_all(&self, _filter: EventFilter) -> DbResult<Vec<Event>> {
        todo!()
    }

    pub async fn _update_event(&self, _uuid: Uuid, _data: EventData) -> DbResult<Event> {
        todo!()
    }

    pub async fn _delete_event(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
