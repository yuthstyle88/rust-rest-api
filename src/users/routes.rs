use actix_web::{delete, get, post, put, Responder, web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{Ok, utils::response::JsonOk};
use crate::users::{User};
use note_utils::MyResult;
use crate::app::NoteContext;
use actix_web::web::Data;
use crate::db::blocking;
use crate::utils::token_utils::get_uid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lastname: Option<String>,
}

#[get("/")]
pub fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}
//
// #[get("/{id}")]
// async fn users_find(context: Data<NoteContext>, id: web::Path<i32>,) -> MyResult<impl Responder> {
//
//     let id=  id.into_inner();
//     let user = blocking(context.pool(), move |conn| {
//         User::find(conn, id)
//     }).await??;
//     Ok!(user)
// }


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .service(index)
    );
}
