use actix_web::http::StatusCode;
use note_utils::my_serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! Ok {
    ($x:expr) => {Ok(web::Json(JsonOk::ok($x)))}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "note_utils::my_serde")]
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