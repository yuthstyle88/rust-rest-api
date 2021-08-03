use actix_web::{delete, get, post, put, Responder, web, HttpRequest};
use uuid::Uuid;

use crate::{Ok, utils::response::JsonOk};
use crate::users::{User, UserRequest, UserRequestUpdate};
use note_utils::MyResult;
use crate::app::NoteContext;
use actix_web::web::Data;
use crate::db::blocking;
use crate::utils::token_utils::get_uid;

#[get("/list")]
async fn users_find_all(context: Data<NoteContext>) -> MyResult<impl Responder> {
    let users = blocking(context.pool(), move |conn| {
        User::find_all(conn)
    }).await??;
    Ok!(users)
}

#[get("/{id}")]
async fn users_find(context: Data<NoteContext>, id: web::Path<String>,req: HttpRequest) -> MyResult<impl Responder> {
    let uid = get_uid(&req)?;
    let uid= uid.to_owned();
    let user = blocking(context.pool(), move |conn| {
        User::find_by_id(conn, &uid)
    }).await??;
    Ok!(user)
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .service(users_find_all)
            .service(users_find)
    );
}
