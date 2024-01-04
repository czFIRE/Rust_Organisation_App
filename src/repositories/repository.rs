use std::sync::Arc;

use async_trait::async_trait;

/// Database repository trait - implements a constructor, optionally implements any of the traits
/// that are defined in this file.
#[async_trait]
pub trait DbRepository {
    /// Database repository constructor
    #[must_use]
    fn new(pool: Arc<sqlx::PgPool>) -> Self;

    /// Method allowing the database repository to disconnect from the database pool gracefully
    async fn disconnect(&mut self) -> ();
}
