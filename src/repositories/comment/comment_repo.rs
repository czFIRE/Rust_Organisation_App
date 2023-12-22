use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{CommentData, CommentExtended, CommentFilter, NewComment};

#[derive(Clone)]
pub struct CommentRepository {
    pub pool: Arc<PgPool>,
}

impl CommentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewComment) -> DbResult<CommentExtended> {
        todo!()
    }

    pub async fn _read_one(&self, _uuid: Uuid) -> DbResult<CommentExtended> {
        // Redis here
        self._read_one_db(_uuid).await
    }

    async fn _read_one_db(&self, _uuid: Uuid) -> DbResult<CommentExtended> {
        todo!()
    }

    pub async fn _read_all_per_event(
        &self,
        _event_id: Uuid,
        _filter: CommentFilter,
    ) -> DbResult<Vec<CommentExtended>> {
        todo!()
    }

    pub async fn _read_all_per_task(
        &self,
        _task_id: Uuid,
        _filter: CommentFilter,
    ) -> DbResult<Vec<CommentExtended>> {
        todo!()
    }

    pub async fn _update(&self, _uuid: Uuid, _data: CommentData) -> DbResult<CommentExtended> {
        todo!()
    }

    pub async fn _delete(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
