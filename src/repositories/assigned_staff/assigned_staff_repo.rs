use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    AssignedStaffData, AssignedStaffExtended, AssignedStaffFilter, NewAssignedStaff,
};

#[derive(Clone)]
pub struct AssignedStaffRepository {
    pub pool: Arc<PgPool>,
}

impl AssignedStaffRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewAssignedStaff) -> DbResult<AssignedStaffExtended> {
        todo!()
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
        todo!()
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
