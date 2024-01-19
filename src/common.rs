#![allow(dead_code)]

type DatabaseError = sqlx::Error;
pub type DbResult<T> = Result<T, DatabaseError>;

// a delta for float comparisons
pub const DELTA: f32 = 0.0000001;
