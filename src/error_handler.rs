use actix_web::{HttpResponse,ResponseError};
use actix_web::http::StatusCode;
use serde::Serialize;

#[cfg(not(tarpaulin_include))]
pub type MyResult<V> = std::result::Result<V, MyError>;

#[derive(Debug, Serialize)]
pub struct ApiError<'a> {
    pub code: u16,
    pub message: &'a str,
}

impl<'a> ApiError<'a> {
    pub fn err(code: u16, msg: &'a str) -> Self {
        ApiError {
            code,
            message: msg,
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

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self.inner.downcast_ref::<diesel::result::Error>() {
            Some(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(ApiError::err(self.status_code().as_u16(), &*self.inner.to_string()))
    }
}