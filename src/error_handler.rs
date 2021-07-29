use std::fmt;

use actix_web::{http, HttpResponse, ResponseError};
use actix_web::dev::{ServiceResponse, HttpResponseBuilder};
use actix_web::error::{JsonPayloadError, PayloadError};
use actix_web::error::Error as ActixError;
use actix_web::http::{StatusCode, header};

use diesel::result::{DatabaseErrorKind, Error};

use serde::{Deserialize, Serialize};
use serde_json::json;

use thiserror::Error;

#[derive(Debug, Error,Serialize)]
#[error("{{\"error\":\"{message}\"}}")]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl ApiError {
    pub fn err(code: u16,msg: &str) -> Self {
        ApiError {
            code,
            message: msg.to_string(),
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

impl  ResponseError for MyError {
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
