use serde::{Deserialize, Deserializer};

type DatabaseError = sqlx::Error;
pub type DbResult<T> = Result<T, DatabaseError>;

/*
 * Used to deserialize string into f32 because
 */
pub fn de_opt_from_string<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    match value {
        Some(value) => {
            //ToDo: Add better error handling. But this should be okay.
            let float = value.parse::<f32>().unwrap();
            Ok(Some(float))
        }
        None => Ok(None),
    }
}
