use anyhow::Error;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct QueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn extract_path_tuple_ids(ids: (String, String)) -> Result<(Uuid, Uuid), Error> {
    Ok((
        Uuid::from_str(ids.0.as_str())?,
        Uuid::from_str(ids.1.as_str())?,
    ))
}
