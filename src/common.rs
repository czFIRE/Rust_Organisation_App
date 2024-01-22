type DatabaseError = sqlx::Error;
pub type DbResult<T> = Result<T, DatabaseError>;

// a delta for float comparisons
#[allow(dead_code)]
pub const DELTA: f32 = 0.0000001;
