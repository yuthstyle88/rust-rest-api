use crate::{
  location_info,
  settings::structs::{
    CaptchaConfig,
    FederationConfig,
    RateLimitConfig,
    Settings,
    SetupConfig,
  },
  MyError,
};
use anyhow::{Context};
use std::{net::IpAddr, sync::RwLock};
use deser_hjson::from_str;

pub mod defaults;
pub mod structs;

// static CONFIG_FILE: &str = "config/config.hjson";

lazy_static! {
  static ref SETTINGS: RwLock<Settings> =
    RwLock::new(Settings::init().expect("Failed to load settings file"));
}


impl Settings {
  fn init() -> Result<Self, MyError> {
    // Read the config file
    let custom_config= Default::default();
    //let mut custom_config = from_str::<Settings>(&Self::read_config_file()?)?;

    Ok(custom_config)
  }

  pub fn get_allowed_instances(&self) -> Option<Vec<String>> {
    self.federation().allowed_instances
  }

  pub fn get_blocked_instances(&self) -> Option<Vec<String>> {
    self.federation().blocked_instances
  }

  pub fn get() -> Self {
    SETTINGS.read().expect("read config").to_owned()
  }

  pub fn get_hostname_without_port(&self) -> Result<String, anyhow::Error> {
    Ok(
      self
        .hostname()
        .split(':')
        .collect::<Vec<&str>>()
        .first()
        .context(location_info!())?
        .to_string(),
    )
  }

  pub fn hostname(&self) -> String {
    self.hostname.to_owned().expect("No hostname given")
  }
  pub fn bind(&self) -> IpAddr {
    self.bind.expect("return bind address")
  }
  pub fn port(&self) -> u16 {
    self
      .port
      .unwrap_or_else(|| Settings::default().port.expect("missing port"))
  }
  pub fn tls_enabled(&self) -> bool {
    self.tls_enabled.unwrap_or_else(|| {
      Settings::default()
        .tls_enabled
        .expect("missing tls_enabled")
    })
  }
  pub fn jwt_secret(&self) -> String {
    self
      .jwt_secret
      .to_owned()
      .unwrap_or_else(|| Settings::default().jwt_secret.expect("missing jwt_secret"))
  }

  pub fn actor_name_max_length(&self) -> usize {
    self.actor_name_max_length.unwrap_or_else(|| {
      Settings::default()
        .actor_name_max_length
        .expect("missing actor name length")
    })
  }

  pub fn rate_limit(&self) -> RateLimitConfig {
    self.rate_limit.to_owned().unwrap_or_default()
  }
  pub fn federation(&self) -> FederationConfig {
    self.federation.to_owned().unwrap_or_default()
  }
  pub fn captcha(&self) -> CaptchaConfig {
    self.captcha.to_owned().unwrap_or_default()
  }
  pub fn setup(&self) -> Option<SetupConfig> {
    self.setup.to_owned()
  }
}
