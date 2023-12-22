use crate::common::DbResult;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{
    Employment, EmploymentData, EmploymentExtended, EmploymentFilter, NewEmployment,
};

#[derive(Clone)]
pub struct EmploymentRepository {
    pub pool: Arc<PgPool>,
}

impl EmploymentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn _create(&self, _data: NewEmployment) -> DbResult<Employment> {
        todo!()
    }

    pub async fn _read_one(
        &self,
        _user_uuid: Uuid,
        _company_uuid: Uuid,
    ) -> DbResult<EmploymentExtended> {
        // Implement redis here.
        self._read_one_db(_user_uuid, _company_uuid).await
    }

    // Actual DB access.
    async fn _read_one_db(
        &self,
        _user_uuid: Uuid,
        _company_uuid: Uuid,
    ) -> DbResult<EmploymentExtended> {
        todo!()
    }

    // Retrieves all employments for a given user.
    pub async fn _read_all_user(
        &self,
        _user_uuid: Uuid,
        _filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        todo!()
    }

    // Retrieves all employments for a given company.
    pub async fn _read_all_company(
        &self,
        _company_uuid: Uuid,
        _filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        todo!()
    }

    // Retrieves all subordinates for a given manager.
    pub async fn _read_subordinates(
        &self,
        _manager_uuid: Uuid,
        _filter: EmploymentFilter,
    ) -> DbResult<Vec<EmploymentExtended>> {
        todo!()
    }

    pub async fn _update_employment(
        &self,
        _user_uuid: Uuid,
        _company_uuid: Uuid,
        _data: EmploymentData,
    ) -> DbResult<EmploymentExtended> {
        todo!()
    }

    pub async fn _delete_employment(&self, _user_uuid: Uuid, _company_uuid: Uuid) -> DbResult<()> {
        todo!()
    }
}
