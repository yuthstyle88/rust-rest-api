use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;
use crate::error_handler::MyError;

#[cfg(not(tarpaulin_include))]
pub type MyResult<V> = std::result::Result<V, MyError>;

#[macro_export]
macro_rules! Ok {
    ($x:expr) => {Ok(web::Json(JsonOk::ok($x)))}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json<'a, K, T>
    where K: Sized,
          T: Sized
{
    pub code: K,
    pub message: &'a str,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOk<'a, T>
    where T: Sized
{
    pub code: u16,
    pub message: &'a str,
    pub data: T,
}


impl<'a, K, T> Json<'a, K, T> {
    pub fn new(code: K, message: &'a str, data: T) -> Json<'a, K, T> {
        Json {
            code,
            message,
            data,
        }
    }
}

impl<'a, T> JsonOk<'a, T> {
    pub fn ok(data: T) -> JsonOk<'a, T> {
        JsonOk {
            code: StatusCode::OK.as_u16(),
            message: "ok",
            data,
        }
    }
}