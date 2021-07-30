use crate::users::{User, Users, UserLogin, UserUpdate};
use crate::error_handler::{MyError, ApiError};
use actix_web::{delete, get, post, web, HttpResponse, Responder,Result};
use serde_json::json;
use anyhow::anyhow;
use actix_web::http::StatusCode;
use crate::{res,response::{Json,JsonOk}};
use crate::response::MyResult;


#[get("/users")]
async fn find_all() -> Result<impl Responder, MyError> {
    let user = Users::find_all().map(|u| u)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{id}")]
async fn find(id: web::Path<i32>) -> Result<impl Responder, MyError> {
    let user = Users::find(id.into_inner()).map(|u| u).map_err(|e| e)?;

    Ok(res!(user))
}

#[post("/post_login")]
async fn login(user: web::Json<UserLogin>) -> Result<HttpResponse, MyError> {
    let user = Users::login(user.username.clone(), user.password.clone())?;
    /* let user = Users{
         id: 0,
         username: "xxx".to_string(),
         password: "xxx".to_string(),
         first_name: "xxx".to_string(),
         last_name: "xxx".to_string(),
         phone_number: 0
     };*/
    let ddd = Json::new(200, &*user.first_name, user.created_at);
   // let ddd = Json::new(200,xxx.as_str(),"sdfsdf");
    Ok(HttpResponse::Ok().json(ddd))
}

#[post("/post_signup")]
async fn signup(user: web::Json<User>) -> Result<HttpResponse, MyError> {
    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<User>) -> MyResult<impl Responder> {
    let user = Users::create(user.into_inner()).map(|u| u)?;
    Ok(web::Json(user))
}

#[post("/update_user/{id}")]
async fn update(
    id: web::Path<i32>,
    user: web::Json<UserUpdate>,
) -> Result<impl Responder, MyError> {
    let user = Users::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json((user)))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<i32>) -> Result<impl Responder, MyError> {
    let deleted_user = Users::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(login);
    comfig.service(signup);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}