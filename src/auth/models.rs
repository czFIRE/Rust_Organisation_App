pub struct AuthGuard;

use chrono::NaiveDate;
use serde::Serialize;
use serde::Deserialize;

use crate::models::Gender;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u16,
    pub refresh_expires_in: u16,
    pub refresh_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenApplication {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}