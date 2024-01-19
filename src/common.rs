type DatabaseError = sqlx::Error;
pub type DbResult<T> = Result<T, DatabaseError>;
