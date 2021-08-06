use note_utils::my_serde::{Deserialize, Serialize};

//pub static DEFAULT_LANGUAGE: &str = "th-TH";

// Static Variable
pub static KEY: [u8; 32] = *include_bytes!("../.secret.key");
// pub static WEB_ID: i32 = 1;
pub static USER_PREFIX: &str = "ebu";
pub static AGENT_PREFIX: &str = "eag";
// pub static MASTER_PREFIX: &str = "emt";
// pub static SENIOR_PREFIX: &str = "esn";
// pub const DEFAULT_QUEUE: &str = "default";
// pub const APP_USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1";

// Messages
//pub const MESSAGE_OK: &str = "ok";

// Headers
// pub const AUTHORIZATION: &str = "Authorization";
pub const AUTHENTICATED: &str = "authenticated";
// pub const X_REFRESH_TOKEN: &str = "X-REF-TOKEN";
pub const X_API_KEY: &str = "x-api-key";
pub const BEARER: &str = "Bearer";
pub const CUSTOMER: &str = "Customer";
// pub const IDP: &str = "idp";
// pub const NOTE_TG: &str = "X-NoteTg-Signature";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "note_utils::my_serde")]
pub struct Empty;

// Misc
//pub const EMPTY: &str = "";
/*
// ignore routes
pub const IGNORE_ROUTES: [&str; 5] = [
    "/robots.txt",
    "/images",
    "/assets/images",
    "/static",
    "/wp-json",
];

pub const PUBLIC_RESTRICT_ROUTES: [&str; 7] = [
    "/api/v1/customer/login",
    "/api/v1/customer/mobile-login",
    "/api/v1/customer/email-login",
    "/api/v1/customer/signup",
    "/api/v1/customer/phone-sign-in",
    "/api/v1/customer/mobile-signup",
    "/api/v1/customer/request-otp",
];
*/