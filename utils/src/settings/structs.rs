use merge::Merge;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize, Clone, Merge)]
pub struct Settings {
  pub(crate) rate_limit: Option<RateLimitConfig>,
  pub(crate) federation: Option<FederationConfig>,
  pub(crate) hostname: Option<String>,
  pub(crate) bind: Option<IpAddr>,
  pub(crate) port: Option<u16>,
  pub(crate) tls_enabled: Option<bool>,
  pub(crate) jwt_secret: Option<String>,
  pub(crate) captcha: Option<CaptchaConfig>,
  pub(crate) setup: Option<SetupConfig>,
  pub(crate) additional_slurs: Option<String>,
  pub(crate) actor_name_max_length: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CaptchaConfig {
  pub enabled: bool,
  pub difficulty: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FederationConfig {
  pub enabled: bool,
  pub allowed_instances: Option<Vec<String>>,
  pub blocked_instances: Option<Vec<String>>,
  pub strict_allowlist: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
  pub register: i32,
  pub register_per_second: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SetupConfig {
  pub admin_username: String,
  pub admin_password: String,
  pub admin_email: Option<String>,
  pub site_name: String,
}
