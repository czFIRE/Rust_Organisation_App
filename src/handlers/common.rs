use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    limit: Option<i64>,
    offset: Option<i64>,
}
