use crate::settings::{CaptchaConfig,  FederationConfig, RateLimitConfig, Settings};
use std::net::{IpAddr, Ipv4Addr};

impl Default for Settings {
  fn default() -> Self {
    Self {
      rate_limit: Some(RateLimitConfig::default()),
      federation: Some(FederationConfig::default()),
      captcha: Some(CaptchaConfig::default()),
      setup: None,
      hostname: None,
      bind: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
      port: Some(8536),
      tls_enabled: Some(true),
      jwt_secret: Some("changeme".into()),
      additional_slurs: None,
      actor_name_max_length: Some(20),
    }
  }
}

impl Default for CaptchaConfig {
  fn default() -> Self {
    Self {
      enabled: true,
      difficulty: "medium".into(),
    }
  }
}

impl Default for FederationConfig {
  fn default() -> Self {
    Self {
      enabled: false,
      allowed_instances: None,
      blocked_instances: None,
      strict_allowlist: Some(true),
    }
  }
}

impl Default for RateLimitConfig {
  fn default() -> Self {
    Self {
      register: 3,
      register_per_second: 3600,

    }
  }
}
