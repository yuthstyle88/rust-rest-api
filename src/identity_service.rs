use actix_identity::{CookieIdentityPolicy, IdentityService};
use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize};
use std::time::Duration;
use core::time;

lazy_static! {
    pub static ref COOKIE: Cookie = Cookie::new().unwrap();
    }

#[derive(Debug, Clone, Deserialize)]
pub struct Cookie {
    pub cookie_secret: String,
    pub domain: String,
}

#[cfg(not(tarpaulin_include))]
impl Cookie {
    pub fn new() -> Result<Self, ConfigError> {
        let res = Config::new();
        match res.try_into() {
            Ok(val) => Ok(val),
            Err(e) => Err(ConfigError::Message(format!("\n\nError: {} reads configuration\n\n", e))),
        }
    }
}


#[cfg(not(tarpaulin_include))]
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &COOKIE.cookie_secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            //TODO change cookie age
            .max_age(216000)
            .domain(&COOKIE.domain)
            .secure(false),
    )
}
