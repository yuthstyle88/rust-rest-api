use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! Ok {
    ($x:expr) => {Ok(web::Json(JsonOk::ok($x)))}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOk<T>
    where T: Sized
{
    pub code: u16,
    pub data: T,
}

impl<T> JsonOk<T> {
    pub fn ok(data: T) -> JsonOk<T> {
        JsonOk {
            code: StatusCode::OK.as_u16(),
            data,
        }
    }
}