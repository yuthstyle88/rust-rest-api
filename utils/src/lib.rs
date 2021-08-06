#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate serde;

pub mod apub;
pub mod claims;
pub mod rate_limit;
pub mod request;
pub mod settings;

#[cfg(test)]
mod test;
pub mod utils;
pub mod version;
pub mod encryption;
pub mod jwt;
pub mod my_serde;

use http::StatusCode;
use std::fmt;

pub type ConnectionId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

impl fmt::Display for IpAddr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[macro_export]
macro_rules! location_info {
  () => {
    format!(
      "None value at {}:{}, column {}",
      file!(),
      line!(),
      column!()
    )
  };
}

use my_serde::Serialize;

#[cfg(not(tarpaulin_include))]
pub type MyResult<V> = std::result::Result<V, MyError>;

#[derive(Debug, Serialize)]
pub struct ApiError<'a> {
  pub code: u16,
  pub message: &'a str,
}

impl<'a> ApiError<'a> {
  pub fn err(code: u16, message: &'a str) -> Self {
    ApiError {
      code,
      message,
    }
  }
}

#[derive(Debug)]
pub struct MyError {
  pub inner: anyhow::Error,
}

impl MyError {
  pub fn new(message: anyhow::Error) -> MyError {
    MyError { inner: message }
  }
}

impl<T> From<T> for MyError
  where
      T: Into<anyhow::Error>,
{
  fn from(t: T) -> Self {
    MyError { inner: t.into() }
  }
}

impl std::fmt::Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.inner.fmt(f)
  }
}

impl actix_web::ResponseError for MyError {
  fn status_code(&self) -> StatusCode {
    match self.inner.downcast_ref::<diesel::result::Error>() {
      Some(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(ApiError::err(self.status_code().as_u16(), &*self.inner.to_string()))
  }
}