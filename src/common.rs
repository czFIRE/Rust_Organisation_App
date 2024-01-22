type DatabaseError = sqlx::Error;
pub type DbResult<T> = Result<T, DatabaseError>;

pub const PAGINATION_LIMIT: i64 = 5;

// Retrieve new offsets for pagination.
pub fn calculate_new_offsets(current_offset: Option<i64>) -> (Option<i64>, Option<i64>) {
    if current_offset.is_none() {
        println!("NONE");
        return (None, None);
    }
    println!("SOME");
    let offset = current_offset.expect("Should be some.");
    let new_prev = if offset - PAGINATION_LIMIT < 0 {
        None
    } else {
        Some(offset - PAGINATION_LIMIT)
    };
    let new_next = Some(offset + PAGINATION_LIMIT);
    (new_prev, new_next)
}
