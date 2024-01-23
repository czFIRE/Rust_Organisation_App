use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::Error;

use super::models::Token;

pub async fn get_token(path: &str, payload: serde_json::Value) -> Result<Token, reqwest::Error> {
    let client = reqwest::Client::new();
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .form(&payload)
        .send()
        .await?
        .error_for_status()?;
    let res_text = k_res.text().await?;
    let token_res: Result<Token, Error> = serde_json::from_str(&res_text);
    Ok(token_res.expect("Should be valid"))
}
