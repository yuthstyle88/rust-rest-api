use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;
#[macro_export]
macro_rules! res {
    ($x:expr) => {web::Json(JsonOk::ok($x))}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json<K, T>
    where K: Sized,
          T: Sized
{
    pub code: K,
    pub message: String,
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


impl<K, T> Json<K, T> {
    pub fn new(code: K, message: &str, data: T) -> Json<K, T> {
        Json {
            code,
            message: message.to_string(),
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