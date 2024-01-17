use crate::common::DbResult;
use sqlx::postgres::PgPool;
use sqlx::Transaction;
use std::sync::Arc;

use chrono::NaiveDate;

use std::ops::DerefMut;

use super::models::WagePreset;

use async_trait::async_trait;

// Reads a single preset from the DB using an existing transaction handler.
pub async fn read_one_db_using_tx(tx: &mut Transaction<'_, sqlx::Postgres>,
                                  preset_name: &String)
                                  -> DbResult<WagePreset> {
    let wage_preset_result: DbResult<WagePreset> = sqlx::query_as!(
        WagePreset,
        r#"
            SELECT *
            FROM wage_preset
            WHERE
                name = $1
            "#,
        preset_name,
    )
        .fetch_one(tx.deref_mut())
        .await;

    wage_preset_result
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct WagePresetRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl crate::repositories::repository::DbRepository for WagePresetRepository {
    /// Database repository constructor
    #[must_use]
    fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Method allowing the database repository to disconnect from the database pool gracefully
    async fn disconnect(&mut self) -> () {
        self.pool.close().await;
    }
}

impl WagePresetRepository {
    // Reads a single preset from the DB.
    pub async fn read_one(&self, preset_name: &String) -> DbResult<WagePreset> {
        // TODO: Redis here

        self.read_one_db(preset_name).await
    }

    async fn read_one_db(&self, preset_name: &String) -> DbResult<WagePreset> {
        let mut tx = self.pool.begin().await?;

        read_one_db_using_tx(&mut tx, preset_name).await
    }

    pub async fn read_all(&self) -> DbResult<Vec<WagePreset>> {
        // TODO: Redis here

        self.read_all_db().await
    }

    async fn read_all_db(&self) -> DbResult<Vec<WagePreset>> {
        let executor = self.pool.as_ref();

        let wage_presets: Vec<WagePreset> = sqlx::query_as!(
            WagePreset,
            r#"
            SELECT *
            FROM wage_preset
            ORDER BY valid_from;
            "#,
        )
        .fetch_all(executor)
        .await?;

        Ok(wage_presets)
    }
}