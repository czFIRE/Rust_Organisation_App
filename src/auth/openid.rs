use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{Error, Value};

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

pub fn get_introspect(path: &str, payload: serde_json::Value) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let data = payload.get("token").unwrap().as_str().unwrap();
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(data).unwrap());

    let k_res = client.post(path).headers(headers).form(&payload).send();

    // let data = k_res?;

    // data
    todo!()
}
pub async fn set_renew(
    path: &str,
    access: &str,
    payload: serde_json::Value,
) -> Result<Token, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(access).unwrap());

    let k_res = client
        .post(path)
        .headers(headers)
        .form(&payload)
        .send()
        .await?
        .error_for_status()?;
    let res_text = k_res.text().await?;
    let token_res: Result<Token, Error> = serde_json::from_str(&res_text);
    Ok(token_res.expect("Should be valid"))
}

pub async fn set_logout(path: &str, access: &str, payload: serde_json::Value) {
    let client = reqwest::Client::new();

    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(access).unwrap());

    client
        .post(path)
        .headers(headers)
        .form(&payload)
        .send()
        .await
        .expect("Should be valid here.");
}
