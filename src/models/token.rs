use std::str;

use chrono::Local;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::constants::KEY;

// static ONE_HOUR: i64 = 60 * 60; // in seconds
static ONE_DAY: i64 = 60 * 60 * 24;
// in seconds
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
// in seconds
static ONE_YEAR: i64 = 60 * 60 * 24 * 365; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
    pub auth_type: String,
}

impl AuthToken {
    pub fn generate_token(username: String, login_session: String, auth_type: String) -> String {
        let now = Local::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = AuthToken {
            iat: now,
            exp: now + ONE_YEAR + ONE_WEEK + ONE_DAY,
            user: username,
            login_session,
            auth_type,
        };

        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthTokenMessage {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMessageInfo {
    pub username: String,
    pub password: String,
}
