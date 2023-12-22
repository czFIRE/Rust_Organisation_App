use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{NewTask, Task, TaskData, TaskExtended, TaskFilter};

#[derive(Clone)]
pub struct TaskRepository {
    pub pool: Arc<PgPool>,
}

impl TaskRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewTask) -> DbResult<TaskExtended> {
        todo!()
    }

    pub async fn _read_one(&self, _uuid: Uuid) -> DbResult<TaskExtended> {
        // Redis here
        self._read_one_db(_uuid).await
    }

    async fn _read_one_db(&self, _uuid: Uuid) -> DbResult<TaskExtended> {
        todo!()
    }

    pub async fn _read_all(&self, _filter: TaskFilter) -> DbResult<Vec<Task>> {
        todo!()
    }

    pub async fn _update_event(&self, _uuid: Uuid, _data: TaskData) -> DbResult<TaskExtended> {
        todo!()
    }

    pub async fn _delete_event(&self, _uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
