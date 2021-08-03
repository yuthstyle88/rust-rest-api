use actix_web::{dev::Payload, FromRequest, HttpRequest};
use anyhow::anyhow;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use note_utils::MyError;

use crate::constants;
use crate::users::User;
use diesel::PgConnection;
use actix_web::dev::RequestHead;
use serde::Deserialize;
use crate::models::AuthToken;
use actix_web::http::HeaderValue;


// pub fn get_refresh_token(req: &HttpRequest) -> Result<String, ApiError> {
//     let headers = req.headers();
//     log::info!("{:#?}", headers);
//     match headers.get(constants::X_REFRESH_TOKEN) {
//         Some(x_refresh_token) => {
//             let refresh_token: &str = x_refresh_token.to_str().unwrap();
//             if !refresh_token.is_empty() {
//                 Ok(refresh_token.to_string())
//             } else {
//                 Err(ApiError::new(ResponseCode::NoContent.as_u16(), tr!("Refresh token is missing")))
//             }
//         }
//         _ => Err(ApiError::new(ResponseCode::InternalServerError.as_u16(), tr!("Refresh token is missing")))
//     }
// }

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<AuthToken>> {
    jsonwebtoken::decode::<AuthToken>(token, &DecodingKey::from_secret(&constants::KEY), &Validation::default())
}

pub fn get_uid(req: &HttpRequest) -> Result<&str, MyError>
{
    match req.headers().get(constants::AUTHENTICATED) {
        Some(uid) => Ok(uid.to_str().unwrap()),
        _ => Err(MyError::new(anyhow!("get uid from HttpRequest error")))
    }
}

// pub async fn get_agent_token() -> String {
//     // let token_file = env::var("TOKEN_FILE").unwrap();
//     // let mut token: String = std::fs::read_to_string(token_file)
//     //     .unwrap()
//     //     .parse()
//     //     .unwrap();
//     let agent_id = env::var("AGENT_ID").expect("Backend User not set");
//     let agent = Agent::find_by_user(agent_id).unwrap();
//     let mut token = agent.token;
//     if token == "" {
//         token = agent_login().await;
//     }
//
//     token
// }

// pub async fn agent_login() -> String {
//     let agent_id = env::var("AGENT_ID").expect("Backend User not set");
//     let password = env::var("AGENT_PASSWORD").expect("Backend Password not set");
//     let backend_service = env::var("BACKEND_SERVICE").unwrap();
//     let token_file = env::var("TOKEN_FILE").unwrap();
//
//     let url = format!("{}/agent/user/login", backend_service);
//
//     let mut post_params = HashMap::new();
//     post_params.insert("username_email", agent_id.as_str());
//     post_params.insert("password", password.as_str());
//
//     let res = reqwest::Client::new().post(&url)
//         .json(&post_params)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await.unwrap();
//
//     let obj: Value = serde_json::from_str(res.as_str()).unwrap();
//     let user_info = obj.get("data").unwrap();
//     let token = user_info.get("token").unwrap();
//     std::fs::write(token_file, token.to_string().clone()).unwrap();
//
//     let _ = Agent::update_token(agent_id, AgentToken{
//         token: token.to_string(),
//         updated_at: Some(Local::now().naive_local())
//     });
//
//     dbg!("New Agent Login");
//     token.to_string()
// }


#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use chrono::{Local, NaiveDateTime};

    use super::*;

    #[test]
    fn test_decode_token() {
        let token_base64 = "ZXlKMGVYQWlPaUpLVjFRaUxDSmhiR2NpT2lKSVV6STFOaUo5LmV5SnBZWFFpT2pFMk1qTTVNRGt5TVRjc0ltVjRjQ0k2TVRZeU5EVXhOREF4Tnl3aWRYTmxjaUk2SWpFd05qUXlPRE13TWpFek9UVTNORE0yTlRRNU1DSXNJbXh2WjJsdVgzTmxjM05wYjI0aU9pSXdPVFpqTURkbVpUaGtPV00wWkdVME9HVmhNekl5WXpVd05URTBZVEl5TXlJc0ltRjFkR2hmZEhsd1pTSTZJa04xYzNSdmJXVnlJbjAubHNOZFY0Ml9mN0Rhc3NUbHpRRzZnMEhpSXBUR3ZEWjZqMW5pQUI0TkFGQQ==".to_string();
        let token_vec = base64::decode(token_base64).unwrap();
        let token = from_utf8(&token_vec).unwrap().to_string();
        println!("Token: {}", &token);
        match decode_token(token) {
            Ok(decoded) => {
                let now = Local::now().timestamp_nanos() / 1_000_000_000;
                // let at = Duration::seconds(decoded.claims.iat);
                let at = NaiveDateTime::from_timestamp_opt(decoded.claims.iat, 0).unwrap();
                let exp = NaiveDateTime::from_timestamp_opt(decoded.claims.exp, 0).unwrap();

                let diff = now - decoded.claims.exp;
                println!("IAT: {:?}", at);
                println!("Exp: {:?}", exp);
                println!("Diff: {}", diff);
                println!("{:?}", &decoded);
            }
            Err(err) => {
                println!("{:?}", &err);
            }
        }
    }
}