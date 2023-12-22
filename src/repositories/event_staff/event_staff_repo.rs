use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{NewStaff, StaffData, StaffExtended, StaffFilter};

#[derive(Clone)]
pub struct StaffRepository {
    pub pool: Arc<PgPool>,
}

impl StaffRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewStaff) -> DbResult<StaffExtended> {
        todo!()
    }

    pub async fn _read_one(&self, _uuid: Uuid) -> DbResult<StaffExtended> {
        // Redis here
        self._read_one_db(_uuid).await
    }

    async fn _read_one_db(&self, _uuid: Uuid) -> DbResult<StaffExtended> {
        todo!()
    }

    pub async fn _read_all(
        &self,
        _event_uuid: Uuid,
        _filter: StaffFilter,
    ) -> DbResult<Vec<StaffExtended>> {
        todo!()
    }

    pub async fn _update(&self, _uuid: Uuid, _data: StaffData) -> DbResult<StaffExtended> {
        todo!()
    }

    pub async fn _delete(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
