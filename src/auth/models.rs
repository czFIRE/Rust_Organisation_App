pub struct AuthGuard;

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use crate::models::Gender;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: i64,
    pub session_state: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenApplication {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub birth: NaiveDate,
    pub gender: Gender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAccess {
    account: RealmAccess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullClaims {
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub typ: String,
    pub azp: String,
    pub session_state: String,
    pub acr: String,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Vec<String>,
    pub realm_access: RealmAccess,
    pub resource_access: ResourceAccess,
    pub scope: String,
    pub sid: String,
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}
